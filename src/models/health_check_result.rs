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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HealthCheckResult {
    #[serde(rename = "ok")]
    Ok,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,

}

impl std::fmt::Display for HealthCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::Notice => write!(f, "notice"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

impl Default for HealthCheckResult {
    fn default() -> HealthCheckResult {
        Self::Ok
    }
}

