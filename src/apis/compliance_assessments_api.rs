/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`ext_aggregate_cluster_assessments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateClusterAssessmentsError {
    Status400(models::DomainPeriodAggregateClusterAssessmentsResponse),
    Status401(models::DomainPeriodAggregateClusterAssessmentsResponse),
    Status403(models::DomainPeriodAggregateClusterAssessmentsResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateClusterAssessmentsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_containers_by_rules_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedContainersByRulesPathError {
    Status400(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status401(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status403(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_containers_count_by_severity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedContainersCountBySeverityError {
    Status400(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status401(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status403(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_images_by_rules_path`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedImagesByRulesPathError {
    Status400(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status401(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status403(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedAssetsByRulesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_images_count_by_severity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedImagesCountBySeverityError {
    Status400(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status401(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status403(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedAssetCountBySeverityResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_rules_by_clusters`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedRulesByClustersError {
    Status400(models::DomainPeriodAggregateFailedRulesByClustersResponse),
    Status401(models::DomainPeriodAggregateFailedRulesByClustersResponse),
    Status403(models::DomainPeriodAggregateFailedRulesByClustersResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedRulesByClustersResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_rules_by_images`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedRulesByImagesError {
    Status400(models::DomainPeriodAggregateFailedRulesByImagesResponse),
    Status401(models::DomainPeriodAggregateFailedRulesByImagesResponse),
    Status403(models::DomainPeriodAggregateFailedRulesByImagesResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedRulesByImagesResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_failed_rules_count_by_severity`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateFailedRulesCountBySeverityError {
    Status400(models::DomainPeriodAggregateFailedRulesCountBySeverityResponse),
    Status401(models::DomainPeriodAggregateFailedRulesCountBySeverityResponse),
    Status403(models::DomainPeriodAggregateFailedRulesCountBySeverityResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateFailedRulesCountBySeverityResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_image_assessments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateImageAssessmentsError {
    Status400(models::DomainPeriodAggregateImageAssessmentsResponse),
    Status401(models::DomainPeriodAggregateImageAssessmentsResponse),
    Status403(models::DomainPeriodAggregateImageAssessmentsResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateImageAssessmentsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_rules_assessments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateRulesAssessmentsError {
    Status400(models::DomainPeriodAggregateRulesAssessmentsResponse),
    Status401(models::DomainPeriodAggregateRulesAssessmentsResponse),
    Status403(models::DomainPeriodAggregateRulesAssessmentsResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateRulesAssessmentsResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`ext_aggregate_rules_by_status`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExtAggregateRulesByStatusError {
    Status400(models::DomainPeriodAggregateRulesByStatusResponse),
    Status401(models::DomainPeriodAggregateRulesByStatusResponse),
    Status403(models::DomainPeriodAggregateRulesByStatusResponse),
    Status429(models::MsaPeriodReplyMetaOnly),
    Status500(models::DomainPeriodAggregateRulesByStatusResponse),
    UnknownValue(serde_json::Value),
}

pub async fn ext_aggregate_cluster_assessments(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateClusterAssessmentsResponse,
    Error<ExtAggregateClusterAssessmentsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/compliance-by-clusters/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateClusterAssessmentsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_containers_by_rules_path(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedAssetsByRulesResponse,
    Error<ExtAggregateFailedContainersByRulesPathError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-containers-by-rules/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedContainersByRulesPathError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_containers_count_by_severity(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedAssetCountBySeverityResponse,
    Error<ExtAggregateFailedContainersCountBySeverityError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-containers-count-by-severity/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedContainersCountBySeverityError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_images_by_rules_path(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedAssetsByRulesResponse,
    Error<ExtAggregateFailedImagesByRulesPathError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-images-by-rules/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedImagesByRulesPathError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_images_count_by_severity(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedAssetCountBySeverityResponse,
    Error<ExtAggregateFailedImagesCountBySeverityError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-images-count-by-severity/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedImagesCountBySeverityError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_rules_by_clusters(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedRulesByClustersResponse,
    Error<ExtAggregateFailedRulesByClustersError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-rules-by-clusters/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedRulesByClustersError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_rules_by_images(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedRulesByImagesResponse,
    Error<ExtAggregateFailedRulesByImagesError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-rules-by-images/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedRulesByImagesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_failed_rules_count_by_severity(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateFailedRulesCountBySeverityResponse,
    Error<ExtAggregateFailedRulesCountBySeverityError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/failed-rules-count-by-severity/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateFailedRulesCountBySeverityError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_image_assessments(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
    after: Option<&str>,
    limit: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateImageAssessmentsResponse,
    Error<ExtAggregateImageAssessmentsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/compliance-by-images/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder =
            local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateImageAssessmentsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_rules_assessments(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<
    models::DomainPeriodAggregateRulesAssessmentsResponse,
    Error<ExtAggregateRulesAssessmentsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/compliance-by-rules/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateRulesAssessmentsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn ext_aggregate_rules_by_status(
    configuration: &configuration::Configuration,
    filter: Option<&str>,
) -> Result<models::DomainPeriodAggregateRulesByStatusResponse, Error<ExtAggregateRulesByStatusError>>
{
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/container-compliance/aggregates/rules-by-status/v2",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ExtAggregateRulesByStatusError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
