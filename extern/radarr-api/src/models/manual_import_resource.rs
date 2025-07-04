/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualImportResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "relativePath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<Option<String>>,
    #[serde(rename = "folderName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "movie", skip_serializing_if = "Option::is_none")]
    pub movie: Option<Box<models::MovieResource>>,
    #[serde(rename = "movieFileId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_file_id: Option<Option<i32>>,
    #[serde(rename = "releaseGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<Option<String>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "qualityWeight", skip_serializing_if = "Option::is_none")]
    pub quality_weight: Option<i32>,
    #[serde(rename = "downloadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<Option<String>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "indexerFlags", skip_serializing_if = "Option::is_none")]
    pub indexer_flags: Option<i32>,
    #[serde(rename = "rejections", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rejections: Option<Option<Vec<models::ImportRejectionResource>>>,
}

impl ManualImportResource {
    pub fn new() -> ManualImportResource {
        ManualImportResource {
            id: None,
            path: None,
            relative_path: None,
            folder_name: None,
            name: None,
            size: None,
            movie: None,
            movie_file_id: None,
            release_group: None,
            quality: None,
            languages: None,
            quality_weight: None,
            download_id: None,
            custom_formats: None,
            custom_format_score: None,
            indexer_flags: None,
            rejections: None,
        }
    }
}

