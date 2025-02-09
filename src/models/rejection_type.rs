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
pub enum RejectionType {
    #[serde(rename = "permanent")]
    Permanent,
    #[serde(rename = "temporary")]
    Temporary,

}

impl std::fmt::Display for RejectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Permanent => write!(f, "permanent"),
            Self::Temporary => write!(f, "temporary"),
        }
    }
}

impl Default for RejectionType {
    fn default() -> RejectionType {
        Self::Permanent
    }
}

