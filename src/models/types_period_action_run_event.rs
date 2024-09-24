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
pub struct TypesPeriodActionRunEvent {
    #[serde(rename = "FlatData")]
    pub flat_data: std::collections::HashMap<String, String>,
    #[serde(rename = "additional_data", skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::TypesPeriodActionRunEventData>>,
    #[serde(rename = "flat_fields", skip_serializing_if = "Option::is_none")]
    pub flat_fields: Option<Vec<String>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "object_type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(rename = "send_time", skip_serializing_if = "Option::is_none")]
    pub send_time: Option<Box<models::TypesPeriodTimestamp>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

impl TypesPeriodActionRunEvent {
    pub fn new(flat_data: std::collections::HashMap<String, String>) -> TypesPeriodActionRunEvent {
        TypesPeriodActionRunEvent {
            flat_data,
            additional_data: None,
            data: None,
            flat_fields: None,
            id: None,
            message: None,
            object: None,
            object_type: None,
            send_time: None,
            status: None,
        }
    }
}