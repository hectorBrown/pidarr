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
pub struct ApiV2CancelWorkerItemPostRequest {
    #[serde(rename = "data")]
    pub data: Box<models::ApiV2CancelWorkerItemPostRequestData>,
}

impl ApiV2CancelWorkerItemPostRequest {
    pub fn new(data: models::ApiV2CancelWorkerItemPostRequestData) -> ApiV2CancelWorkerItemPostRequest {
        ApiV2CancelWorkerItemPostRequest {
            data: Box::new(data),
        }
    }
}

