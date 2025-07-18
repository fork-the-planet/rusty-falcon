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
pub struct DomainPeriodScanV2 {
    #[serde(rename = "affected_hosts_count")]
    pub affected_hosts_count: i32,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(
        rename = "cloud_ml_level_detection",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_ml_level_detection: Option<i32>,
    #[serde(
        rename = "cloud_ml_level_prevention",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_ml_level_prevention: Option<i32>,
    #[serde(
        rename = "cloud_pup_adware_level_detection",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_pup_adware_level_detection: Option<i32>,
    #[serde(
        rename = "cloud_pup_adware_level_prevention",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_pup_adware_level_prevention: Option<i32>,
    #[serde(rename = "completed_host_count")]
    pub completed_host_count: i32,
    #[serde(rename = "cpu_priority", skip_serializing_if = "Option::is_none")]
    pub cpu_priority: Option<i32>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "created_on", skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "endpoint_notification",
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_notification: Option<bool>,
    #[serde(rename = "file_paths", skip_serializing_if = "Option::is_none")]
    pub file_paths: Option<Vec<String>>,
    #[serde(rename = "filecount", skip_serializing_if = "Option::is_none")]
    pub filecount: Option<Box<models::DomainPeriodFileCountV2>>,
    #[serde(rename = "host_groups", skip_serializing_if = "Option::is_none")]
    pub host_groups: Option<Vec<String>>,
    #[serde(rename = "hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "incomplete_host_count")]
    pub incomplete_host_count: i32,
    #[serde(rename = "initiated_from", skip_serializing_if = "Option::is_none")]
    pub initiated_from: Option<String>,
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "max_duration", skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<i32>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<String>>,
    #[serde(rename = "missing_host_count")]
    pub missing_host_count: i32,
    #[serde(rename = "not_started_host_count")]
    pub not_started_host_count: i32,
    #[serde(rename = "pause_duration", skip_serializing_if = "Option::is_none")]
    pub pause_duration: Option<i32>,
    #[serde(rename = "policy_setting", skip_serializing_if = "Option::is_none")]
    pub policy_setting: Option<Vec<i32>>,
    #[serde(
        rename = "preemption_priority",
        skip_serializing_if = "Option::is_none"
    )]
    pub preemption_priority: Option<i32>,
    #[serde(rename = "profile_id", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    #[serde(rename = "quarantine", skip_serializing_if = "Option::is_none")]
    pub quarantine: Option<bool>,
    #[serde(rename = "scan_completed_on", skip_serializing_if = "Option::is_none")]
    pub scan_completed_on: Option<String>,
    #[serde(rename = "scan_exclusions", skip_serializing_if = "Option::is_none")]
    pub scan_exclusions: Option<Vec<String>>,
    #[serde(rename = "scan_inclusions", skip_serializing_if = "Option::is_none")]
    pub scan_inclusions: Option<Vec<String>>,
    #[serde(rename = "scan_started_on", skip_serializing_if = "Option::is_none")]
    pub scan_started_on: Option<String>,
    #[serde(
        rename = "sensor_ml_level_detection",
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_ml_level_detection: Option<i32>,
    #[serde(
        rename = "sensor_ml_level_prevention",
        skip_serializing_if = "Option::is_none"
    )]
    pub sensor_ml_level_prevention: Option<i32>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    #[serde(rename = "started_host_count")]
    pub started_host_count: i32,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targeted_host_count")]
    pub targeted_host_count: i32,
}

impl DomainPeriodScanV2 {
    pub fn new(
        affected_hosts_count: i32,
        completed_host_count: i32,
        id: String,
        incomplete_host_count: i32,
        missing_host_count: i32,
        not_started_host_count: i32,
        started_host_count: i32,
        targeted_host_count: i32,
    ) -> DomainPeriodScanV2 {
        DomainPeriodScanV2 {
            affected_hosts_count,
            cid: None,
            cloud_ml_level_detection: None,
            cloud_ml_level_prevention: None,
            cloud_pup_adware_level_detection: None,
            cloud_pup_adware_level_prevention: None,
            completed_host_count,
            cpu_priority: None,
            created_by: None,
            created_on: None,
            description: None,
            endpoint_notification: None,
            file_paths: None,
            filecount: None,
            host_groups: None,
            hosts: None,
            id,
            incomplete_host_count,
            initiated_from: None,
            last_updated: None,
            max_duration: None,
            metadata: None,
            missing_host_count,
            not_started_host_count,
            pause_duration: None,
            policy_setting: None,
            preemption_priority: None,
            profile_id: None,
            quarantine: None,
            scan_completed_on: None,
            scan_exclusions: None,
            scan_inclusions: None,
            scan_started_on: None,
            sensor_ml_level_detection: None,
            sensor_ml_level_prevention: None,
            severity: None,
            started_host_count,
            status: None,
            targeted_host_count,
        }
    }
}
