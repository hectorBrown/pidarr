use anyhow::{Result, anyhow};
use leptos::logging::{error, log};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use pidarr_shared::{
    ConnectionState, DaemonState, InternalMessage, Media, MessageType, Settings, settings_fields,
};
use serde::Serialize;
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_sockets::{self, EventClient, Message};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

// defines a struct that contains the settings controls -- i.e. the leptos RwSignals that are bound
// to the gui elements, initialises with the shared defaults
macro_rules! settings_controls {
    ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => {
        #[derive(Clone)]
        pub struct SettingsControls {
            $(pub $field: ArcRwSignal<String>,)*
        }

        impl SettingsControls {
            pub fn new() -> Self {
                Self {
                    $($field: ArcRwSignal::new($default.to_string()),)*
                }
            }
        }
    };
}
#[derive(Clone)]
pub struct DaemonStateControls {
    pub radarr_connected: ArcRwSignal<ConnectionState>,
    pub sonarr_connected: ArcRwSignal<ConnectionState>,
    pub qbit_connected: ArcRwSignal<ConnectionState>,
    pub tdarr_connected: ArcRwSignal<ConnectionState>,
    pub media: ArcRwSignal<HashMap<i32, ArcRwSignal<Media>>>,
}
impl DaemonStateControls {
    pub fn new() -> Self {
        Self {
            radarr_connected: ArcRwSignal::new(ConnectionState::Unknown),
            sonarr_connected: ArcRwSignal::new(ConnectionState::Unknown),
            qbit_connected: ArcRwSignal::new(ConnectionState::Unknown),
            tdarr_connected: ArcRwSignal::new(ConnectionState::Unknown),
            media: ArcRwSignal::new(HashMap::new()),
        }
    }
}
settings_fields!(settings_controls);

#[component]
pub fn App() -> impl IntoView {
    //TODO: this should be the web address serving
    let daemon_addr = "ws://127.0.0.1:2323/ws";

    let settings_controls = SettingsControls::new();
    let saved_settings = SettingsControls::new();
    let daemon_state_controls = DaemonStateControls::new();
    let connected = RwSignal::new(false);
    //TODO: its possible that the client should just be in a rwsignal
    let client = Arc::new(RefCell::new(None::<EventClient>));

    // Function to attempt connection to the daemon
    let connect_to_daemon = {
        let client = client.clone();
        let connected = connected.clone();
        let settings_controls = settings_controls.clone();
        let saved_settings = saved_settings.clone();
        let daemon_state_controls = daemon_state_controls.clone();
        move || {
            let new_client = connect_to_daemon_impl(
                daemon_addr,
                connected.clone(),
                settings_controls.clone(),
                saved_settings.clone(),
                daemon_state_controls.clone(),
            );
            //TODO: could this be better with a mutex
            *client.borrow_mut() = Some(new_client);
        }
    };

    //this defines the gui element that handles each settings field in HTML
    macro_rules! settings_gui_element {
        ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => { view! {
            $(<tr>
                <th>
                    <p>$desc:</p>
                </th>
                <th>
                    <p>
                        <input type="text" bind:value=RwSignal::from(settings_controls.clone().$field) />
                    </p>
                </th>
            </tr>)*
        }}
    }
    macro_rules! connection_element {
        ( $field:ident , ($desc:expr)) => {
            view! {
            <p>$desc: {
                let daemon_state_controls = daemon_state_controls.clone();
                move || {
                format!(" {}", if connected.get() {
                    match daemon_state_controls.$field.get() {
                        ConnectionState::Connected => "Connected",
                        ConnectionState::Disconnected => "Disconnected",
                        ConnectionState::Unauthorized => "Unauthorized",
                        ConnectionState::Unknown => "Unknown",
                    }
                } else {
                    "Unknown"
                })
            }}</p> }
        };
    }

    // Initial connection attempt
    connect_to_daemon();

    // define the main view
    view! {
        <h2>Status</h2>
        <p>Connected to daemon: {move || format!(" {}", match connected.get() {
            true => "Connected",
            false => "Disconnected",
        })}</p>
        // list of settings and input fields
        { connection_element!(radarr_connected, ("Connected to Radarr")) }
        { connection_element!(sonarr_connected, ("Connected to Sonarr")) }
        { connection_element!(qbit_connected, ("Connected to qBittorrent")) }
        { connection_element!(tdarr_connected, ("Connected to Tdarr")) }
        <h2>Settings</h2>
        <table>
            { settings_fields!(settings_gui_element) }
        </table>
        // save settings button -- sends the settings values to the daemon
        <button on:click={
            //grab a handle to the arc
            let settings_controls = settings_controls.clone();
            let saved_settings = saved_settings.clone();
            move|_|{
            // defines a payload which is just a pidarr_shared setting struct with the data from
            // settings_controls
            macro_rules! settings_payload {
                ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => {{
                    $(
                        let $field: $type;
                        //TODO: this should result in some kind of error if the parse fails to
                        //notify
                        $field = settings_controls.$field.get().parse::<$type>().unwrap_or($default);
                    )*
                    Settings {
                        $( $field, )*
                    }
                }}
            }
            let payload = InternalMessage { message_type: MessageType::Settings,
                body: json!(settings_fields!(settings_payload))
            };
            //attempt to send the payload to the daemon
            if connected.get() {
                if let Some(client) = client.borrow().as_ref() {
                    match send_to_daemon(&payload, client) {
                        Ok(_) => {
                            //TODO: this is only an assurance that the request didn't fail, not
                            //that it was saved successfully -- also useful to confirm with user
                            //that save occured
                            macro_rules! update_saved_settings {
                                ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => {{
                                    $(
                                        saved_settings.$field.set(settings_controls.$field.get());
                                    )*
                                }}
                            }
                            settings_fields!(update_saved_settings);
                            log!("Sent payload to daemon: {}", serde_json::to_string(&payload).unwrap())
                        },
                        Err(e) => error!("Failed to send payload to daemon: {}", e),
                    }
                }
            } else {
                error!("Can't save settings, not connected to daemon.")
            }
        }}>Save</button>
        //TODO: this can be removed in prod versions
        <button on:click=move |_| {
            log!("Retrying connection to daemon...");
            connect_to_daemon();
        }>Retry Connection</button>
        {torrent_table(daemon_state_controls.clone(), settings_controls.clone(), saved_settings.clone())}
    }
}

fn send_to_daemon(payload: &impl Serialize, client: &EventClient) -> Result<()> {
    match client.send_string(&serde_json::to_string(&payload)?) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to send message to daemon: {:?}", e)),
    }
}

fn torrent_table(
    daemon_state_controls: DaemonStateControls,
    settings_controls: SettingsControls,
    saved_settings: SettingsControls,
) -> impl IntoView {
    view! {
        <h2>Media</h2>
        <table>
        <tr>
            <td><b>Title</b></td>
            <td><b>Status</b></td>
            <td><b>Download Progress</b></td>
            <td><b>Transcode Progress</b></td>
            <td><b>Seeding Progress</b></td>
        </tr>
        <For
            each=move || daemon_state_controls.media.get()
            key=|item| item.0.clone()

            children=move |(_, media)| {
                view! {
                    <tr>
                        <td>{let media = media.clone();
                        move || media.get().title}</td>
                        <td>{let media = media.clone();
                            move || media.get().status.to_string()
                        }</td>
                        <td><progress value={let media = media.clone();
                            move || match media.get().download_progress {
                            Some(p) => p,
                            None => 0.0,
                        }} max=100></progress></td>
                        <td><progress value={let media = media.clone();
                            move || match media.get().transcode_progress {
                            Some(p) => p,
                            None => 0.0,
                        }} max=100></progress></td>
                        <td><progress value={move || match media.get().seeding_ratio {
                            Some(p) => p,
                            None => 0.0,
                        }} max={let saved_settings = saved_settings.clone();
                            move || saved_settings.target_seeding_ratio.get()
                        }>
                        </progress></td>
                    </tr>
                }
            }
        />
        </table>
    }
}

fn connect_to_daemon_impl(
    addr: &str,
    connected: RwSignal<bool>,
    settings_controls: SettingsControls,
    saved_settings: SettingsControls,
    daemon_state_controls: DaemonStateControls,
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
        if let Err(e) = handle_message(
            &client,
            &msg,
            settings_controls.clone(),
            saved_settings.clone(),
            daemon_state_controls.clone(),
        ) {
            error!("Failed to handle message: {:?}\nError: {}", msg, e);
        };
    })));
    client
}

fn handle_message(
    client: &EventClient,
    msg: &Message,
    settings_controls: SettingsControls,
    saved_settings: SettingsControls,
    daemon_state_controls: DaemonStateControls,
) -> Result<()> {
    // server should never send us a binary message
    let msg_string = match msg {
        Message::Text(s) => Ok(s),
        Message::Binary(u) => Err(anyhow!("Binary message received from server: {:?}", u)),
    }?;

    // deserialize into an internalmessage
    let message: InternalMessage = serde_json::from_str(&msg_string)?;

    log!("Received message from daemon: {:?}", message);

    // act based on message type
    match message.message_type {
        MessageType::Settings => update_settings(
            serde_json::from_value::<Settings>(message.body)?,
            settings_controls.clone(),
            saved_settings.clone(),
        )?,
        MessageType::DaemonState => update_daemon_state(
            serde_json::from_value::<DaemonState>(message.body)?,
            daemon_state_controls.clone(),
        )?,
    }

    Ok(())
}

fn update_settings(
    settings: Settings,
    settings_controls: SettingsControls,
    saved_settings: SettingsControls,
) -> Result<()> {
    // use the settings fields macro to set all settings_controls with the received payload
    macro_rules! update_settings_fields {
        ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => {
            $(
                settings_controls.$field.set(settings.$field.to_string());
                saved_settings.$field.set(settings.$field.to_string());
            )*
        }
    }
    settings_fields!(update_settings_fields);

    Ok(())
}

fn update_daemon_state(
    state: DaemonState,
    daemon_state_controls: DaemonStateControls,
) -> Result<()> {
    daemon_state_controls
        .qbit_connected
        .set(state.qbit_connected);
    daemon_state_controls
        .radarr_connected
        .set(state.radarr_connected);
    daemon_state_controls
        .sonarr_connected
        .set(state.sonarr_connected);
    daemon_state_controls
        .tdarr_connected
        .set(state.tdarr_connected);
    for (i, item) in state.media.iter() {
        if daemon_state_controls.media.get_untracked().contains_key(i) {
            daemon_state_controls
                .media
                .update(|m| m.get_mut(&i).unwrap().set(item.clone()));
        } else {
            daemon_state_controls.media.update(|m| {
                m.insert(*i, ArcRwSignal::new(item.clone()));
            });
        }
    }

    Ok(())
}
