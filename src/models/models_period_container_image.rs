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
pub struct ModelsPeriodContainerImage {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cluster_info")]
    pub cluster_info: Vec<models::ModelsPeriodClusterInfo>,
    #[serde(rename = "container_count")]
    pub container_count: i32,
    #[serde(rename = "containers_running_status")]
    pub containers_running_status: std::collections::HashMap<String, bool>,
    #[serde(rename = "hosts")]
    pub hosts: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "image_detection_count")]
    pub image_detection_count: i32,
    #[serde(rename = "image_digest")]
    pub image_digest: String,
    #[serde(rename = "image_has_been_assessed")]
    pub image_has_been_assessed: bool,
    #[serde(rename = "image_id")]
    pub image_id: String,
    #[serde(rename = "image_name")]
    pub image_name: String,
    #[serde(rename = "image_registry")]
    pub image_registry: String,
    #[serde(rename = "image_repository")]
    pub image_repository: String,
    #[serde(rename = "image_tag")]
    pub image_tag: String,
    #[serde(rename = "image_vulnerability_count")]
    pub image_vulnerability_count: i32,
    #[serde(rename = "last_seen")]
    pub last_seen: String,
    #[serde(rename = "running_container_count")]
    pub running_container_count: i32,
}

impl ModelsPeriodContainerImage {
    pub fn new(
        cid: String,
        cluster_info: Vec<models::ModelsPeriodClusterInfo>,
        container_count: i32,
        containers_running_status: std::collections::HashMap<String, bool>,
        hosts: Vec<String>,
        id: String,
        image_detection_count: i32,
        image_digest: String,
        image_has_been_assessed: bool,
        image_id: String,
        image_name: String,
        image_registry: String,
        image_repository: String,
        image_tag: String,
        image_vulnerability_count: i32,
        last_seen: String,
        running_container_count: i32,
    ) -> ModelsPeriodContainerImage {
        ModelsPeriodContainerImage {
            cid,
            cluster_info,
            container_count,
            containers_running_status,
            hosts,
            id,
            image_detection_count,
            image_digest,
            image_has_been_assessed,
            image_id,
            image_name,
            image_registry,
            image_repository,
            image_tag,
            image_vulnerability_count,
            last_seen,
            running_container_count,
        }
    }
}
