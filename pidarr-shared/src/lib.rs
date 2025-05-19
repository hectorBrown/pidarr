use serde::{Deserialize, Serialize};
use std::default::Default;

#[macro_export]
macro_rules! settings_fields {
    ($macro:ident) => {
        $macro! {
            radarr_addr:("http://localhost:7878"):("Radarr address"),
            radarr_api_key:(""):("Radarr API key"),
            radarr_user:("admin"):("Radarr user"),
            qbit_addr:("http://localhost:8080"):("qBittorrent address")
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
settings_fields!(define_settings_struct);

macro_rules! define_settings_default {
    ( $( $field:ident : ( $default:expr ) : ( $desc:expr )),* ) => {
        impl Default for Settings {
            fn default() -> Self {
                Self {
                    $( $field: $default.to_string(), )*
                }
            }
        }
    }
}
settings_fields!(define_settings_default);

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MessageType {
    Settings,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InternalMessage {
    pub message_type: MessageType,
    pub body: serde_json::Value,
}
