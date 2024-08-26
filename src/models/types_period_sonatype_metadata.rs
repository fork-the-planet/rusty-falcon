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
pub struct TypesPeriodSonatypeMetadata {
    #[serde(rename = "CVEId", skip_serializing_if = "Option::is_none")]
    pub cveid: Option<String>,
    #[serde(
        rename = "applicationPublicId",
        skip_serializing_if = "Option::is_none"
    )]
    pub application_public_id: Option<String>,
    #[serde(
        rename = "componentNameVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub component_name_version: Option<String>,
    #[serde(rename = "iqServerUrl", skip_serializing_if = "Option::is_none")]
    pub iq_server_url: Option<String>,
}

impl TypesPeriodSonatypeMetadata {
    pub fn new() -> TypesPeriodSonatypeMetadata {
        TypesPeriodSonatypeMetadata {
            cveid: None,
            application_public_id: None,
            component_name_version: None,
            iq_server_url: None,
        }
    }
}
