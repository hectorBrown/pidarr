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
pub struct ApiV2PublicAuthLoginPostRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl ApiV2PublicAuthLoginPostRequest {
    pub fn new(username: String, password: String) -> ApiV2PublicAuthLoginPostRequest {
        ApiV2PublicAuthLoginPostRequest {
            username,
            password,
        }
    }
}

