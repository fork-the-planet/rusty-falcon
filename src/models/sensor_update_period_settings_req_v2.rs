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
pub struct SensorUpdatePeriodSettingsReqV2 {
    /// The target build to apply to the policy
    #[serde(rename = "build", skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(rename = "scheduler")]
    pub scheduler: Box<models::PolicyPeriodSensorUpdateScheduler>,
    /// If true, early adopter builds will be visible on the sensor update policy page
    #[serde(
        rename = "show_early_adopter_builds",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_early_adopter_builds: Option<bool>,
    /// The uninstall protection state to apply to the policy
    #[serde(
        rename = "uninstall_protection",
        skip_serializing_if = "Option::is_none"
    )]
    pub uninstall_protection: Option<UninstallProtection>,
    #[serde(rename = "variants")]
    pub variants: Vec<models::SensorUpdatePeriodBuildReqV1>,
}

impl SensorUpdatePeriodSettingsReqV2 {
    pub fn new(
        scheduler: models::PolicyPeriodSensorUpdateScheduler,
        variants: Vec<models::SensorUpdatePeriodBuildReqV1>,
    ) -> SensorUpdatePeriodSettingsReqV2 {
        SensorUpdatePeriodSettingsReqV2 {
            build: None,
            scheduler: Box::new(scheduler),
            show_early_adopter_builds: None,
            uninstall_protection: None,
            variants,
        }
    }
}
/// The uninstall protection state to apply to the policy
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UninstallProtection {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "MAINTENANCE_MODE")]
    MaintenanceMode,
    #[serde(rename = "IGNORE")]
    Ignore,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for UninstallProtection {
    fn default() -> UninstallProtection {
        Self::Enabled
    }
}
