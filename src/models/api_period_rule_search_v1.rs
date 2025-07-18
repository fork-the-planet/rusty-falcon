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
pub struct ApiPeriodRuleSearchV1 {
    #[serde(rename = "filter")]
    pub filter: String,
    #[serde(rename = "lookback")]
    pub lookback: String,
    #[serde(rename = "outcome")]
    pub outcome: String,
    #[serde(rename = "trigger_mode")]
    pub trigger_mode: String,
    #[serde(rename = "use_ingest_time", skip_serializing_if = "Option::is_none")]
    pub use_ingest_time: Option<bool>,
}

impl ApiPeriodRuleSearchV1 {
    pub fn new(
        filter: String,
        lookback: String,
        outcome: String,
        trigger_mode: String,
    ) -> ApiPeriodRuleSearchV1 {
        ApiPeriodRuleSearchV1 {
            filter,
            lookback,
            outcome,
            trigger_mode,
            use_ingest_time: None,
        }
    }
}
