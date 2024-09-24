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
pub struct ClientPeriodJobStatus {
    #[serde(rename = "content_length", skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i32>,
    #[serde(rename = "digest_algo", skip_serializing_if = "Option::is_none")]
    pub digest_algo: Option<String>,
    #[serde(rename = "digest_hex", skip_serializing_if = "Option::is_none")]
    pub digest_hex: Option<String>,
    #[serde(rename = "event_count", skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i32>,
    #[serde(rename = "file_link", skip_serializing_if = "Option::is_none")]
    pub file_link: Option<String>,
    #[serde(
        rename = "filtered_event_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub filtered_event_count: Option<i32>,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "job_url", skip_serializing_if = "Option::is_none")]
    pub job_url: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "percent_complete", skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
    #[serde(rename = "result_count", skip_serializing_if = "Option::is_none")]
    pub result_count: Option<i32>,
    #[serde(rename = "run_duration", skip_serializing_if = "Option::is_none")]
    pub run_duration: Option<f64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ClientPeriodJobStatus {
    pub fn new(job_id: String) -> ClientPeriodJobStatus {
        ClientPeriodJobStatus {
            content_length: None,
            digest_algo: None,
            digest_hex: None,
            event_count: None,
            file_link: None,
            filtered_event_count: None,
            job_id,
            job_url: None,
            message: None,
            percent_complete: None,
            result_count: None,
            run_duration: None,
            status: None,
        }
    }
}