use leptos::logging::log;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_sockets::{self, WebSocketError};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let daemon_addr = "ws://127.0.0.1:2323";

    let radarr_addr = RwSignal::new("127.0.0.1:7878".to_string());
    let qbit_addr = RwSignal::new("127.0.0.1:8888".to_string());

    log!("Creating connection with daemon");
    let mut client = wasm_sockets::EventClient::new(daemon_addr).unwrap();
    client.set_on_connection(Some(Box::new(|client: &wasm_sockets::EventClient| {
        log!("Sending message...");
        client.send_string("Hello, World!").unwrap();
    })));

    view! {
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
                //TODO: save addresses
                // send_data(&mut ws_stream, "Hello!");
            }
        >Save</button>

    }
}

// fn send_data(ws_stream: &mut Client<Box<dyn NetworkStream + Send>>, message: &str) {
//     // Sending a message to the server
//     ws_stream.send_message(&Message::text(message));
// }
