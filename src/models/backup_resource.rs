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
pub struct BackupResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::BackupType>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl BackupResource {
    pub fn new() -> BackupResource {
        BackupResource {
            id: None,
            name: None,
            path: None,
            r#type: None,
            size: None,
            time: None,
        }
    }
}

