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
pub struct DomainPeriodExecutionMetadataV1 {
    #[serde(rename = "report_params")]
    pub report_params: Box<models::DomainPeriodReportParams>,
    #[serde(rename = "retry_allowed")]
    pub retry_allowed: bool,
    #[serde(rename = "retry_performed")]
    pub retry_performed: bool,
    #[serde(rename = "retry_report_execution_id")]
    pub retry_report_execution_id: String,
    #[serde(rename = "subtype")]
    pub subtype: String,
    #[serde(rename = "unscheduled_execution_type")]
    pub unscheduled_execution_type: String,
    #[serde(rename = "xdr_data")]
    pub xdr_data: Box<models::DomainPeriodXdrData>,
    #[serde(rename = "xdr_params")]
    pub xdr_params: Box<models::DomainPeriodXdrParams>,
}

impl DomainPeriodExecutionMetadataV1 {
    pub fn new(
        report_params: models::DomainPeriodReportParams,
        retry_allowed: bool,
        retry_performed: bool,
        retry_report_execution_id: String,
        subtype: String,
        unscheduled_execution_type: String,
        xdr_data: models::DomainPeriodXdrData,
        xdr_params: models::DomainPeriodXdrParams,
    ) -> DomainPeriodExecutionMetadataV1 {
        DomainPeriodExecutionMetadataV1 {
            report_params: Box::new(report_params),
            retry_allowed,
            retry_performed,
            retry_report_execution_id,
            subtype,
            unscheduled_execution_type,
            xdr_data: Box::new(xdr_data),
            xdr_params: Box::new(xdr_params),
        }
    }
}
