use anyhow::Result;
use anyhow::anyhow;
use axum::{
    Router,
    extract::State,
    extract::ws::{Message as AxumMessage, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get_service,
};
use config::Config;
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};
use pidarr_shared::{DaemonState, InternalMessage, MessageType, Settings};
use serde_json::json;
use std::env::var;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{Duration, sleep};
use tower_http::services::ServeDir;

mod daemon;

//app state for the axum websocket server
#[derive(Clone)]
struct AppState {
    settings: Arc<Mutex<Settings>>,
    daemon_state: Arc<Mutex<DaemonState>>,
    config_path: String,
    api_configs: Arc<Mutex<daemon::ApiConfigs>>,
}

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
        println!(
            "Creating a configuration file with default values at: {}",
            config_path
        );

        // Create the file with default settings
        match create_default_config(Path::new(&config_path)) {
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
    let settings = Arc::new(Mutex::new(match config.try_deserialize::<Settings>() {
        Ok(settings) => settings,
        Err(e) => {
            eprintln!(
                "Failed to deserialize settings. Recreating the configuration file with default values at {}.\nError: {}",
                config_path, e
            );
            if let Err(e) = create_default_config(Path::new(&config_path)) {
                eprintln!(
                    "Failed to create config file at location {}. \n{}",
                    config_path, e,
                );
            }
            Settings::default()
        }
    }));

    // initialise default shared var daemon state
    let daemon_state = Arc::new(Mutex::new(DaemonState::default()));
    let api_configs = Arc::new(Mutex::new(daemon::ApiConfigs {
        radarr_config: None,
        qbit_config: None,
        tdarr_config: None,
    }));

    // host on *arr style addr
    let addr = "127.0.0.1:2323".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}/ws", addr);

    //construct the router

    let app = build_router().with_state(AppState {
        settings: settings.clone(),
        daemon_state: daemon_state.clone(),
        config_path: config_path.clone(),
        api_configs: api_configs.clone(),
    });

    // await both the webserver and the daemon
    tokio::join!(
        async {
            axum::serve(listener, app).await.unwrap();
        },
        async {
            daemon::main(settings.clone(), daemon_state.clone(), api_configs.clone()).await;
        }
    );

    Ok(())
}

fn build_router() -> Router<AppState> {
    //router serves leptos wasm (from web-gui) on the root, and accepts websocketupgrade on /ws
    Router::new()
        .fallback_service(get_service(ServeDir::new("./web-gui/dist")))
        .route("/ws", axum::routing::get(websocket_upgrade))
        .fallback_service(axum::routing::get_service(ServeDir::new("web-gui/dist")))
}

fn create_default_config(config_path: &Path) -> Result<()> {
    if config_path.exists() {
        let backup_path = format!("{}.bak", config_path.display());
        std::fs::rename(config_path, &backup_path)?;
        println!("Backup of existing config created at: {}", backup_path);
    } else if let Some(parent_dir) = config_path.parent() {
        std::fs::create_dir_all(parent_dir)?; // Ensure parent directories exist
    }

    let default_settings = Settings::default();
    let mut file = File::create(config_path)?;
    file.write_all(toml::to_string(&default_settings)?.as_bytes())?;
    Ok(())
}

async fn websocket_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let settings = state.settings;
    let daemon_state = state.daemon_state;
    let config_path = state.config_path;
    ws.on_upgrade(move |socket| handle_connection(socket, settings, daemon_state, config_path))
}

async fn handle_connection(
    stream: WebSocket,
    settings: Arc<Mutex<Settings>>,
    daemon_state: Arc<Mutex<DaemonState>>,
    config_path: String,
) {
    let (mut sender, mut receiver) = stream.split();
    // Send the current settings to the client to populate the form
    if let Err(e) = send_message(
        &mut sender,
        InternalMessage {
            message_type: MessageType::Settings,
            body: json!(&*settings),
        },
    )
    .await
    {
        eprintln!(
            "There was an error sending the settings to the client: {:?}",
            e
        );
    };

    // Loop for messages from the client
    let client_loop = async {
        loop {
            let msg = receive_message(&mut receiver).await;
            if let Ok(msg) = msg {
                println!("Received message from gui {:?}", msg);
                match msg.message_type {
                    MessageType::Settings => {
                        if let Ok(updated_settings) = serde_json::from_value::<Settings>(msg.body) {
                            if let Err(e) =
                                update_settings(&config_path, updated_settings, settings.clone())
                                    .await
                            {
                                eprintln!("There was an error updating the settings: {:?}", e);
                            }
                        } else {
                            eprintln!("Failed to deserialize settings update from client.");
                        }
                    }
                    _ => {
                        eprintln!(
                            "Received unrecognised message of type {:?}: {:?}",
                            msg.message_type, msg
                        );
                    }
                }
            } else {
                let e = msg.unwrap_err();
                eprintln!(
                    "There was an error receiving a message from the gui: {:?}",
                    e
                );
            }
        }
    };
    let server_loop = async {
        loop {
            if let Err(e) = send_message(
                &mut sender,
                InternalMessage {
                    message_type: MessageType::DaemonState,
                    body: json!(&*daemon_state),
                },
            )
            .await
            {
                eprintln!(
                    "There was an error sending the daemon state to the client: {:?}",
                    e
                );
            };
            sleep(Duration::from_secs(5)).await;
        }
    };
    tokio::join!(client_loop, server_loop);
}

async fn update_settings(
    config_path: &String,
    updated_settings: Settings,
    settings: Arc<Mutex<Settings>>,
) -> Result<()> {
    println!(
        "Received settings update from client: {:?}",
        updated_settings
    );

    //take out a lock on the shared settings var
    *settings.lock().unwrap() = updated_settings.clone();

    // Save the updated settings to the configuration file
    if let Some(parent_dir) = Path::new(&config_path).parent() {
        std::fs::create_dir_all(parent_dir)?; // Ensure parent directories exist
    }

    let mut file = File::create(config_path)?;
    file.write_all(toml::to_string(&updated_settings)?.as_bytes())?;

    println!("Updated settings saved to {}", config_path);

    Ok(())
}

async fn send_message(
    sender: &mut SplitSink<WebSocket, AxumMessage>,
    message: InternalMessage,
) -> Result<()> {
    sender
        .send(AxumMessage::Text(serde_json::to_string(&message)?.into()))
        .await?;
    println!("Sent message to gui: {:?}", message);
    Ok(())
}

async fn receive_message(receiver: &mut SplitStream<WebSocket>) -> Result<InternalMessage> {
    let msg = receiver.next().await.unwrap()?;
    //TODO: handle ping messages
    if let AxumMessage::Text(message) = msg {
        let int_message: InternalMessage = serde_json::from_str(message.as_str())?;
        Ok(int_message)
    } else {
        Err(anyhow!("Received non-text message from client: {:?}", msg))
    }
}
