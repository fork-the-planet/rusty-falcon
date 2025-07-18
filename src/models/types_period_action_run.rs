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
pub struct TypesPeriodActionRun {
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<models::TypesPeriodActionRunEvent>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "latestEvent", skip_serializing_if = "Option::is_none")]
    pub latest_event: Option<Box<models::TypesPeriodActionRunEvent>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::TypesPeriodActionRunMetadata>>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    #[serde(rename = "scheduled", skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,
    #[serde(rename = "traceUuid", skip_serializing_if = "Option::is_none")]
    pub trace_uuid: Option<String>,
}

impl TypesPeriodActionRun {
    pub fn new() -> TypesPeriodActionRun {
        TypesPeriodActionRun {
            create_time: None,
            events: None,
            id: None,
            latest_event: None,
            metadata: None,
            progress: None,
            scheduled: None,
            trace_uuid: None,
        }
    }
}
