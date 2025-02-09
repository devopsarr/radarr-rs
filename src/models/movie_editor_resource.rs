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
pub struct MovieEditorResource {
    #[serde(rename = "movieIds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub movie_ids: Option<Option<Vec<i32>>>,
    #[serde(rename = "monitored", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub monitored: Option<Option<bool>>,
    #[serde(rename = "qualityProfileId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_profile_id: Option<Option<i32>>,
    #[serde(rename = "minimumAvailability", skip_serializing_if = "Option::is_none")]
    pub minimum_availability: Option<models::MovieStatusType>,
    #[serde(rename = "rootFolderPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<Option<String>>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "applyTags", skip_serializing_if = "Option::is_none")]
    pub apply_tags: Option<models::ApplyTags>,
    #[serde(rename = "moveFiles", skip_serializing_if = "Option::is_none")]
    pub move_files: Option<bool>,
    #[serde(rename = "deleteFiles", skip_serializing_if = "Option::is_none")]
    pub delete_files: Option<bool>,
    #[serde(rename = "addImportExclusion", skip_serializing_if = "Option::is_none")]
    pub add_import_exclusion: Option<bool>,
}

impl MovieEditorResource {
    pub fn new() -> MovieEditorResource {
        MovieEditorResource {
            movie_ids: None,
            monitored: None,
            quality_profile_id: None,
            minimum_availability: None,
            root_folder_path: None,
            tags: None,
            apply_tags: None,
            move_files: None,
            delete_files: None,
            add_import_exclusion: None,
        }
    }
}

