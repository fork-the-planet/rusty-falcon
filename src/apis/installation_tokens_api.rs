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

/// struct for typed errors of method [`audit_events_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditEventsQueryError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`audit_events_read`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuditEventsReadError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`customer_settings_read`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomerSettingsReadError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tokens_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensCreateError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status409(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tokens_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensDeleteError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tokens_query`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensQueryError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tokens_read`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensReadError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tokens_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokensUpdateError {
    Status400(models::MsaspecPeriodResponseFields),
    Status403(models::MsaspecPeriodResponseFields),
    Status404(models::MsaspecPeriodQueryResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::MsaspecPeriodResponseFields),
    UnknownValue(serde_json::Value),
}

pub async fn audit_events_query(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<AuditEventsQueryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!(
        "{}/installation-tokens/queries/audit-events/v1",
        configuration.base_path
    );
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
        let entity: Option<AuditEventsQueryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn audit_events_read(
    configuration: &configuration::Configuration,
    ids: Option<Vec<String>>,
) -> Result<models::ApiPeriodAuditEventDetailsResponseV1, Error<AuditEventsReadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/installation-tokens/entities/audit-events/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_ids {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("ids".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "ids",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiPeriodAuditEventDetailsResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiPeriodAuditEventDetailsResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AuditEventsReadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn customer_settings_read(
    configuration: &configuration::Configuration,
) -> Result<models::ApiPeriodCustomerSettingsResponseV1, Error<CustomerSettingsReadError>> {
    let uri_str = format!(
        "{}/installation-tokens/entities/customer-settings/v1",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiPeriodCustomerSettingsResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiPeriodCustomerSettingsResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CustomerSettingsReadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tokens_create(
    configuration: &configuration::Configuration,
    body: models::ApiPeriodTokenCreateRequestV1,
) -> Result<models::ApiPeriodTokenDetailsResponseV1, Error<TokensCreateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_body = body;

    let uri_str = format!(
        "{}/installation-tokens/entities/tokens/v1",
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TokensCreateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tokens_delete(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
) -> Result<models::MsaspecPeriodResponseFields, Error<TokensDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/installation-tokens/entities/tokens/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::MsaspecPeriodResponseFields`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::MsaspecPeriodResponseFields`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TokensDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tokens_query(
    configuration: &configuration::Configuration,
    offset: Option<i32>,
    limit: Option<i32>,
    sort: Option<&str>,
    filter: Option<&str>,
) -> Result<models::MsaspecPeriodQueryResponse, Error<TokensQueryError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_offset = offset;
    let p_limit = limit;
    let p_sort = sort;
    let p_filter = filter;

    let uri_str = format!(
        "{}/installation-tokens/queries/tokens/v1",
        configuration.base_path
    );
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
        let entity: Option<TokensQueryError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tokens_read(
    configuration: &configuration::Configuration,
    ids: Option<Vec<String>>,
) -> Result<models::ApiPeriodTokenDetailsResponseV1, Error<TokensReadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;

    let uri_str = format!(
        "{}/installation-tokens/entities/tokens/v1",
        configuration.base_path
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_ids {
        req_builder = match "multi" {
            "multi" => req_builder.query(
                &param_value
                    .into_iter()
                    .map(|p| ("ids".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => req_builder.query(&[(
                "ids",
                &param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TokensReadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}

pub async fn tokens_update(
    configuration: &configuration::Configuration,
    ids: Vec<String>,
    body: models::ApiPeriodTokenPatchRequestV1,
) -> Result<models::ApiPeriodTokenDetailsResponseV1, Error<TokensUpdateError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ids = ids;
    let p_body = body;

    let uri_str = format!(
        "{}/installation-tokens/entities/tokens/v1",
        configuration.base_path
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::PATCH, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ApiPeriodTokenDetailsResponseV1`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TokensUpdateError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content,
            entity,
        }))
    }
}
