/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: v5.18.4.9674
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_manual_import`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateManualImportError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_manual_import`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListManualImportError {
    UnknownValue(serde_json::Value),
}


pub async fn create_manual_import(configuration: &configuration::Configuration, manual_import_reprocess_resource: Option<Vec<models::ManualImportReprocessResource>>) -> Result<(), Error<CreateManualImportError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_manual_import_reprocess_resource = manual_import_reprocess_resource;

    let uri_str = format!("{}/api/v3/manualimport", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };
    req_builder = req_builder.json(&p_manual_import_reprocess_resource);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateManualImportError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn list_manual_import(configuration: &configuration::Configuration, folder: Option<&str>, download_id: Option<&str>, movie_id: Option<i32>, filter_existing_files: Option<bool>) -> Result<Vec<models::ManualImportResource>, Error<ListManualImportError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_folder = folder;
    let p_download_id = download_id;
    let p_movie_id = movie_id;
    let p_filter_existing_files = filter_existing_files;

    let uri_str = format!("{}/api/v3/manualimport", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_folder {
        req_builder = req_builder.query(&[("folder", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_download_id {
        req_builder = req_builder.query(&[("downloadId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_movie_id {
        req_builder = req_builder.query(&[("movieId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter_existing_files {
        req_builder = req_builder.query(&[("filterExistingFiles", &param_value.to_string())]);
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.query(&[("apikey", value)]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-Api-Key", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::ManualImportResource&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::ManualImportResource&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ListManualImportError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

