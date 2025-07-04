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
pub struct ApiV2UseTokenPostRequestData {
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "redirect")]
    pub redirect: String,
}

impl ApiV2UseTokenPostRequestData {
    pub fn new(token: String, redirect: String) -> ApiV2UseTokenPostRequestData {
        ApiV2UseTokenPostRequestData {
            token,
            redirect,
        }
    }
}

