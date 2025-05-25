use pidarr_shared::{ConnectionState, DaemonState, Settings};
use qbittorrent_rust::{api_fns as qbit, core::api::QbitApi};
use radarr_api::apis as radarr;
use std::sync::{Arc, Mutex};
use tdarr_api::apis as tdarr;

#[derive(Clone)]
pub struct ApiConfigs {
    pub radarr_config: Option<radarr::configuration::Configuration>,
    pub qbit_config: Option<QbitApi>,
    pub tdarr_config: Option<tdarr::configuration::Configuration>,
}

pub async fn main(
    settings: Arc<Mutex<Settings>>,
    state: Arc<Mutex<DaemonState>>,
    api_configs: Arc<Mutex<ApiConfigs>>,
) {
    let radarr_addr = settings.lock().unwrap().radarr_addr.clone();
    let radarr_api_key = settings.lock().unwrap().radarr_api_key.clone();
    let qbit_addr = settings.lock().unwrap().qbit_addr.clone();
    let tdarr_addr = settings.lock().unwrap().tdarr_addr.clone();
    api_configs.lock().unwrap().radarr_config =
        connect_radarr(radarr_addr, radarr_api_key, state.clone()).await;
    api_configs.lock().unwrap().qbit_config = connect_qbit(qbit_addr, state.clone()).await;
    api_configs.lock().unwrap().tdarr_config = connect_tdarr(tdarr_addr, state.clone()).await;
}

}

pub async fn connect_radarr(
    radarr_addr: String,
    radarr_api_key: String,
    state: Arc<Mutex<DaemonState>>,
) -> Option<radarr::configuration::Configuration> {
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
    let res = match connection_state {
        ConnectionState::Connected => Some(radarr_config),
        _ => None,
    };
    state.lock().unwrap().radarr_connected = connection_state;
    res
}

pub async fn connect_tdarr(
    tdarr_addr: String,
    state: Arc<Mutex<DaemonState>>,
) -> Option<tdarr::configuration::Configuration> {
    let mut tdarr_config = tdarr::configuration::Configuration::new();
    tdarr_config.base_path = tdarr_addr;

    let connection_state = match tdarr::default_api::api_v2_status_get(&tdarr_config).await {
        Ok(_) => {
            println!("Connected to Tdarr successfully");
            ConnectionState::Connected
        }
        Err(e) => {
            eprintln!("There was an error connecting to Tdarr: {:?}", e);
            ConnectionState::Disconnected
        }
    };
    let res = match connection_state {
        ConnectionState::Connected => Some(tdarr_config),
        _ => None,
    };
    state.lock().unwrap().tdarr_connected = connection_state;
    res
}

pub async fn connect_qbit(qbit_addr: String, state: Arc<Mutex<DaemonState>>) -> Option<QbitApi> {
    let qbit_api = QbitApi::new(
        qbit_addr,
        qbittorrent_rust::core::creds::Credentials::new("", ""),
    )
    .await;
    let mut res = None;
    let connection_state = match qbit_api {
        Ok(q) => {
            println!("Connected to qBittorrent successfully");
            res = Some(q);
            ConnectionState::Connected
        }
        Err(e) => match e.err_type {
            qbittorrent_rust::error_handling::error_type::ErrorType::WrongCreds => {
                eprintln!(
                    "Connection to qBittorrent successful but unauthorized, please whitelist Pidarr in the qBittorrent client: {:?}",
                    e
                );
                ConnectionState::Unauthorized
            }
            _ => {
                eprintln!("There was an error connecting to qBittorrent: {:?}", e);
                ConnectionState::Disconnected
            }
        },
    };
    state.lock().unwrap().qbit_connected = connection_state.clone();
    res
}
