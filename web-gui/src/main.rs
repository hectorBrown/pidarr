use leptos::logging::{error, log};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use pidarr_shared::Settings;
use serde::Serialize;
use wasm_sockets::{self, EventClient, Message};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let daemon_addr = "ws://127.0.0.1:2323";

    let radarr_addr = RwSignal::new("127.0.0.1:7878".to_string());
    let qbit_addr = RwSignal::new("127.0.0.1:8888".to_string());

    let connected = RwSignal::new(false);

    let client = connect_to_daemon(daemon_addr, connected);

    view! {
        <p>Connected to daemon: {move || format!(" {}", connected.get().to_string())}</p>
        <table>
            <tr>
                <th>
                    <p>Radarr address:</p>
                </th>
                <th>
                    <p>
                        <input type="text" bind:value=radarr_addr />
                    </p>
                </th>
            </tr>
            <tr>
                <th>
                    <p>qBittorrent address:</p>
                </th>
                <th>
                    <p>
                        <input type="text" bind:value=qbit_addr />
                    </p>
                </th>
            </tr>
        </table>
        <button on:click=move |_| {
            let payload = Settings {
                radarr_addr: radarr_addr.get().to_string(),
                qbit_addr: qbit_addr.get().to_string(),
            };
            if connected.get() {
                match send_to_daemon(&payload, &client) {
                    Ok(_) => log!("Sent payload to daemon: {}", serde_json::to_string(&payload).unwrap()),
                    Err(e) => error!("Failed to send payload to daemon: {}", e),
                }
            } else {
                error!("Can't save settings, not connected to daemon.")
            }
        }>Save</button>
    }
}

fn send_to_daemon(payload: &impl Serialize, client: &EventClient) -> Result<()> {
    match client.send_string(&serde_json::to_string(&payload)?) {
        Ok(_) => return Ok(()),
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to send payload to daemon.",
            )
            .into());
        }
    };
}

fn connect_to_daemon(addr: &str, connected: RwSignal<bool>) -> EventClient {
// fn send_data(ws_stream: &mut Client<Box<dyn NetworkStream + Send>>, message: &str) {
//     // Sending a message to the server
//     ws_stream.send_message(&Message::text(message));
// }
