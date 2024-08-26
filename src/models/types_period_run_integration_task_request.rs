/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypesPeriodRunIntegrationTaskRequest {
    #[serde(rename = "access_token", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
    #[serde(rename = "scheduled", skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,
    #[serde(rename = "task_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
}

impl TypesPeriodRunIntegrationTaskRequest {
    pub fn new() -> TypesPeriodRunIntegrationTaskRequest {
        TypesPeriodRunIntegrationTaskRequest {
            access_token: None,
            category: None,
            data: None,
            r#override: None,
            scheduled: None,
            task_id: None,
        }
    }
}
