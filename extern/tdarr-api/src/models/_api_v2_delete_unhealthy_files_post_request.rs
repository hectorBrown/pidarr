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
pub struct ApiV2DeleteUnhealthyFilesPostRequest {
    #[serde(rename = "data")]
    pub data: Box<models::ApiV2DeleteUnhealthyFilesPostRequestData>,
}

impl ApiV2DeleteUnhealthyFilesPostRequest {
    pub fn new(data: models::ApiV2DeleteUnhealthyFilesPostRequestData) -> ApiV2DeleteUnhealthyFilesPostRequest {
        ApiV2DeleteUnhealthyFilesPostRequest {
            data: Box::new(data),
        }
    }
}

