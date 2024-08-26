/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiPeriodCertBasedExclusionV1 {
    #[serde(rename = "applied_globally", skip_serializing_if = "Option::is_none")]
    pub applied_globally: Option<bool>,
    #[serde(rename = "certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Box<models::ApiPeriodCertificateV1>>,
    #[serde(rename = "children_cids", skip_serializing_if = "Option::is_none")]
    pub children_cids: Option<Vec<String>>,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "created_on", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "host_groups", skip_serializing_if = "Option::is_none")]
    pub host_groups: Option<Vec<String>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "modified_on", skip_serializing_if = "Option::is_none")]
    pub modified_on: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ApiPeriodCertBasedExclusionV1 {
    pub fn new(cid: String, id: String) -> ApiPeriodCertBasedExclusionV1 {
        ApiPeriodCertBasedExclusionV1 {
            applied_globally: None,
            certificate: None,
            children_cids: None,
            cid,
            comment: None,
            created_by: None,
            created_on: None,
            description: None,
            host_groups: None,
            id,
            modified_by: None,
            modified_on: None,
            name: None,
            status: None,
        }
    }
}
