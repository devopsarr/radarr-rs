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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovieResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "originalTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<Option<String>>,
    #[serde(rename = "originalLanguage", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<Box<models::Language>>,
    #[serde(rename = "alternateTitles", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub alternate_titles: Option<Option<Vec<models::AlternativeTitleResource>>>,
    #[serde(rename = "secondaryYear", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub secondary_year: Option<Option<i32>>,
    #[serde(rename = "secondaryYearSourceId", skip_serializing_if = "Option::is_none")]
    pub secondary_year_source_id: Option<i32>,
    #[serde(rename = "sortTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<Option<String>>,
    #[serde(rename = "sizeOnDisk", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<Option<i64>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::MovieStatusType>,
    #[serde(rename = "overview", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub overview: Option<Option<String>>,
    #[serde(rename = "inCinemas", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub in_cinemas: Option<Option<String>>,
    #[serde(rename = "physicalRelease", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub physical_release: Option<Option<String>>,
    #[serde(rename = "digitalRelease", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub digital_release: Option<Option<String>>,
    #[serde(rename = "releaseDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<Option<String>>,
    #[serde(rename = "physicalReleaseNote", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub physical_release_note: Option<Option<String>>,
    #[serde(rename = "images", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub images: Option<Option<Vec<models::MediaCover>>>,
    #[serde(rename = "website", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub website: Option<Option<String>>,
    #[serde(rename = "remotePoster", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remote_poster: Option<Option<String>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "youTubeTrailerId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub you_tube_trailer_id: Option<Option<String>>,
    #[serde(rename = "studio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub studio: Option<Option<String>>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "qualityProfileId", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<i32>,
    #[serde(rename = "hasFile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub has_file: Option<Option<bool>>,
    #[serde(rename = "movieFileId", skip_serializing_if = "Option::is_none")]
    pub movie_file_id: Option<i32>,
    #[serde(rename = "monitored", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<bool>,
    #[serde(rename = "minimumAvailability", skip_serializing_if = "Option::is_none")]
    pub minimum_availability: Option<models::MovieStatusType>,
    #[serde(rename = "isAvailable", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[serde(rename = "folderName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<Option<String>>,
    #[serde(rename = "runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<i32>,
    #[serde(rename = "cleanTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<Option<String>>,
    #[serde(rename = "imdbId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<Option<String>>,
    #[serde(rename = "tmdbId", skip_serializing_if = "Option::is_none")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "titleSlug", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_slug: Option<Option<String>>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "folder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub folder: Option<Option<String>>,
    #[serde(rename = "certification", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub certification: Option<Option<String>>,
    #[serde(rename = "genres", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Option<Vec<String>>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<String>,
    #[serde(rename = "addOptions", skip_serializing_if = "Option::is_none")]
    pub add_options: Option<Box<models::AddMovieOptions>>,
    #[serde(rename = "ratings", skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Box<models::Ratings>>,
    #[serde(rename = "movieFile", skip_serializing_if = "Option::is_none")]
    pub movie_file: Option<Box<models::MovieFileResource>>,
    #[serde(rename = "collection", skip_serializing_if = "Option::is_none")]
    pub collection: Option<Box<models::MovieCollectionResource>>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,
    #[serde(rename = "lastSearchTime", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_search_time: Option<Option<String>>,
    #[serde(rename = "statistics", skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Box<models::MovieStatisticsResource>>,
}

impl MovieResource {
    pub fn new() -> MovieResource {
        MovieResource {
            id: None,
            title: None,
            original_title: None,
            original_language: None,
            alternate_titles: None,
            secondary_year: None,
            secondary_year_source_id: None,
            sort_title: None,
            size_on_disk: None,
            status: None,
            overview: None,
            in_cinemas: None,
            physical_release: None,
            digital_release: None,
            release_date: None,
            physical_release_note: None,
            images: None,
            website: None,
            remote_poster: None,
            year: None,
            you_tube_trailer_id: None,
            studio: None,
            path: None,
            quality_profile_id: None,
            has_file: None,
            movie_file_id: None,
            monitored: None,
            minimum_availability: None,
            is_available: None,
            folder_name: None,
            runtime: None,
            clean_title: None,
            imdb_id: None,
            tmdb_id: None,
            title_slug: None,
            root_folder_path: None,
            folder: None,
            certification: None,
            genres: None,
            tags: None,
            added: None,
            add_options: None,
            ratings: None,
            movie_file: None,
            collection: None,
            popularity: None,
            last_search_time: None,
            statistics: None,
        }
    }
}

