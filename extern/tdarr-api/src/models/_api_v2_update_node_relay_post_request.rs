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
pub struct ApiV2UpdateNodeRelayPostRequest {
    #[serde(rename = "data")]
    pub data: Box<models::ApiV2UpdateNodeRelayPostRequestData>,
}

impl ApiV2UpdateNodeRelayPostRequest {
    pub fn new(data: models::ApiV2UpdateNodeRelayPostRequestData) -> ApiV2UpdateNodeRelayPostRequest {
        ApiV2UpdateNodeRelayPostRequest {
            data: Box::new(data),
        }
    }
}

