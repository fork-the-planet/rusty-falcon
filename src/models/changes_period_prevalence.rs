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
pub struct ChangesPeriodPrevalence {
    #[serde(rename = "computed_timestamp")]
    pub computed_timestamp: String,
    /// Possible values: PENDING, UNIQUE, LOW, COMMON.
    #[serde(rename = "current")]
    pub current: String,
    #[serde(rename = "key")]
    pub key: String,
    /// Possible values: PENDING, UNIQUE, LOW, COMMON.
    #[serde(rename = "reported")]
    pub reported: String,
}

impl ChangesPeriodPrevalence {
    pub fn new(
        computed_timestamp: String,
        current: String,
        key: String,
        reported: String,
    ) -> ChangesPeriodPrevalence {
        ChangesPeriodPrevalence {
            computed_timestamp,
            current,
            key,
            reported,
        }
    }
}
