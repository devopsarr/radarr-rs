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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostConfigResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "bindAddress", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bind_address: Option<Option<String>>,
    #[serde(rename = "port", skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "sslPort", skip_serializing_if = "Option::is_none")]
    pub ssl_port: Option<i32>,
    #[serde(rename = "enableSsl", skip_serializing_if = "Option::is_none")]
    pub enable_ssl: Option<bool>,
    #[serde(rename = "launchBrowser", skip_serializing_if = "Option::is_none")]
    pub launch_browser: Option<bool>,
    #[serde(rename = "authenticationMethod", skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<models::AuthenticationType>,
    #[serde(rename = "authenticationRequired", skip_serializing_if = "Option::is_none")]
    pub authentication_required: Option<models::AuthenticationRequiredType>,
    #[serde(rename = "analyticsEnabled", skip_serializing_if = "Option::is_none")]
    pub analytics_enabled: Option<bool>,
    #[serde(rename = "username", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub username: Option<Option<String>>,
    #[serde(rename = "password", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password: Option<Option<String>>,
    #[serde(rename = "passwordConfirmation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub password_confirmation: Option<Option<String>>,
    #[serde(rename = "logLevel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<Option<String>>,
    #[serde(rename = "logSizeLimit", skip_serializing_if = "Option::is_none")]
    pub log_size_limit: Option<i32>,
    #[serde(rename = "consoleLogLevel", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub console_log_level: Option<Option<String>>,
    #[serde(rename = "branch", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub branch: Option<Option<String>>,
    #[serde(rename = "apiKey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<Option<String>>,
    #[serde(rename = "sslCertPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssl_cert_path: Option<Option<String>>,
    #[serde(rename = "sslCertPassword", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub ssl_cert_password: Option<Option<String>>,
    #[serde(rename = "urlBase", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url_base: Option<Option<String>>,
    #[serde(rename = "instanceName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<Option<String>>,
    #[serde(rename = "applicationUrl", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub application_url: Option<Option<String>>,
    #[serde(rename = "updateAutomatically", skip_serializing_if = "Option::is_none")]
    pub update_automatically: Option<bool>,
    #[serde(rename = "updateMechanism", skip_serializing_if = "Option::is_none")]
    pub update_mechanism: Option<models::UpdateMechanism>,
    #[serde(rename = "updateScriptPath", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub update_script_path: Option<Option<String>>,
    #[serde(rename = "proxyEnabled", skip_serializing_if = "Option::is_none")]
    pub proxy_enabled: Option<bool>,
    #[serde(rename = "proxyType", skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<models::ProxyType>,
    #[serde(rename = "proxyHostname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_hostname: Option<Option<String>>,
    #[serde(rename = "proxyPort", skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUsername", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_username: Option<Option<String>>,
    #[serde(rename = "proxyPassword", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<Option<String>>,
    #[serde(rename = "proxyBypassFilter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub proxy_bypass_filter: Option<Option<String>>,
    #[serde(rename = "proxyBypassLocalAddresses", skip_serializing_if = "Option::is_none")]
    pub proxy_bypass_local_addresses: Option<bool>,
    #[serde(rename = "certificateValidation", skip_serializing_if = "Option::is_none")]
    pub certificate_validation: Option<models::CertificateValidationType>,
    #[serde(rename = "backupFolder", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub backup_folder: Option<Option<String>>,
    #[serde(rename = "backupInterval", skip_serializing_if = "Option::is_none")]
    pub backup_interval: Option<i32>,
    #[serde(rename = "backupRetention", skip_serializing_if = "Option::is_none")]
    pub backup_retention: Option<i32>,
    #[serde(rename = "trustCgnatIpAddresses", skip_serializing_if = "Option::is_none")]
    pub trust_cgnat_ip_addresses: Option<bool>,
}

impl HostConfigResource {
    pub fn new() -> HostConfigResource {
        HostConfigResource {
            id: None,
            bind_address: None,
            port: None,
            ssl_port: None,
            enable_ssl: None,
            launch_browser: None,
            authentication_method: None,
            authentication_required: None,
            analytics_enabled: None,
            username: None,
            password: None,
            password_confirmation: None,
            log_level: None,
            log_size_limit: None,
            console_log_level: None,
            branch: None,
            api_key: None,
            ssl_cert_path: None,
            ssl_cert_password: None,
            url_base: None,
            instance_name: None,
            application_url: None,
            update_automatically: None,
            update_mechanism: None,
            update_script_path: None,
            proxy_enabled: None,
            proxy_type: None,
            proxy_hostname: None,
            proxy_port: None,
            proxy_username: None,
            proxy_password: None,
            proxy_bypass_filter: None,
            proxy_bypass_local_addresses: None,
            certificate_validation: None,
            backup_folder: None,
            backup_interval: None,
            backup_retention: None,
            trust_cgnat_ip_addresses: None,
        }
    }
}

