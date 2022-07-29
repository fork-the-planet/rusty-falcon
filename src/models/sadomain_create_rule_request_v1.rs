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
pub struct SadomainCreateRuleRequestV1 {
    /// Whether to monitor for breach data. Available only for `Company Domains` and `Email addresses` rule topics. When enabled, ownership of the monitored domains or emails is required.
    #[serde(rename = "breach_monitoring_enabled")]
    pub breach_monitoring_enabled: bool,
    /// The filter to be used for searching
    #[serde(rename = "filter")]
    pub filter: String,
    /// The name of a particular rule
    #[serde(rename = "name")]
    pub name: String,
    /// The permissions for a particular rule which specifies the rule's access by other users. Possible values: [private public]
    #[serde(rename = "permissions")]
    pub permissions: String,
    /// The priority for a particular rule. Possible values: [low medium high]
    #[serde(rename = "priority")]
    pub priority: String,
    /// The topic of a given rule. Possible values: [SA_BRAND_PRODUCT SA_THIRD_PARTY SA_IP SA_CVE SA_DOMAIN SA_AUTHOR SA_CUSTOM SA_VIP SA_BIN SA_EMAIL SA_ALIAS]
    #[serde(rename = "topic")]
    pub topic: String,
}

impl SadomainCreateRuleRequestV1 {
    pub fn new(breach_monitoring_enabled: bool, filter: String, name: String, permissions: String, priority: String, topic: String) -> SadomainCreateRuleRequestV1 {
        SadomainCreateRuleRequestV1 {
            breach_monitoring_enabled,
            filter,
            name,
            permissions,
            priority,
            topic,
        }
    }
}
