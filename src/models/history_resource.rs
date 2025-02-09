/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: v5.18.4.9674
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "movieId", skip_serializing_if = "Option::is_none")]
    pub movie_id: Option<i32>,
    #[serde(rename = "sourceTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source_title: Option<Option<String>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<models::Language>>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::QualityModel>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "qualityCutoffNotMet", skip_serializing_if = "Option::is_none")]
    pub quality_cutoff_not_met: Option<bool>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "downloadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<Option<String>>,
    #[serde(rename = "eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<models::MovieHistoryEventType>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "movie", skip_serializing_if = "Option::is_none")]
    pub movie: Option<Box<models::MovieResource>>,
}

impl HistoryResource {
    pub fn new() -> HistoryResource {
        HistoryResource {
            id: None,
            movie_id: None,
            source_title: None,
            languages: None,
            quality: None,
            custom_formats: None,
            custom_format_score: None,
            quality_cutoff_not_met: None,
            date: None,
            download_id: None,
            event_type: None,
            data: None,
            movie: None,
        }
    }
}

