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
pub struct ApiV2ClientClientTypePostRequestDataFiltersInner {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl ApiV2ClientClientTypePostRequestDataFiltersInner {
    pub fn new(id: String, value: String) -> ApiV2ClientClientTypePostRequestDataFiltersInner {
        ApiV2ClientClientTypePostRequestDataFiltersInner {
            id,
            value,
        }
    }
}

