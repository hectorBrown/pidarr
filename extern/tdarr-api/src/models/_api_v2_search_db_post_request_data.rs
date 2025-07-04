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
pub struct ApiV2SearchDbPostRequestData {
    #[serde(rename = "string")]
    pub string: String,
    #[serde(rename = "greaterThanGB")]
    pub greater_than_gb: f64,
    #[serde(rename = "lessThanGB")]
    pub less_than_gb: f64,
}

impl ApiV2SearchDbPostRequestData {
    pub fn new(string: String, greater_than_gb: f64, less_than_gb: f64) -> ApiV2SearchDbPostRequestData {
        ApiV2SearchDbPostRequestData {
            string,
            greater_than_gb,
            less_than_gb,
        }
    }
}

