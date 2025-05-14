use anyhow::Result;
use config::Config;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use pidarr_shared::{InternalMessage, MessageType, Settings};
use serde_json::json;
use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{WebSocketStream, accept_async};

#[tokio::main]
async fn main() -> Result<()> {
    // construct the config file path
    // TODO: test this
    let config_path = format!(
        "{}{}",
        var("XDG_CONFIG_HOME").unwrap_or(format!("{}{}", var("HOME")?, "/.config")),
        "/pidarr/config.toml",
    );

    // check if the config file exists
    if !Path::new(&config_path).exists() {
        eprintln!("Configuration file not found: {}", config_path);
        eprintln!(
            "Creating a configuration file with default values at: {}",
            config_path
        );

        // Create the file with default settings
        match create_default_config(&config_path) {
            Ok(res) => res,
            Err(e) => {
                eprintln!(
                    "Failed to create default config at location {}. \n{}",
                    config_path, e,
                );
            }
        }
    }

    // establish config source
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    // try to extract from the file
    let settings = match config.try_deserialize::<Settings>() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!(
                "Failed to deserialize settings. Recreating the configuration file with default values. A backup of your original config can be found at {}.bak.\n{}",
                config_path, e
            );
            match create_default_config(&config_path) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!(
                        "Failed to create default config at location {}. \n{}",
                        config_path, e,
                    );
                }
            }
            Settings::default()
        }
    };

    // host on *arr style addr
    let addr = "127.0.0.1:2323".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(
            stream,
            settings.clone(),
            config_path.clone(),
        ));
    }
    Ok(())
}

fn create_default_config(config_path: &str) -> Result<()> {
    if let Some(parent_dir) = Path::new(config_path).parent() {
        std::fs::create_dir_all(parent_dir)?; // Ensure parent directories exist
    }

    let default_settings = Settings::default();
    let mut file = File::create(config_path)?;
    file.write_all(toml::to_string(&default_settings)?.as_bytes())?;
    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    settings: Settings,
    config_path: String,
) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");

    let (mut ws_send, mut ws_rec) = ws_stream.split();

    // Send the current settings to the client
    send_message(
        &mut ws_send,
        InternalMessage {
            message_type: MessageType::Settings,
            body: json!(settings),
        },
    )
    .await?;

    // Loop for responses indicating changes to the settings
    loop {
        let msg = receive_message(&mut ws_rec).await;
        let msg = msg?;
        println!("Received message from gui {:?}", msg);
        match msg.message_type {
            MessageType::Settings => {
                update_settings(&config_path, serde_json::from_value::<Settings>(msg.body)?).await?
            }
        }
    }
}

async fn update_settings(config_path: &String, settings: Settings) -> Result<()> {
    println!("Received settings update from client: {:?}", settings);

    //TODO: will need to retry connections to tdarr/qbit here
    Ok(())
}

async fn send_message(
    ws_send: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    message: InternalMessage,
) -> Result<()> {
    ws_send
        .send(Message::Text(serde_json::to_string(&message)?.into()))
        .await?;
    println!("Sent message to gui: {:?}", message);
    Ok(())
}

async fn receive_message(
    ws_rec: &mut SplitStream<WebSocketStream<TcpStream>>,
) -> Result<InternalMessage> {
    let msg = ws_rec.next().await.unwrap()?;
    let message: InternalMessage = serde_json::from_str(msg.to_text()?)?;
    Ok(message)
}
