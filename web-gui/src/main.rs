use anyhow::{Result, anyhow};
use leptos::logging::{error, log};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use pidarr_shared::{InternalMessage, MessageType, Settings};
use serde::Serialize;
use serde_json::json;
use std::cell::RefCell;
use std::sync::Arc;
use wasm_sockets::{self, EventClient, Message};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[derive(Clone)]
struct SettingsControls {
    pub radarr_addr: RwSignal<String>,
    pub qbit_addr: RwSignal<String>,
}

impl SettingsControls {
    pub fn new(radarr_addr: String, qbit_addr: String) -> Arc<Self> {
        Arc::new(Self {
            radarr_addr: RwSignal::new(radarr_addr),
            qbit_addr: RwSignal::new(qbit_addr),
        })
    }
}

#[component]
pub fn App() -> impl IntoView {
    //TODO: this should be the web address serving
    let daemon_addr = "ws://127.0.0.1:2323";

    let settings_controls =
        SettingsControls::new("127.0.0.1:7878".to_string(), "127.0.0.1:8888".to_string());

    let connected = RwSignal::new(false);
    let client = Arc::new(RefCell::new(None::<EventClient>));

    // Function to attempt connection
    let connect_to_daemon = {
        let client = client.clone();
        let connected = connected.clone();
        let settings_controls = settings_controls.clone();
        move || {
            let new_client =
                connect_to_daemon_impl(daemon_addr, connected.clone(), settings_controls.clone());
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
                        <input type="text" bind:value=settings_controls.radarr_addr />
                    </p>
                </th>
            </tr>
            <tr>
                <th>
                    <p>qBittorrent address:</p>
                </th>
                <th>
                    <p>
                        <input type="text" bind:value=settings_controls.qbit_addr />
                    </p>
                </th>
            </tr>
        </table>
        <button on:click=move|_|{
            let settings_controls = settings_controls.clone();
            let payload = InternalMessage { message_type: MessageType::Settings,
            body: json!(Settings {
                radarr_addr: settings_controls.radarr_addr.get().to_string(),
                qbit_addr: settings_controls.qbit_addr.get().to_string(),
            })};
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

fn connect_to_daemon_impl(
    addr: &str,
    connected: RwSignal<bool>,
    settings_controls: Arc<SettingsControls>,
) -> EventClient {
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
    client.set_on_message(Some(Box::new(move |client: &EventClient, msg: Message| {
        if let Err(e) = handle_message(&client, &msg, settings_controls.clone()) {
            error!("Failed to handle message: {:?}\nError: {}", msg, e);
        };
    })));
    client
}

fn handle_message(
    client: &EventClient,
    msg: &Message,
    settings_controls: Arc<SettingsControls>,
) -> Result<()> {
    let msg_string = match msg {
        Message::Text(s) => Ok(s),
        Message::Binary(u) => Err(anyhow!("Binary message received from server: {:?}", u)),
    }?;

    let message: InternalMessage = serde_json::from_str(&msg_string)?;

    log!("Received message from daemon: {:?}", message);

    match message.message_type {
        MessageType::Settings => update_settings(
            serde_json::from_value::<Settings>(message.body)?,
            settings_controls.clone(),
        )?,
    }

    Ok(())
}

fn update_settings(settings: Settings, settings_controls: Arc<SettingsControls>) -> Result<()> {
    // Update settings
    settings_controls.radarr_addr.set(settings.radarr_addr);
    settings_controls.qbit_addr.set(settings.qbit_addr);

    Ok(())
}
