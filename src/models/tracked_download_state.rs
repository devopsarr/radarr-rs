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
pub enum TrackedDownloadState {
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "importBlocked")]
    ImportBlocked,
    #[serde(rename = "importPending")]
    ImportPending,
    #[serde(rename = "importing")]
    Importing,
    #[serde(rename = "imported")]
    Imported,
    #[serde(rename = "failedPending")]
    FailedPending,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "ignored")]
    Ignored,

}

impl std::fmt::Display for TrackedDownloadState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Downloading => write!(f, "downloading"),
            Self::ImportBlocked => write!(f, "importBlocked"),
            Self::ImportPending => write!(f, "importPending"),
            Self::Importing => write!(f, "importing"),
            Self::Imported => write!(f, "imported"),
            Self::FailedPending => write!(f, "failedPending"),
            Self::Failed => write!(f, "failed"),
            Self::Ignored => write!(f, "ignored"),
        }
    }
}

impl Default for TrackedDownloadState {
    fn default() -> TrackedDownloadState {
        Self::Downloading
    }
}

