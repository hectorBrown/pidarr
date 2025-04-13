use anyhow::Result;
use config::Config;
use futures_util::{SinkExt, StreamExt};
use pidarr_shared::Settings;
use std::env::var;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[tokio::main]
async fn main() -> Result<()> {
    // establish config source
    let config = Config::builder()
        // should be ~/.config/pidarr/config.toml
        .add_source(config::File::with_name(&format!(
            "{}{}",
            var("XDG_CONFIG_HOME").unwrap_or(format!("{}{}", var("HOME")?, "/.config")),
            "/pidarr/config",
        )))
        .build()?;

    // try to extract from the file
    let settings = config.try_deserialize::<Settings>()?;

    // host on *arr style addr
    let addr = "127.0.0.1:2323".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, settings.clone()));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, settings: Settings) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");

    let (mut ws_send, mut ws_rec) = ws_stream.split();

    // set hello message with a copy of the config to populate the web gui
    ws_send
        .send(Message::Text(serde_json::to_string(&settings)?.into()))
        .await?;

    // loop for responses indicating changes to config
    while let Some(msg) = ws_rec.next().await {
        let msg = msg?;
        if msg.is_text() {
            let settings: Settings = serde_json::from_str(msg.to_text().unwrap()).unwrap();
            println!("{}\n{}", settings.radarr_addr, settings.qbit_addr);
        }
    }

    Ok(())
}
