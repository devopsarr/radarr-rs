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
pub struct LogFileResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "filename", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filename: Option<Option<String>>,
    #[serde(rename = "lastWriteTime", skip_serializing_if = "Option::is_none")]
    pub last_write_time: Option<String>,
    #[serde(rename = "contentsUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<Option<String>>,
    #[serde(rename = "downloadUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_url: Option<Option<String>>,
}

impl LogFileResource {
    pub fn new() -> LogFileResource {
        LogFileResource {
            id: None,
            filename: None,
            last_write_time: None,
            contents_url: None,
            download_url: None,
        }
    }
}

