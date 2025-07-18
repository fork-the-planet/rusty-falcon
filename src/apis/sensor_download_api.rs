/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, ContentType, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::de::Error as _;

/// struct for typed errors of method [`download_sensor_installer_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadSensorInstallerByIdError {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_sensor_installer_by_id_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadSensorInstallerByIdV2Error {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_combined_sensor_installers_by_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCombinedSensorInstallersByQueryError {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_combined_sensor_installers_by_query_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCombinedSensorInstallersByQueryV2Error {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sensor_installers_by_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSensorInstallersByQueryError {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sensor_installers_by_query_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSensorInstallersByQueryV2Error {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sensor_installers_ccidby_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSensorInstallersCcidbyQueryError {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sensor_installers_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSensorInstallersEntitiesError {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_sensor_installers_entities_v2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetSensorInstallersEntitiesV2Error {
    Status400(models::MsaspecPeriodQueryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

pub async fn download_sensor_installer_by_id(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<serde_json::Value, Error<DownloadSensorInstallerByIdError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/sensors/entities/download-installer/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadSensorInstallerByIdError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn download_sensor_installer_by_id_v2(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<serde_json::Value, Error<DownloadSensorInstallerByIdV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/sensors/entities/download-installer/v2",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadSensorInstallerByIdV2Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_combined_sensor_installers_by_query(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::DomainPeriodSensorInstallersV1, Error<GetCombinedSensorInstallersByQueryError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/sensors/combined/installers/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCombinedSensorInstallersByQueryError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_combined_sensor_installers_by_query_v2(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::DomainPeriodSensorInstallersV2, Error<GetCombinedSensorInstallersByQueryV2Error>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/sensors/combined/installers/v2", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetCombinedSensorInstallersByQueryV2Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sensor_installers_by_query(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<GetSensorInstallersByQueryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/sensors/queries/installers/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSensorInstallersByQueryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sensor_installers_by_query_v2(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<GetSensorInstallersByQueryV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!("{}/sensors/queries/installers/v2", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSensorInstallersByQueryV2Error> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sensor_installers_ccidby_query(
    configuration: &configuration::Configuration,
) -> Result<models::MsaspecPeriodQueryResponse, Error<GetSensorInstallersCcidbyQueryError>> {
    let uri_str = format!(
        "{}/sensors/queries/installers/ccid/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaspecPeriodQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSensorInstallersCcidbyQueryError> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sensor_installers_entities(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::DomainPeriodSensorInstallersV1, Error<GetSensorInstallersEntitiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!("{}/sensors/entities/installers/v1", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = match "multi" {
        "multi" => req_builder.query(
            &p_ids
                .into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => req_builder.query(&[(
            "ids",
            &p_ids
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSensorInstallersEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn get_sensor_installers_entities_v2(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::DomainPeriodSensorInstallersV2, Error<GetSensorInstallersEntitiesV2Error>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!("{}/sensors/entities/installers/v2", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = match "multi" {
        "multi" => req_builder.query(
            &p_ids
                .into_iter()
                .map(|p| ("ids".to_owned(), p.to_string()))
                .collect::<Vec<(std::string::String, std::string::String)>>(),
        ),
        _ => req_builder.query(&[(
            "ids",
            &p_ids
                .into_iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(",")
                .to_string(),
        )]),
    };
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodSensorInstallersV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetSensorInstallersEntitiesV2Error> =
            serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
