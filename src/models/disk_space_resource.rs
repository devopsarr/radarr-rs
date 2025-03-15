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
pub struct DiskSpaceResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "label", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub label: Option<Option<String>>,
    #[serde(rename = "freeSpace", skip_serializing_if = "Option::is_none")]
    pub free_space: Option<i64>,
    #[serde(rename = "totalSpace", skip_serializing_if = "Option::is_none")]
    pub total_space: Option<i64>,
}

impl DiskSpaceResource {
    pub fn new() -> DiskSpaceResource {
        DiskSpaceResource {
            id: None,
            path: None,
            label: None,
            free_space: None,
            total_space: None,
        }
    }
}

