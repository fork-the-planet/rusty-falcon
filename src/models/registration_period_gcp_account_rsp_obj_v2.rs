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
pub struct RegistrationPeriodGcpAccountRspObjV2 {
    #[serde(rename = "client_email", skip_serializing_if = "Option::is_none")]
    pub client_email: Option<String>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "parent_type", skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "service_account_conditions",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_conditions: Option<Vec<models::DomainPeriodCondition>>,
    #[serde(rename = "service_account_id", skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<i32>,
}

impl RegistrationPeriodGcpAccountRspObjV2 {
    pub fn new(parent_id: String) -> RegistrationPeriodGcpAccountRspObjV2 {
        RegistrationPeriodGcpAccountRspObjV2 {
            client_email: None,
            client_id: None,
            parent_id,
            parent_type: None,
            project_id: None,
            service_account_conditions: None,
            service_account_id: None,
        }
    }
}
