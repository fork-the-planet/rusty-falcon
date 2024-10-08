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
pub struct TypesPeriodPolicyRulesCreateBody {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<Box<models::PeriodActivity>>,
    #[serde(rename = "destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<Box<models::PeriodDestination>>,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "simulationMode")]
    pub simulation_mode: bool,
    #[serde(rename = "sourceEndpoint", skip_serializing_if = "Option::is_none")]
    pub source_endpoint: Option<Box<models::PeriodSourceEndpoint>>,
    #[serde(rename = "sourceUser", skip_serializing_if = "Option::is_none")]
    pub source_user: Option<Box<models::PeriodSourceUser>>,
    #[serde(rename = "trigger")]
    pub trigger: String,
}

impl TypesPeriodPolicyRulesCreateBody {
    pub fn new(
        action: String,
        enabled: bool,
        name: String,
        simulation_mode: bool,
        trigger: String,
    ) -> TypesPeriodPolicyRulesCreateBody {
        TypesPeriodPolicyRulesCreateBody {
            action,
            activity: None,
            destination: None,
            enabled,
            name,
            simulation_mode,
            source_endpoint: None,
            source_user: None,
            trigger,
        }
    }
}
