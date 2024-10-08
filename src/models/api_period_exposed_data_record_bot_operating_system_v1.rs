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
pub struct ApiPeriodExposedDataRecordBotOperatingSystemV1 {
    #[serde(rename = "antivirus", skip_serializing_if = "Option::is_none")]
    pub antivirus: Option<Vec<String>>,
    #[serde(rename = "computer_name", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "hardware_id", skip_serializing_if = "Option::is_none")]
    pub hardware_id: Option<String>,
    #[serde(rename = "installed_software", skip_serializing_if = "Option::is_none")]
    pub installed_software: Option<Vec<String>>,
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "layouts", skip_serializing_if = "Option::is_none")]
    pub layouts: Option<Vec<String>>,
    #[serde(rename = "os_architecture", skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "uac", skip_serializing_if = "Option::is_none")]
    pub uac: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ApiPeriodExposedDataRecordBotOperatingSystemV1 {
    pub fn new() -> ApiPeriodExposedDataRecordBotOperatingSystemV1 {
        ApiPeriodExposedDataRecordBotOperatingSystemV1 {
            antivirus: None,
            computer_name: None,
            domain: None,
            hardware_id: None,
            installed_software: None,
            language: None,
            layouts: None,
            os_architecture: None,
            os_version: None,
            timezone: None,
            uac: None,
            username: None,
        }
    }
}
