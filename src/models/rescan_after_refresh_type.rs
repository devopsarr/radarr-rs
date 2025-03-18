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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RescanAfterRefreshType {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "afterManual")]
    AfterManual,
    #[serde(rename = "never")]
    Never,

}

impl std::fmt::Display for RescanAfterRefreshType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::AfterManual => write!(f, "afterManual"),
            Self::Never => write!(f, "never"),
        }
    }
}

impl Default for RescanAfterRefreshType {
    fn default() -> RescanAfterRefreshType {
        Self::Always
    }
}

