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
pub struct LogResourcePagingResource {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "sortKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_key: Option<Option<String>>,
    #[serde(rename = "sortDirection", skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<models::SortDirection>,
    #[serde(rename = "totalRecords", skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i32>,
    #[serde(rename = "records", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub records: Option<Option<Vec<models::LogResource>>>,
}

impl LogResourcePagingResource {
    pub fn new() -> LogResourcePagingResource {
        LogResourcePagingResource {
            page: None,
            page_size: None,
            sort_key: None,
            sort_direction: None,
            total_records: None,
            records: None,
        }
    }
}

