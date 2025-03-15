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
pub struct MediaCover {
    #[serde(rename = "coverType", skip_serializing_if = "Option::is_none")]
    pub cover_type: Option<models::MediaCoverTypes>,
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "remoteUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<Option<String>>,
}

impl MediaCover {
    pub fn new() -> MediaCover {
        MediaCover {
            cover_type: None,
            url: None,
            remote_url: None,
        }
    }
}

