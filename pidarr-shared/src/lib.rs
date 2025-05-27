use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use std::default::Default;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConnectionState {
    Unknown,
    Unauthorized,
    Disconnected,
    Connected,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Media {
    pub title: String,
    pub download_id: String,
    pub radarr_path: Option<String>,
    pub tdarr_path: Option<String>,
    pub download_progress: Option<f64>,
    pub seeding_ratio: Option<f64>,
    pub transcode_progress: Option<f64>,
    pub status: MediaStatus,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MediaStatus {
    Downloading,
    Transcoding,
    Seeding,
    Completed,
    Unknown,
}

impl fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaStatus::Downloading => write!(f, "Downloading"),
            MediaStatus::Transcoding => write!(f, "Transcoding"),
            MediaStatus::Seeding => write!(f, "Seeding"),
            MediaStatus::Completed => write!(f, "Completed"),
            MediaStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MessageType {
    Settings,
    DaemonState,
}

//characterises an internal message between the daemon and web gui
#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InternalMessage {
    pub message_type: MessageType,
    pub body: serde_json::Value,
}

//this is a macro to define the settings fields
#[macro_export]
macro_rules! settings_fields {
    ($macro:ident) => {
        $macro! {
            radarr_addr:("http://localhost:7878".to_string()):(String):("Radarr address"),
            radarr_api_key:("".to_string()):(String):("Radarr API key"),
            radarr_output:("/radarr".to_string()):(String):("Radarr output path"),
            qbit_addr:("http://localhost:8080".to_string()):(String):("qBittorrent address"),
            tdarr_addr:("http://localhost:8265".to_string()):(String):("Tdarr address"),
            tdarr_output:("/tdarr".to_string()):(String):("Tdarr output path"),
            jellyfin_input:("/jellyfin".to_string()):(String):("Jellyfin media path"),
            target_seeding_ratio:(2.0):(f64):("Target seeing ratio")
        }
    };
}

#[macro_export]
macro_rules! daemon_state_fields {
    ($macro:ident) => {
        $macro! {
            radarr_connected:(ConnectionState):(ConnectionState::Unknown):("Radarr connected"),
            qbit_connected:(ConnectionState):(ConnectionState::Unknown):("qBittorrent connected"),
            tdarr_connected:(ConnectionState):(ConnectionState::Unknown):("Tdarr connected"),
            media:(HashMap<i32, Media>):(HashMap::new()):("List of media")
        }
    };
}

macro_rules! define_settings_struct {
    ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr ) ),* ) => {
        #[derive(Clone, Deserialize, Debug, Serialize)]
        pub struct Settings {
            $( pub $field: $type, )*
        }
    }
}

macro_rules! define_daemon_state_struct {
    ( $( $field:ident : ( $type:ty ) : ( $default:expr ) : ( $desc:expr ) ),* ) => {
        #[derive(Clone, Deserialize, Debug, Serialize)]
        pub struct DaemonState {
            $( pub $field: $type, )*
        }
    }
}

settings_fields!(define_settings_struct);
daemon_state_fields!(define_daemon_state_struct);

macro_rules! define_settings_default {
    ( $( $field:ident : ( $default:expr ) : ( $type:ty ) : ( $desc:expr )),* ) => {
        impl Default for Settings {
            fn default() -> Self {
                Self {
                    $( $field: $default, )*
                }
            }
        }
    }
}
macro_rules! define_daemon_state_default {
    ( $( $field:ident : ( $type:ty ) : ( $default:expr ) : ( $desc:expr )),* ) => {
        impl Default for DaemonState {
            fn default() -> Self {
                Self {
                    $( $field: $default, )*
                }
            }
        }
    }
}
settings_fields!(define_settings_default);
daemon_state_fields!(define_daemon_state_default);
