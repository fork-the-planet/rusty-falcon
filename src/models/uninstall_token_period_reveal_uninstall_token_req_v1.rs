/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

/// UninstallTokenPeriodRevealUninstallTokenReqV1 : A req to reveal an uninstall token for a specific device
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UninstallTokenPeriodRevealUninstallTokenReqV1 {
    /// An optional message to append to the recorded audit log
    #[serde(rename = "audit_message", skip_serializing_if = "Option::is_none")]
    pub audit_message: Option<String>,
    /// The id of the device to reveal the token for
    #[serde(rename = "device_id")]
    pub device_id: String,
}

impl UninstallTokenPeriodRevealUninstallTokenReqV1 {
    /// A req to reveal an uninstall token for a specific device
    pub fn new(device_id: String) -> UninstallTokenPeriodRevealUninstallTokenReqV1 {
        UninstallTokenPeriodRevealUninstallTokenReqV1 {
            audit_message: None,
            device_id,
        }
    }
}
