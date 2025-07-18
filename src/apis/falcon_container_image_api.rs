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

/// struct for typed errors of method [`create_registry_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRegistryEntitiesError {
    Status400(models::DomainPeriodExternalRegistryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status409(models::DomainPeriodExternalRegistryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodExternalRegistryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_registry_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRegistryEntitiesError {
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaPeriodReplyMetaOnly),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_export_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadExportFileError {
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`head_image_scan_inventory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HeadImageScanInventoryError {
    Status400(),
    Status403(models::CorePeriodEntitiesResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`launch_export_job`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LaunchExportJobError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaspecPeriodResponseFields),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_image_scan_inventory`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostImageScanInventoryError {
    Status400(),
    Status403(models::CorePeriodEntitiesResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`query_export_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryExportJobsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_export_jobs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadExportJobsError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_registry_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadRegistryEntitiesError {
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::DomainPeriodExternalQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodExternalQueryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`read_registry_entities_by_uuid`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadRegistryEntitiesByUuidError {
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::DomainPeriodExternalRegistryListResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodExternalRegistryListResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_registry_entities`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateRegistryEntitiesError {
    Status400(models::DomainPeriodExternalRegistryResponse),
    Status403(models::MsaPeriodReplyMetaOnly),
    Status404(models::DomainPeriodExternalRegistryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodExternalRegistryResponse),
    UnknownValue(serde_json::Value),
}

pub async fn create_registry_entities(
    configuration: &configuration::Configuration,
    body: models::RegistryassessmentPeriodExternalRegistryPayload,
) -> Result<models::DomainPeriodExternalRegistryResponse, Error<CreateRegistryEntitiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/container-security/entities/registries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalRegistryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalRegistryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateRegistryEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn delete_registry_entities(
    configuration: &configuration::Configuration,
    ids: &str,
) -> Result<models::DomainPeriodExternalRegistryListResponse, Error<DeleteRegistryEntitiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/container-security/entities/registries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

    req_builder = req_builder.query(&[("ids", &p_ids.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalRegistryListResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalRegistryListResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteRegistryEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn download_export_file(
    configuration: &configuration::Configuration,
    id: &str,
) -> Result<Vec<i32>, Error<DownloadExportFileError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!(
        "{}/container-security/entities/exports/files/v1",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadExportFileError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn head_image_scan_inventory(
    configuration: &configuration::Configuration,
) -> Result<(), Error<HeadImageScanInventoryError>> {
    let uri_str = format!(
        "{}/image-assessment/entities/image-inventory/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::HEAD, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<HeadImageScanInventoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn launch_export_job(
    configuration: &configuration::Configuration,
    body: models::ExportsPeriodLaunchExportRequest,
) -> Result<models::ExportsPeriodLaunchExportResponse, Error<LaunchExportJobError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/container-security/entities/exports/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ExportsPeriodLaunchExportResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ExportsPeriodLaunchExportResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LaunchExportJobError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn post_image_scan_inventory(
    configuration: &configuration::Configuration,
    body: models::ModelsPeriodInventoryScanRequestType,
) -> Result<(), Error<PostImageScanInventoryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/image-assessment/entities/image-inventory/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<PostImageScanInventoryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn query_export_jobs(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<QueryExportJobsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_filter = filter;

    let uri_str = format!(
        "{}/container-security/queries/exports/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
        let entity: Option<QueryExportJobsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn read_export_jobs(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::ExportsPeriodExportsResponse, Error<ReadExportJobsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/container-security/entities/exports/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = match "csv" {
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ExportsPeriodExportsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ExportsPeriodExportsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadExportJobsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn read_registry_entities(
    configuration: &configuration::Configuration,
    limit: Option<i32>,
    offset: Option<i32>,
    sort: Option<&str>,
) -> Result<models::DomainPeriodExternalQueryResponse, Error<ReadRegistryEntitiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_limit = limit;
    let p_offset = offset;
    let p_sort = sort;

    let uri_str = format!(
        "{}/container-security/queries/registries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_offset {
        req_builder = req_builder.query(&[("offset", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalQueryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalQueryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadRegistryEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn read_registry_entities_by_uuid(
    configuration: &configuration::Configuration,
    ids: &str,
) -> Result<models::DomainPeriodExternalRegistryListResponse, Error<ReadRegistryEntitiesByUuidError>>
{
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/container-security/entities/registries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("ids", &p_ids.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalRegistryListResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalRegistryListResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ReadRegistryEntitiesByUuidError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn update_registry_entities(
    configuration: &configuration::Configuration,
    id: &str,
    body: models::RegistryassessmentPeriodExternalRegistryPatchPayload,
) -> Result<models::DomainPeriodExternalRegistryResponse, Error<UpdateRegistryEntitiesError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_body = body;

    let uri_str = format!(
        "{}/container-security/entities/registries/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_body);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DomainPeriodExternalRegistryResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DomainPeriodExternalRegistryResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateRegistryEntitiesError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
