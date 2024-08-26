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
pub struct ModelPeriodSessionLog {
    #[serde(rename = "base_command")]
    pub base_command: String,
    #[serde(rename = "cloud_request_id")]
    pub cloud_request_id: String,
    #[serde(rename = "command_string")]
    pub command_string: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "current_directory")]
    pub current_directory: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "session_id")]
    pub session_id: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ModelPeriodSessionLog {
    pub fn new(
        base_command: String,
        cloud_request_id: String,
        command_string: String,
        created_at: String,
        current_directory: String,
        id: i32,
        session_id: String,
        updated_at: String,
    ) -> ModelPeriodSessionLog {
        ModelPeriodSessionLog {
            base_command,
            cloud_request_id,
            command_string,
            created_at,
            current_directory,
            id,
            session_id,
            updated_at,
        }
    }
}
