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
pub struct DomainPeriodExposedDataRecordBotV1 {
    #[serde(rename = "bot_id")]
    pub bot_id: String,
    #[serde(rename = "domain_detects")]
    pub domain_detects: Vec<String>,
    #[serde(rename = "infection_build_id")]
    pub infection_build_id: String,
    #[serde(rename = "infection_date")]
    pub infection_date: String,
    #[serde(rename = "infection_path")]
    pub infection_path: String,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "location")]
    pub location: Box<models::DomainPeriodExposedDataRecordBotLocationV1>,
    #[serde(rename = "operating_system")]
    pub operating_system: Box<models::DomainPeriodExposedDataRecordBotOperatingSystemV1>,
}

impl DomainPeriodExposedDataRecordBotV1 {
    pub fn new(
        bot_id: String,
        domain_detects: Vec<String>,
        infection_build_id: String,
        infection_date: String,
        infection_path: String,
        location: models::DomainPeriodExposedDataRecordBotLocationV1,
        operating_system: models::DomainPeriodExposedDataRecordBotOperatingSystemV1,
    ) -> DomainPeriodExposedDataRecordBotV1 {
        DomainPeriodExposedDataRecordBotV1 {
            bot_id,
            domain_detects,
            infection_build_id,
            infection_date,
            infection_path,
            ip: None,
            location: Box::new(location),
            operating_system: Box::new(operating_system),
        }
    }
}
