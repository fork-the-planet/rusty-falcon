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
pub struct ApiPeriodRuleGroupV1 {
    #[serde(rename = "comment")]
    pub comment: String,
    #[serde(rename = "committed_on")]
    pub committed_on: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "created_on")]
    pub created_on: String,
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "rule_ids")]
    pub rule_ids: Vec<String>,
    #[serde(rename = "rules")]
    pub rules: Vec<models::ApiPeriodRuleV1>,
    #[serde(rename = "version")]
    pub version: i64,
}

impl ApiPeriodRuleGroupV1 {
    pub fn new(
        comment: String,
        committed_on: String,
        created_by: String,
        created_on: String,
        customer_id: String,
        deleted: bool,
        description: String,
        enabled: bool,
        id: String,
        modified_by: String,
        modified_on: String,
        name: String,
        platform: String,
        rule_ids: Vec<String>,
        rules: Vec<models::ApiPeriodRuleV1>,
        version: i64,
    ) -> ApiPeriodRuleGroupV1 {
        ApiPeriodRuleGroupV1 {
            comment,
            committed_on,
            created_by,
            created_on,
            customer_id,
            deleted,
            description,
            enabled,
            id,
            modified_by,
            modified_on,
            name,
            platform,
            rule_ids,
            rules,
            version,
        }
    }
}
