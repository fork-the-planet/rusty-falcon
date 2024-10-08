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
pub struct DomainPeriodApiFindingRuleV1 {
    #[serde(rename = "authority", skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(rename = "benchmark_type", skip_serializing_if = "Option::is_none")]
    pub benchmark_type: Option<String>,
    #[serde(rename = "cce", skip_serializing_if = "Option::is_none")]
    pub cce: Option<String>,
    #[serde(
        rename = "compliance_mappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub compliance_mappings: Option<Vec<models::DomainPeriodApiComplianceMappingV1>>,
    #[serde(rename = "edited")]
    pub edited: bool,
    #[serde(rename = "group_id", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "group_name", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(
        rename = "last_edited_timestamp",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_edited_timestamp: Option<String>,
    #[serde(
        rename = "mitre_attack_tactics",
        skip_serializing_if = "Option::is_none"
    )]
    pub mitre_attack_tactics: Option<Vec<models::DomainPeriodApiMitreAttackTacticV1>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "platform_name", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "policy_id", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "policy_name", skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "recommendation_id", skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl DomainPeriodApiFindingRuleV1 {
    pub fn new(edited: bool, id: String) -> DomainPeriodApiFindingRuleV1 {
        DomainPeriodApiFindingRuleV1 {
            authority: None,
            benchmark_type: None,
            cce: None,
            compliance_mappings: None,
            edited,
            group_id: None,
            group_name: None,
            id,
            last_edited_timestamp: None,
            mitre_attack_tactics: None,
            name: None,
            platform_name: None,
            policy_id: None,
            policy_name: None,
            recommendation_id: None,
            severity: None,
        }
    }
}
