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
pub struct ApiV2StatusGet200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "isProduction", skip_serializing_if = "Option::is_none")]
    pub is_production: Option<bool>,
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "uptime", skip_serializing_if = "Option::is_none")]
    pub uptime: Option<i32>,
}

impl ApiV2StatusGet200Response {
    pub fn new() -> ApiV2StatusGet200Response {
        ApiV2StatusGet200Response {
            status: None,
            is_production: None,
            os: None,
            version: None,
            uptime: None,
        }
    }
}

