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
pub struct ApiV2RunHelpCommandPostRequestData {
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(rename = "text")]
    pub text: String,
}

impl ApiV2RunHelpCommandPostRequestData {
    pub fn new(mode: String, text: String) -> ApiV2RunHelpCommandPostRequestData {
        ApiV2RunHelpCommandPostRequestData {
            mode,
            text,
        }
    }
}

