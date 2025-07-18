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

/// DomainPeriodDiscoverApiApplicationBrowserExtension : Uniquely identifies a browser extension.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DomainPeriodDiscoverApiApplicationBrowserExtension {
    /// The architecture of the browser extension
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The name of the browser that's running the extension
    #[serde(rename = "browser_name", skip_serializing_if = "Option::is_none")]
    pub browser_name: Option<String>,
    /// Describes if the extension is enabled on the host on at least one browser profile
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The unique ID of the browser extension.
    #[serde(rename = "id")]
    pub id: String,
    /// The installations of this browser extension for each browser profile
    #[serde(rename = "installations", skip_serializing_if = "Option::is_none")]
    pub installations:
        Option<Vec<models::DomainPeriodDiscoverApiApplicationBrowserExtensionInstallation>>,
    /// The computed serverity of all permissions requested by the browser extension
    #[serde(
        rename = "permission_severity",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_severity: Option<String>,
    /// The browser permissions the extension requires to run
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    /// The calculated browser webstore URL for this extension
    #[serde(rename = "store_url", skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
}

impl DomainPeriodDiscoverApiApplicationBrowserExtension {
    /// Uniquely identifies a browser extension.
    pub fn new(enabled: bool, id: String) -> DomainPeriodDiscoverApiApplicationBrowserExtension {
        DomainPeriodDiscoverApiApplicationBrowserExtension {
            architecture: None,
            browser_name: None,
            enabled,
            id,
            installations: None,
            permission_severity: None,
            permissions: None,
            store_url: None,
        }
    }
}
