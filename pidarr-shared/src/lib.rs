use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ConnectionState {
    Unknown,
    Unauthorized,
    Disconnected,
    Connected,
}
//this is a macro to define the settings fields
#[macro_export]
macro_rules! settings_fields {
    ($macro:ident) => {
        $macro! {
            radarr_addr:("http://localhost:7878".to_string()):("Radarr address"),
            radarr_api_key:("".to_string()):("Radarr API key"),
            qbit_addr:("http://localhost:8080".to_string()):("qBittorrent address")
        }
    };
}

#[macro_export]
macro_rules! daemon_state_fields {
    ($macro:ident) => {
        $macro! {
            radarr_connected:(ConnectionState):(ConnectionState::Unknown):("Radarr connected"),
            qbit_connected:(ConnectionState):(ConnectionState::Unknown):("qBittorrent connected")
        }
    };
}

macro_rules! define_settings_struct {
    ( $( $field:ident : ( $default:expr ) : ( $desc:expr ) ),* ) => {
        #[derive(Clone, Deserialize, Debug, Serialize)]
        pub struct Settings {
            $( pub $field: String, )*
        }
    }
}

macro_rules! define_daemon_state_struct {
    ( $( $field:ident : ( $type:ident ) :( $default:expr ) : ( $desc:expr ) ),* ) => {
        #[derive(Clone, Deserialize, Debug, Serialize)]
        pub struct DaemonState {
            $( pub $field: $type, )*
        }
    }
}

settings_fields!(define_settings_struct);
daemon_state_fields!(define_daemon_state_struct);

macro_rules! define_settings_default {
    ( $( $field:ident : ( $default:expr ) : ( $desc:expr )),* ) => {
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
    ( $( $field:ident : ( $type:ident ) : ( $default:expr ) : ( $desc:expr )),* ) => {
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
