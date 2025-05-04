use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Settings {
    pub radarr_addr: String,
    pub qbit_addr: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            radarr_addr: "http://localhost:7878".to_string(),
            qbit_addr: "http://localhost:8080".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MessageType {
    Settings,
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct InternalMessage<T> {
    pub message_type: MessageType,
    pub body: T,
}
