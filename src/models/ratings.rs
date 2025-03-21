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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ratings {
    #[serde(rename = "imdb", skip_serializing_if = "Option::is_none")]
    pub imdb: Option<Box<models::RatingChild>>,
    #[serde(rename = "tmdb", skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<Box<models::RatingChild>>,
    #[serde(rename = "metacritic", skip_serializing_if = "Option::is_none")]
    pub metacritic: Option<Box<models::RatingChild>>,
    #[serde(rename = "rottenTomatoes", skip_serializing_if = "Option::is_none")]
    pub rotten_tomatoes: Option<Box<models::RatingChild>>,
    #[serde(rename = "trakt", skip_serializing_if = "Option::is_none")]
    pub trakt: Option<Box<models::RatingChild>>,
}

impl Ratings {
    pub fn new() -> Ratings {
        Ratings {
            imdb: None,
            tmdb: None,
            metacritic: None,
            rotten_tomatoes: None,
            trakt: None,
        }
    }
}

