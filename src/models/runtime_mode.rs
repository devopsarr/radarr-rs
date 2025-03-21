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
pub enum RuntimeMode {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "tray")]
    Tray,

}

impl std::fmt::Display for RuntimeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Console => write!(f, "console"),
            Self::Service => write!(f, "service"),
            Self::Tray => write!(f, "tray"),
        }
    }
}

impl Default for RuntimeMode {
    fn default() -> RuntimeMode {
        Self::Console
    }
}

