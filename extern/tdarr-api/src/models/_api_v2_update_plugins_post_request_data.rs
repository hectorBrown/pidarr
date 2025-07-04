/*
 * Tdarr API
 *
 * Tdarr API Docs
 *
 * The version of the OpenAPI document: 2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiV2UpdatePluginsPostRequestData {
    #[serde(rename = "force")]
    pub force: bool,
}

impl ApiV2UpdatePluginsPostRequestData {
    pub fn new(force: bool) -> ApiV2UpdatePluginsPostRequestData {
        ApiV2UpdatePluginsPostRequestData {
            force,
        }
    }
}

