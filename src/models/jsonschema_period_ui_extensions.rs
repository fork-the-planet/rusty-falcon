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
pub struct JsonschemaPeriodUiExtensions {
    /// supported file extensions for file upload, eg. '.yaml', '.json'
    #[serde(rename = "accept", skip_serializing_if = "Option::is_none")]
    pub accept: Option<Vec<String>>,
    #[serde(rename = "component", skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "customGroupName", skip_serializing_if = "Option::is_none")]
    pub custom_group_name: Option<String>,
    #[serde(rename = "durationOptions", skip_serializing_if = "Option::is_none")]
    pub duration_options: Option<Vec<models::JsonschemaPeriodDurationOption>>,
    /// supported encoding for file upload, eg. 'base64'
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(rename = "helperText", skip_serializing_if = "Option::is_none")]
    pub helper_text: Option<String>,
    /// generate a hidden card during mobiledoc generation
    #[serde(rename = "hide", skip_serializing_if = "Option::is_none")]
    pub hide: Option<bool>,
    /// local reference to look up the dynamic json schema and mobiledoc card configuration returned in the top-level of api response
    #[serde(rename = "schema_reference", skip_serializing_if = "Option::is_none")]
    pub schema_reference: Option<String>,
    /// skip generating a card during mobiledoc generation
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    /// decimal step to increment float values
    #[serde(rename = "step", skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
}

impl JsonschemaPeriodUiExtensions {
    pub fn new() -> JsonschemaPeriodUiExtensions {
        JsonschemaPeriodUiExtensions {
            accept: None,
            component: None,
            custom_group_name: None,
            duration_options: None,
            encoding: None,
            helper_text: None,
            hide: None,
            schema_reference: None,
            skip: None,
            step: None,
        }
    }
}
