/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainExposedDataRecordLocationV1 {
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "federal_admin_region")]
    pub federal_admin_region: String,
    #[serde(rename = "federal_district")]
    pub federal_district: String,
    #[serde(rename = "postal_code")]
    pub postal_code: String,
    #[serde(rename = "state")]
    pub state: String,
}

impl DomainExposedDataRecordLocationV1 {
    pub fn new(city: String, country_code: String, federal_admin_region: String, federal_district: String, postal_code: String, state: String) -> DomainExposedDataRecordLocationV1 {
        DomainExposedDataRecordLocationV1 {
            city,
            country_code,
            federal_admin_region,
            federal_district,
            postal_code,
            state,
        }
    }
}