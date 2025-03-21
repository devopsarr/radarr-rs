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
pub struct CollectionResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "sortTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<Option<String>>,
    #[serde(rename = "tmdbId", skip_serializing_if = "Option::is_none")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "qualityProfileId", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    #[serde(rename = "searchOnAdd", skip_serializing_if = "Option::is_none")]
    pub search_on_add: Option<bool>,
    #[serde(rename = "minimumAvailability", skip_serializing_if = "Option::is_none")]
    pub minimum_availability: Option<models::MovieStatusType>,
    #[serde(rename = "movies", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movies: Option<Option<Vec<models::CollectionMovieResource>>>,
    #[serde(rename = "missingMovies", skip_serializing_if = "Option::is_none")]
    pub missing_movies: Option<i32>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
}

impl CollectionResource {
    pub fn new() -> CollectionResource {
        CollectionResource {
            id: None,
            title: None,
            sort_title: None,
            tmdb_id: None,
            images: None,
            overview: None,
            monitored: None,
            root_folder_path: None,
            quality_profile_id: None,
            search_on_add: None,
            minimum_availability: None,
            movies: None,
            missing_movies: None,
            tags: None,
        }
    }
}

