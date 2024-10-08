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
pub struct DomainPeriodDiscoverApiDeviceSlot {
    /// The Firmware of the Rack Slot of IoT Asset
    #[serde(rename = "slot_firmware", skip_serializing_if = "Option::is_none")]
    pub slot_firmware: Option<String>,
    /// The IoT asset's IP address
    #[serde(rename = "slot_ip_address", skip_serializing_if = "Option::is_none")]
    pub slot_ip_address: Option<String>,
    /// The IoT asset's MAC address
    #[serde(rename = "slot_mac_address", skip_serializing_if = "Option::is_none")]
    pub slot_mac_address: Option<String>,
    /// The Model of the Rack Slot of IoT Asset
    #[serde(rename = "slot_model", skip_serializing_if = "Option::is_none")]
    pub slot_model: Option<String>,
    /// The Name of the Rack Slot of IoT Asset
    #[serde(rename = "slot_name", skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    /// The Slot Number of the Rack Slot of IoT Asset
    #[serde(rename = "slot_number", skip_serializing_if = "Option::is_none")]
    pub slot_number: Option<i32>,
    /// The Serial Number of the Rack Slot of IoT Asset
    #[serde(rename = "slot_serial_number", skip_serializing_if = "Option::is_none")]
    pub slot_serial_number: Option<String>,
    /// The IoT asset's slot type
    #[serde(rename = "slot_type", skip_serializing_if = "Option::is_none")]
    pub slot_type: Option<String>,
    /// The Vendor of the Rack Slot of IoT Asset
    #[serde(rename = "slot_vendor", skip_serializing_if = "Option::is_none")]
    pub slot_vendor: Option<String>,
}

impl DomainPeriodDiscoverApiDeviceSlot {
    pub fn new() -> DomainPeriodDiscoverApiDeviceSlot {
        DomainPeriodDiscoverApiDeviceSlot {
            slot_firmware: None,
            slot_ip_address: None,
            slot_mac_address: None,
            slot_model: None,
            slot_name: None,
            slot_number: None,
            slot_serial_number: None,
            slot_type: None,
            slot_vendor: None,
        }
    }
}
