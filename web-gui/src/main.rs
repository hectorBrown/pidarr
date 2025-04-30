use leptos::logging::{error, log};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use pidarr_shared::Settings;
use serde::Serialize;
use wasm_sockets::{self, EventClient, Message};
use std::cell::RefCell;
use std::rc::Rc;

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
    let client = Rc::new(RefCell::new(None::<EventClient>));

    // Function to attempt connection
    let connect_to_daemon = {
        let client = client.clone();
        let connected = connected.clone();
        move || {
            let new_client = connect_to_daemon_impl(daemon_addr, connected.clone());
            *client.borrow_mut() = Some(new_client);
        }
    };

    // Initial connection attempt
    connect_to_daemon();

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
                if let Some(client) = client.borrow().as_ref() {
                    match send_to_daemon(&payload, client) {
                        Ok(_) => log!("Sent payload to daemon: {}", serde_json::to_string(&payload).unwrap()),
                        Err(e) => error!("Failed to send payload to daemon: {}", e),
                    }
                }
            } else {
                error!("Can't save settings, not connected to daemon.")
            }
        }>Save</button>
        <button on:click=move |_| {
            log!("Retrying connection to daemon...");
            connect_to_daemon();
        }>Retry Connection</button>
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

fn connect_to_daemon_impl(addr: &str, connected: RwSignal<bool>) -> EventClient {
    log!("Creating connection with daemon");
    let mut client = wasm_sockets::EventClient::new(addr).unwrap();
    client.set_on_connection(Some(Box::new(move |_| {
        log!("Connection to daemon established.");
        connected.set(true);
    })));
    client.set_on_close(Some(Box::new(move |_| {
        error!("Connection to daemon closed.");
        connected.set(false);
    })));
    client.set_on_error(Some(Box::new(move |error| {
        error!("{:#?}", error);
        connected.set(false);
    })));
    client.set_on_message(Some(Box::new(|client: &EventClient, msg: Message| {
        log!("Received message from daemon: {:#?}", msg);
        handle_message(&client, msg);
    })));
    client
}

fn handle_message(client: &EventClient, msg: Message) {
    log!("New message!")
}
