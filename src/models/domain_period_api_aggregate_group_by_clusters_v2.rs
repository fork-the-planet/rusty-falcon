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
pub struct DomainPeriodApiAggregateGroupByClustersV2 {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cloud_account_id")]
    pub cloud_account_id: String,
    #[serde(rename = "cloud_provider")]
    pub cloud_provider: String,
    #[serde(rename = "cloud_region")]
    pub cloud_region: String,
    #[serde(rename = "cluster_derived_id")]
    pub cluster_derived_id: String,
    #[serde(rename = "cluster_id")]
    pub cluster_id: String,
    #[serde(rename = "cluster_name")]
    pub cluster_name: String,
    #[serde(rename = "cluster_type")]
    pub cluster_type: String,
    #[serde(rename = "failed_containers_count")]
    pub failed_containers_count: i32,
    #[serde(rename = "failed_images_count")]
    pub failed_images_count: i32,
    #[serde(rename = "failed_nodes_count")]
    pub failed_nodes_count: i32,
    #[serde(rename = "failed_rules_count")]
    pub failed_rules_count: i32,
    #[serde(rename = "passed_containers_count")]
    pub passed_containers_count: i32,
    #[serde(rename = "passed_images_count")]
    pub passed_images_count: i32,
    #[serde(rename = "passed_nodes_count")]
    pub passed_nodes_count: i32,
    #[serde(rename = "passed_rules_count")]
    pub passed_rules_count: i32,
    #[serde(rename = "percentage_of_passed_assets")]
    pub percentage_of_passed_assets: f64,
    #[serde(rename = "percentage_of_passed_rules")]
    pub percentage_of_passed_rules: f64,
    #[serde(rename = "total_containers_count")]
    pub total_containers_count: i32,
    #[serde(rename = "total_images_count")]
    pub total_images_count: i32,
    #[serde(rename = "total_nodes_count")]
    pub total_nodes_count: i32,
}

impl DomainPeriodApiAggregateGroupByClustersV2 {
    pub fn new(
        cid: String,
        cloud_account_id: String,
        cloud_provider: String,
        cloud_region: String,
        cluster_derived_id: String,
        cluster_id: String,
        cluster_name: String,
        cluster_type: String,
        failed_containers_count: i32,
        failed_images_count: i32,
        failed_nodes_count: i32,
        failed_rules_count: i32,
        passed_containers_count: i32,
        passed_images_count: i32,
        passed_nodes_count: i32,
        passed_rules_count: i32,
        percentage_of_passed_assets: f64,
        percentage_of_passed_rules: f64,
        total_containers_count: i32,
        total_images_count: i32,
        total_nodes_count: i32,
    ) -> DomainPeriodApiAggregateGroupByClustersV2 {
        DomainPeriodApiAggregateGroupByClustersV2 {
            cid,
            cloud_account_id,
            cloud_provider,
            cloud_region,
            cluster_derived_id,
            cluster_id,
            cluster_name,
            cluster_type,
            failed_containers_count,
            failed_images_count,
            failed_nodes_count,
            failed_rules_count,
            passed_containers_count,
            passed_images_count,
            passed_nodes_count,
            passed_rules_count,
            percentage_of_passed_assets,
            percentage_of_passed_rules,
            total_containers_count,
            total_images_count,
            total_nodes_count,
        }
    }
}
