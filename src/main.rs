use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Deserialize, Debug)]
struct Settings {
    radarr_addr: String,
    qbit_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:2323".to_string();
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("WebSocket connection established");

    let (mut ws_send, mut ws_rec) = ws_stream.split();

    ws_send.send(Message::Text("Hello, world!".into())).await?;

    while let Some(msg) = ws_rec.next().await {
        let msg = msg?;
        if msg.is_text() {
            let settings: Settings = serde_json::from_str(msg.to_text().unwrap()).unwrap();
            println!("{}\n{}", settings.radarr_addr, settings.qbit_addr);
        }
    }

    Ok(())
}
