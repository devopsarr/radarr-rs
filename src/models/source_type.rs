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
pub enum SourceType {
    #[serde(rename = "tmdb")]
    Tmdb,
    #[serde(rename = "mappings")]
    Mappings,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "indexer")]
    Indexer,

}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Tmdb => write!(f, "tmdb"),
            Self::Mappings => write!(f, "mappings"),
            Self::User => write!(f, "user"),
            Self::Indexer => write!(f, "indexer"),
        }
    }
}

impl Default for SourceType {
    fn default() -> SourceType {
        Self::Tmdb
    }
}

