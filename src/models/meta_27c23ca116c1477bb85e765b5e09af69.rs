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
pub struct Meta27c23ca116c1477bb85e765b5e09af69 {
    #[serde(
        rename = "pagination",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Option<Box<models::Pagination3f3e43ffd8e64379931bdcb29aebc087>>>,
    #[serde(
        rename = "query_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub query_time: Option<Option<f64>>,
    #[serde(
        rename = "trace_id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trace_id: Option<Option<String>>,
}

impl Meta27c23ca116c1477bb85e765b5e09af69 {
    pub fn new() -> Meta27c23ca116c1477bb85e765b5e09af69 {
        Meta27c23ca116c1477bb85e765b5e09af69 {
            pagination: None,
            query_time: None,
            trace_id: None,
        }
    }
}
