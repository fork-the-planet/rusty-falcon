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
pub struct FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1 {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "dns_resolution_targets_polling_interval")]
    pub dns_resolution_targets_polling_interval: i32,
    #[serde(rename = "https_reachable_hosts_polling_interval")]
    pub https_reachable_hosts_polling_interval: i32,
    #[serde(rename = "icmp_request_targets_polling_interval")]
    pub icmp_request_targets_polling_interval: i32,
    #[serde(rename = "location_precedence")]
    pub location_precedence: Vec<String>,
}

impl FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1 {
    pub fn new(
        cid: String,
        dns_resolution_targets_polling_interval: i32,
        https_reachable_hosts_polling_interval: i32,
        icmp_request_targets_polling_interval: i32,
        location_precedence: Vec<String>,
    ) -> FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1 {
        FwmgrPeriodApiPeriodNetworkLocationModifyMetadataRequestV1 {
            cid,
            dns_resolution_targets_polling_interval,
            https_reachable_hosts_polling_interval,
            icmp_request_targets_polling_interval,
            location_precedence,
        }
    }
}
