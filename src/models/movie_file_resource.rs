/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: v5.19.3.9730
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieFileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "movieId", skip_serializing_if = "Option::is_none")]
    pub movie_id: Option<i32>,
    #[serde(rename = "relativePath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<Option<String>>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "dateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(rename = "sceneName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scene_name: Option<Option<String>>,
    #[serde(rename = "releaseGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<Option<String>>,
    #[serde(rename = "edition", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edition: Option<Option<String>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "indexerFlags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub indexer_flags: Option<Option<i32>>,
    #[serde(rename = "mediaInfo", skip_serializing_if = "Option::is_none")]
    pub media_info: Option<Box<models::MediaInfoResource>>,
    #[serde(rename = "originalFilePath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_file_path: Option<Option<String>>,
    #[serde(rename = "qualityCutoffNotMet", skip_serializing_if = "Option::is_none")]
    pub quality_cutoff_not_met: Option<bool>,
}

impl MovieFileResource {
    pub fn new() -> MovieFileResource {
        MovieFileResource {
            id: None,
            movie_id: None,
            relative_path: None,
            path: None,
            size: None,
            date_added: None,
            scene_name: None,
            release_group: None,
            edition: None,
            languages: None,
            quality: None,
            custom_formats: None,
            custom_format_score: None,
            indexer_flags: None,
            media_info: None,
            original_file_path: None,
            quality_cutoff_not_met: None,
        }
    }
}

