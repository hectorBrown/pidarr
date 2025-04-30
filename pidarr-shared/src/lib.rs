use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Settings {
    pub radarr_addr: String,
    pub qbit_addr: String,
}

impl Settings {
    pub fn default() -> Self {
        Settings {
            radarr_addr: "http://localhost:7878".to_string(),
            qbit_addr: "http://localhost:8080".to_string(),
        }
    }
}
