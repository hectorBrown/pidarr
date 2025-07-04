/*
 * Sonarr
 *
 * Sonarr API docs - The v3 API docs apply to both v3 and v4 versions of Sonarr. Some functionality may only be available in v4 of the Sonarr application.
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QualityProfileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "upgradeAllowed", skip_serializing_if = "Option::is_none")]
    pub upgrade_allowed: Option<bool>,
    #[serde(rename = "cutoff", skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<i32>,
    #[serde(rename = "items", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub items: Option<Option<Vec<models::QualityProfileQualityItemResource>>>,
    #[serde(rename = "minFormatScore", skip_serializing_if = "Option::is_none")]
    pub min_format_score: Option<i32>,
    #[serde(rename = "cutoffFormatScore", skip_serializing_if = "Option::is_none")]
    pub cutoff_format_score: Option<i32>,
    #[serde(rename = "minUpgradeFormatScore", skip_serializing_if = "Option::is_none")]
    pub min_upgrade_format_score: Option<i32>,
    #[serde(rename = "formatItems", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub format_items: Option<Option<Vec<models::ProfileFormatItemResource>>>,
}

impl QualityProfileResource {
    pub fn new() -> QualityProfileResource {
        QualityProfileResource {
            id: None,
            name: None,
            upgrade_allowed: None,
            cutoff: None,
            items: None,
            min_format_score: None,
            cutoff_format_score: None,
            min_upgrade_format_score: None,
            format_items: None,
        }
    }
}

