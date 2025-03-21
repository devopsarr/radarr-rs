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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MovieRuntimeFormatType {
    #[serde(rename = "hoursMinutes")]
    HoursMinutes,
    #[serde(rename = "minutes")]
    Minutes,

}

impl std::fmt::Display for MovieRuntimeFormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::HoursMinutes => write!(f, "hoursMinutes"),
            Self::Minutes => write!(f, "minutes"),
        }
    }
}

impl Default for MovieRuntimeFormatType {
    fn default() -> MovieRuntimeFormatType {
        Self::HoursMinutes
    }
}

