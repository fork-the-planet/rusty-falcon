/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainMaliciousFile {
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "filepath", skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,
    #[serde(rename = "hash", skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(rename = "host_id", skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(rename = "host_scan_id", skip_serializing_if = "Option::is_none")]
    pub host_scan_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "pattern_id", skip_serializing_if = "Option::is_none")]
    pub pattern_id: Option<i32>,
    #[serde(rename = "quarantined", skip_serializing_if = "Option::is_none")]
    pub quarantined: Option<bool>,
    #[serde(rename = "scan_id", skip_serializing_if = "Option::is_none")]
    pub scan_id: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
}

impl DomainMaliciousFile {
    pub fn new(id: String) -> DomainMaliciousFile {
        DomainMaliciousFile {
            cid: None,
            filename: None,
            filepath: None,
            hash: None,
            host_id: None,
            host_scan_id: None,
            id,
            last_updated: None,
            pattern_id: None,
            quarantined: None,
            scan_id: None,
            severity: None,
        }
    }
}
