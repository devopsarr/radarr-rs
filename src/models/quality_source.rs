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
pub enum QualitySource {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "cam")]
    Cam,
    #[serde(rename = "telesync")]
    Telesync,
    #[serde(rename = "telecine")]
    Telecine,
    #[serde(rename = "workprint")]
    Workprint,
    #[serde(rename = "dvd")]
    Dvd,
    #[serde(rename = "tv")]
    Tv,
    #[serde(rename = "webdl")]
    Webdl,
    #[serde(rename = "webrip")]
    Webrip,
    #[serde(rename = "bluray")]
    Bluray,

}

impl std::fmt::Display for QualitySource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Cam => write!(f, "cam"),
            Self::Telesync => write!(f, "telesync"),
            Self::Telecine => write!(f, "telecine"),
            Self::Workprint => write!(f, "workprint"),
            Self::Dvd => write!(f, "dvd"),
            Self::Tv => write!(f, "tv"),
            Self::Webdl => write!(f, "webdl"),
            Self::Webrip => write!(f, "webrip"),
            Self::Bluray => write!(f, "bluray"),
        }
    }
}

impl Default for QualitySource {
    fn default() -> QualitySource {
        Self::Unknown
    }
}

