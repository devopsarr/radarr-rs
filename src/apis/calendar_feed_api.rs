/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: v5.19.3.9730
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`get_feed_v3_calendar_radarr_period_ics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeedV3CalendarRadarrPeriodIcsError {
    UnknownValue(serde_json::Value),
}


pub async fn get_feed_v3_calendar_radarr_period_ics(configuration: &configuration::Configuration, past_days: Option<i32>, future_days: Option<i32>, tags: Option<&str>, unmonitored: Option<bool>) -> Result<(), Error<GetFeedV3CalendarRadarrPeriodIcsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_past_days = past_days;
    let p_future_days = future_days;
    let p_tags = tags;
    let p_unmonitored = unmonitored;

    let uri_str = format!("{}/feed/v3/calendar/radarr.ics", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_past_days {
        req_builder = req_builder.query(&[("pastDays", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_future_days {
        req_builder = req_builder.query(&[("futureDays", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tags {
        req_builder = req_builder.query(&[("tags", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_unmonitored {
        req_builder = req_builder.query(&[("unmonitored", &param_value.to_string())]);
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

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<GetFeedV3CalendarRadarrPeriodIcsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

