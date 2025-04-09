use closure::closure;
use leptos::logging::log;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use serde::Serialize;
use wasm_sockets::{self, Message};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Serialize)]
struct Settings {
    radarr_addr: String,
    qbit_addr: String,
}

#[component]
pub fn App() -> impl IntoView {
    let daemon_addr = "ws://127.0.0.1:2323";

    let radarr_addr = RwSignal::new("127.0.0.1:7878".to_string());
    let qbit_addr = RwSignal::new("127.0.0.1:8888".to_string());

    let connected = RwSignal::new(false);

    log!("Creating connection with daemon");
    let mut client = wasm_sockets::EventClient::new(daemon_addr).unwrap();
    client.set_on_connection(Some(Box::new(closure!(move connected,|_| {
        log!("Connection to daemon established.");
        connected.set(true);
    }))));

    view! {
        <p>Connected to daemon: {move || format!(" {}", connected.get().to_string())}</p>
        <table>
            <tr>
                <th><p> Radarr address: </p></th>
                <th><p>
                    <input type="text"
                        bind:value=radarr_addr
                    />
                </p></th>
            </tr>
            <tr>
                <th><p> qBittorrent address: </p></th>
                <th><p>
                    <input type="text"
                        bind:value=qbit_addr
                    />
                </p></th>
            </tr>
        </table>
        <button on:click=move |_| {
                let payload = Settings {
        radarr_addr: radarr_addr.get().to_string(),
        qbit_addr: qbit_addr.get().to_string(),
    };
            client.send_string(&serde_json::to_string(&payload).unwrap());
            }
     >Save</button>

    }
}

// fn send_data(ws_stream: &mut Client<Box<dyn NetworkStream + Send>>, message: &str) {
//     // Sending a message to the server
//     ws_stream.send_message(&Message::text(message));
// }
