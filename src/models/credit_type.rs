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
pub enum CreditType {
    #[serde(rename = "cast")]
    Cast,
    #[serde(rename = "crew")]
    Crew,

}

impl std::fmt::Display for CreditType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cast => write!(f, "cast"),
            Self::Crew => write!(f, "crew"),
        }
    }
}

impl Default for CreditType {
    fn default() -> CreditType {
        Self::Cast
    }
}

