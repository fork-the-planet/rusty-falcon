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
pub struct DomainPeriodUpdateRuleRequestV1 {
    /// Monitor only for breach data. Must be accompanied by breach_monitoring_enabled:true.
    #[serde(rename = "breach_monitor_only")]
    pub breach_monitor_only: bool,
    /// Whether to monitor for breach data. Available only for `Company Domains` and `Email addresses` rule topics. When enabled, ownership of the monitored domains or emails is required
    #[serde(rename = "breach_monitoring_enabled")]
    pub breach_monitoring_enabled: bool,
    /// The FQL filter to be used for searching
    #[serde(rename = "filter")]
    pub filter: String,
    /// The rule ID to be updated
    #[serde(rename = "id")]
    pub id: String,
    /// Which result types to monitor for. Can be set to only monitor domains or subdomains, as well as both. Only available for the `Typosquatting` rule topic.
    #[serde(rename = "match_on_tsq_result_types")]
    pub match_on_tsq_result_types: Vec<String>,
    /// The name of a given rule
    #[serde(rename = "name")]
    pub name: String,
    /// The permissions for a given rule which specifies the rule's access by other users. Possible values: [`public`, `private`]
    #[serde(rename = "permissions")]
    pub permissions: String,
    /// The priority for a given rule. Possible values: [`low`, `medium`, `high`]
    #[serde(rename = "priority")]
    pub priority: String,
    /// Whether to monitor for substring matches. Only available for the `Typosquatting` topic.
    #[serde(rename = "substring_matching_enabled")]
    pub substring_matching_enabled: bool,
}

impl DomainPeriodUpdateRuleRequestV1 {
    pub fn new(
        breach_monitor_only: bool,
        breach_monitoring_enabled: bool,
        filter: String,
        id: String,
        match_on_tsq_result_types: Vec<String>,
        name: String,
        permissions: String,
        priority: String,
        substring_matching_enabled: bool,
    ) -> DomainPeriodUpdateRuleRequestV1 {
        DomainPeriodUpdateRuleRequestV1 {
            breach_monitor_only,
            breach_monitoring_enabled,
            filter,
            id,
            match_on_tsq_result_types,
            name,
            permissions,
            priority,
            substring_matching_enabled,
        }
    }
}
