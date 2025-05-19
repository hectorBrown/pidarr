use pidarr_shared::{ConnectionState, DaemonState, Settings};
use radarr_api::apis as radarr;
use std::sync::{Arc, Mutex};

pub async fn main(settings: Arc<Mutex<Settings>>, state: Arc<Mutex<DaemonState>>) {
    let radarr_addr = settings.lock().unwrap().radarr_addr.clone();
    let radarr_api_key = settings.lock().unwrap().radarr_api_key.clone();
    connect_radarr(radarr_addr, radarr_api_key, state.clone()).await;
}

pub async fn connect_radarr(
    radarr_addr: String,
    radarr_api_key: String,
    state: Arc<Mutex<DaemonState>>,
) {
    let mut radarr_config = radarr::configuration::Configuration::new();
    radarr_config.base_path = radarr_addr;
    radarr_config.api_key = Some(radarr::configuration::ApiKey {
        prefix: None,
        key: radarr_api_key,
    });

    let connection_state = match radarr::system_api::api_v3_system_status_get(&radarr_config).await
    {
        Ok(_) => {
            println!("Connected to Radarr successfully");
            ConnectionState::Connected
        }
        Err(radarr::Error::ResponseError(r)) => {
            if r.status == 401 {
                eprintln!("Connection to Radarr successful but unauthorized: {:?}", r);
                ConnectionState::Unauthorized
            } else {
                eprintln!("There was an error connecting to Radarr: {:?}", r);
                ConnectionState::Disconnected
            }
        }
        Err(e) => {
            eprintln!("There was an error while cnnecting to Radarr: {}", e);
            ConnectionState::Disconnected
        }
    };
    state.lock().unwrap().radarr_connected = connection_state;
}
