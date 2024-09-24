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
pub struct ModelsPeriodIssuesSummaryContainerInfo {
    #[serde(rename = "first_seen")]
    pub first_seen: i64,
    #[serde(rename = "last_seen")]
    pub last_seen: i64,
    #[serde(rename = "running_containers")]
    pub running_containers: i64,
}

impl ModelsPeriodIssuesSummaryContainerInfo {
    pub fn new(
        first_seen: i64,
        last_seen: i64,
        running_containers: i64,
    ) -> ModelsPeriodIssuesSummaryContainerInfo {
        ModelsPeriodIssuesSummaryContainerInfo {
            first_seen,
            last_seen,
            running_containers,
        }
    }
}