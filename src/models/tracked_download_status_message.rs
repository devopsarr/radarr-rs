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
pub struct TrackedDownloadStatusMessage {
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Option<Vec<String>>>,
}

impl TrackedDownloadStatusMessage {
    pub fn new() -> TrackedDownloadStatusMessage {
        TrackedDownloadStatusMessage {
            title: None,
            messages: None,
        }
    }
}

