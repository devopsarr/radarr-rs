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
pub struct ImportListExclusionBulkResource {
    #[serde(rename = "ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Option<Vec<i32>>>,
}

impl ImportListExclusionBulkResource {
    pub fn new() -> ImportListExclusionBulkResource {
        ImportListExclusionBulkResource {
            ids: None,
        }
    }
}

