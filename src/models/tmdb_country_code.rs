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
pub enum TmdbCountryCode {
    #[serde(rename = "au")]
    Au,
    #[serde(rename = "br")]
    Br,
    #[serde(rename = "ca")]
    Ca,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "ie")]
    Ie,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "nz")]
    Nz,
    #[serde(rename = "ro")]
    Ro,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "us")]
    Us,

}

impl std::fmt::Display for TmdbCountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Au => write!(f, "au"),
            Self::Br => write!(f, "br"),
            Self::Ca => write!(f, "ca"),
            Self::Fr => write!(f, "fr"),
            Self::De => write!(f, "de"),
            Self::Gb => write!(f, "gb"),
            Self::In => write!(f, "in"),
            Self::Ie => write!(f, "ie"),
            Self::It => write!(f, "it"),
            Self::Nz => write!(f, "nz"),
            Self::Ro => write!(f, "ro"),
            Self::Es => write!(f, "es"),
            Self::Us => write!(f, "us"),
        }
    }
}

impl Default for TmdbCountryCode {
    fn default() -> TmdbCountryCode {
        Self::Au
    }
}

