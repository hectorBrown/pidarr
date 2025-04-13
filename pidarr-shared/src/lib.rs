use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Settings {
    pub radarr_addr: String,
    pub qbit_addr: String,
}
