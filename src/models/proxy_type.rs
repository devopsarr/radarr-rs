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
pub enum ProxyType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "socks4")]
    Socks4,
    #[serde(rename = "socks5")]
    Socks5,

}

impl std::fmt::Display for ProxyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Socks4 => write!(f, "socks4"),
            Self::Socks5 => write!(f, "socks5"),
        }
    }
}

impl Default for ProxyType {
    fn default() -> ProxyType {
        Self::Http
    }
}

