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
pub struct MetadataConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "certificationCountry", skip_serializing_if = "Option::is_none")]
    pub certification_country: Option<models::TmdbCountryCode>,
}

impl MetadataConfigResource {
    pub fn new() -> MetadataConfigResource {
        MetadataConfigResource {
            id: None,
            certification_country: None,
        }
    }
}

