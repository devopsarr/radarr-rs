/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: v5.20.2.9777
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "source", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub source: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::HealthCheckResult>,
    #[serde(rename = "message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message: Option<Option<String>>,
    #[serde(rename = "wikiUrl", skip_serializing_if = "Option::is_none")]
    pub wiki_url: Option<String>,
}

impl HealthResource {
    pub fn new() -> HealthResource {
        HealthResource {
            id: None,
            source: None,
            r#type: None,
            message: None,
            wiki_url: None,
        }
    }
}

