/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2023-08-08T23:00:01Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodRuleV1 {
    #[serde(rename = "action_label")]
    pub action_label: String,
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
    #[serde(rename = "disposition_id")]
    pub disposition_id: i32,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "field_values")]
    pub field_values: Vec<crate::models::DomainPeriodFieldValue>,
    #[serde(rename = "instance_id")]
    pub instance_id: String,
    #[serde(rename = "instance_version")]
    pub instance_version: i32,
    #[serde(rename = "magic_cookie")]
    pub magic_cookie: i64,
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    #[serde(rename = "modified_on")]
    pub modified_on: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "pattern_id")]
    pub pattern_id: String,
    #[serde(rename = "pattern_severity")]
    pub pattern_severity: String,
    #[serde(rename = "rulegroup_id")]
    pub rulegroup_id: String,
    #[serde(rename = "ruletype_id")]
    pub ruletype_id: String,
    #[serde(rename = "ruletype_name")]
    pub ruletype_name: String,
    #[serde(rename = "version_ids")]
    pub version_ids: Vec<String>,
}

impl ApiPeriodRuleV1 {
    pub fn new(
        action_label: String,
        comment: String,
        committed_on: String,
        created_by: String,
        created_on: String,
        customer_id: String,
        deleted: bool,
        description: String,
        disposition_id: i32,
        enabled: bool,
        field_values: Vec<crate::models::DomainPeriodFieldValue>,
        instance_id: String,
        instance_version: i32,
        magic_cookie: i64,
        modified_by: String,
        modified_on: String,
        name: String,
        pattern_id: String,
        pattern_severity: String,
        rulegroup_id: String,
        ruletype_id: String,
        ruletype_name: String,
        version_ids: Vec<String>,
    ) -> ApiPeriodRuleV1 {
        ApiPeriodRuleV1 {
            action_label,
            comment,
            committed_on,
            created_by,
            created_on,
            customer_id,
            deleted,
            description,
            disposition_id,
            enabled,
            field_values,
            instance_id,
            instance_version,
            magic_cookie,
            modified_by,
            modified_on,
            name,
            pattern_id,
            pattern_severity,
            rulegroup_id,
            ruletype_id,
            ruletype_name,
            version_ids,
        }
    }
}