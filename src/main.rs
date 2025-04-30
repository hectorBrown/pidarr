use anyhow::Result;
use config::Config;
use futures_util::{SinkExt, StreamExt};
use pidarr_shared::Settings;
use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<()> {
    // construct the config file path
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
        create_default_config(&config_path)?;
    }

    // establish config source
    let config = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()?;

    // try to extract from the file
    let settings = match config.try_deserialize::<Settings>() {
        Ok(settings) => settings,
        Err(_) => {
            eprintln!(
                "Failed to deserialize settings. Recreating the configuration file with default values."
            );
            create_default_config(&config_path)?;
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
    ws_send
        .send(Message::Text(serde_json::to_string(&settings)?.into()))
        .await?;

    // Loop for responses indicating changes to the settings
    while let Some(msg) = ws_rec.next().await {
        let msg = msg?;
        if msg.is_text() {
            // Deserialize the updated settings from the received message
            let updated_settings: Settings = serde_json::from_str(msg.to_text().unwrap())?;

            // Save the updated settings to the configuration file
            if let Some(parent_dir) = Path::new(&config_path).parent() {
                std::fs::create_dir_all(parent_dir)?; // Ensure parent directories exist
            }

            let mut file = File::create(&config_path)?;
            file.write_all(toml::to_string(&updated_settings)?.as_bytes())?;

            println!("Updated settings saved to {}", config_path);
        }
    }

    Ok(())
}
