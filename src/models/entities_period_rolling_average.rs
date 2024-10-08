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
pub struct EntitiesPeriodRollingAverage {
    #[serde(rename = "chrome_os")]
    pub chrome_os: f64,
    #[serde(rename = "containers")]
    pub containers: f64,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "lumos")]
    pub lumos: f64,
    #[serde(rename = "mobile")]
    pub mobile: f64,
    #[serde(rename = "public_cloud_with_containers")]
    pub public_cloud_with_containers: f64,
    #[serde(rename = "public_cloud_without_containers")]
    pub public_cloud_without_containers: f64,
    #[serde(rename = "servers_with_containers")]
    pub servers_with_containers: f64,
    #[serde(rename = "servers_without_containers")]
    pub servers_without_containers: f64,
    #[serde(rename = "workstations")]
    pub workstations: f64,
}

impl EntitiesPeriodRollingAverage {
    pub fn new(
        chrome_os: f64,
        containers: f64,
        date: String,
        lumos: f64,
        mobile: f64,
        public_cloud_with_containers: f64,
        public_cloud_without_containers: f64,
        servers_with_containers: f64,
        servers_without_containers: f64,
        workstations: f64,
    ) -> EntitiesPeriodRollingAverage {
        EntitiesPeriodRollingAverage {
            chrome_os,
            containers,
            date,
            lumos,
            mobile,
            public_cloud_with_containers,
            public_cloud_without_containers,
            servers_with_containers,
            servers_without_containers,
            workstations,
        }
    }
}
