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
pub struct QualityModel {
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<models::Quality>>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<Box<models::Revision>>,
}

impl QualityModel {
    pub fn new() -> QualityModel {
        QualityModel {
            quality: None,
            revision: None,
        }
    }
}

