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
pub enum QueueStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "downloading")]
    Downloading,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "delay")]
    Delay,
    #[serde(rename = "downloadClientUnavailable")]
    DownloadClientUnavailable,
    #[serde(rename = "fallback")]
    Fallback,

}

impl std::fmt::Display for QueueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Queued => write!(f, "queued"),
            Self::Paused => write!(f, "paused"),
            Self::Downloading => write!(f, "downloading"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Warning => write!(f, "warning"),
            Self::Delay => write!(f, "delay"),
            Self::DownloadClientUnavailable => write!(f, "downloadClientUnavailable"),
            Self::Fallback => write!(f, "fallback"),
        }
    }
}

impl Default for QueueStatus {
    fn default() -> QueueStatus {
        Self::Unknown
    }
}

