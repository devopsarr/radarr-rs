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
pub struct MediaManagementConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "autoUnmonitorPreviouslyDownloadedMovies", skip_serializing_if = "Option::is_none")]
    pub auto_unmonitor_previously_downloaded_movies: Option<bool>,
    #[serde(rename = "recycleBin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recycle_bin: Option<Option<String>>,
    #[serde(rename = "recycleBinCleanupDays", skip_serializing_if = "Option::is_none")]
    pub recycle_bin_cleanup_days: Option<i32>,
    #[serde(rename = "downloadPropersAndRepacks", skip_serializing_if = "Option::is_none")]
    pub download_propers_and_repacks: Option<models::ProperDownloadTypes>,
    #[serde(rename = "createEmptyMovieFolders", skip_serializing_if = "Option::is_none")]
    pub create_empty_movie_folders: Option<bool>,
    #[serde(rename = "deleteEmptyFolders", skip_serializing_if = "Option::is_none")]
    pub delete_empty_folders: Option<bool>,
    #[serde(rename = "fileDate", skip_serializing_if = "Option::is_none")]
    pub file_date: Option<models::FileDateType>,
    #[serde(rename = "rescanAfterRefresh", skip_serializing_if = "Option::is_none")]
    pub rescan_after_refresh: Option<models::RescanAfterRefreshType>,
    #[serde(rename = "autoRenameFolders", skip_serializing_if = "Option::is_none")]
    pub auto_rename_folders: Option<bool>,
    #[serde(rename = "pathsDefaultStatic", skip_serializing_if = "Option::is_none")]
    pub paths_default_static: Option<bool>,
    #[serde(rename = "setPermissionsLinux", skip_serializing_if = "Option::is_none")]
    pub set_permissions_linux: Option<bool>,
    #[serde(rename = "chmodFolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub chmod_folder: Option<Option<String>>,
    #[serde(rename = "chownGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub chown_group: Option<Option<String>>,
    #[serde(rename = "skipFreeSpaceCheckWhenImporting", skip_serializing_if = "Option::is_none")]
    pub skip_free_space_check_when_importing: Option<bool>,
    #[serde(rename = "minimumFreeSpaceWhenImporting", skip_serializing_if = "Option::is_none")]
    pub minimum_free_space_when_importing: Option<i32>,
    #[serde(rename = "copyUsingHardlinks", skip_serializing_if = "Option::is_none")]
    pub copy_using_hardlinks: Option<bool>,
    #[serde(rename = "useScriptImport", skip_serializing_if = "Option::is_none")]
    pub use_script_import: Option<bool>,
    #[serde(rename = "scriptImportPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub script_import_path: Option<Option<String>>,
    #[serde(rename = "importExtraFiles", skip_serializing_if = "Option::is_none")]
    pub import_extra_files: Option<bool>,
    #[serde(rename = "extraFileExtensions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extra_file_extensions: Option<Option<String>>,
    #[serde(rename = "enableMediaInfo", skip_serializing_if = "Option::is_none")]
    pub enable_media_info: Option<bool>,
}

impl MediaManagementConfigResource {
    pub fn new() -> MediaManagementConfigResource {
        MediaManagementConfigResource {
            id: None,
            auto_unmonitor_previously_downloaded_movies: None,
            recycle_bin: None,
            recycle_bin_cleanup_days: None,
            download_propers_and_repacks: None,
            create_empty_movie_folders: None,
            delete_empty_folders: None,
            file_date: None,
            rescan_after_refresh: None,
            auto_rename_folders: None,
            paths_default_static: None,
            set_permissions_linux: None,
            chmod_folder: None,
            chown_group: None,
            skip_free_space_check_when_importing: None,
            minimum_free_space_when_importing: None,
            copy_using_hardlinks: None,
            use_script_import: None,
            script_import_path: None,
            import_extra_files: None,
            extra_file_extensions: None,
            enable_media_info: None,
        }
    }
}

