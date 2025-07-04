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
pub struct ApiV2DeleteBackupPostRequest {
    #[serde(rename = "data")]
    pub data: Box<models::ApiV2DeleteBackupPostRequestData>,
}

impl ApiV2DeleteBackupPostRequest {
    pub fn new(data: models::ApiV2DeleteBackupPostRequestData) -> ApiV2DeleteBackupPostRequest {
        ApiV2DeleteBackupPostRequest {
            data: Box::new(data),
        }
    }
}

