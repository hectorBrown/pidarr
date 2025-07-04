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
pub struct ApiV2GetFilescannerStatusPostRequestData {
    #[serde(rename = "dbID")]
    pub db_id: String,
}

impl ApiV2GetFilescannerStatusPostRequestData {
    pub fn new(db_id: String) -> ApiV2GetFilescannerStatusPostRequestData {
        ApiV2GetFilescannerStatusPostRequestData {
            db_id,
        }
    }
}

