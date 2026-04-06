use chrono::{DateTime, Utc};
use validator::Validate;

use crate::v1_6::helpers::datetime_rfc3339;

/// This contains the field definition of the GetDiagnostics.req PDU sent by the Central System to the Charge Point. See also Get Diagnostics
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsRequest {
    /// Required. This contains the location (directory) where the diagnostics file shall be uploaded to.
    pub location: String,
    /// Optional. This specifies how many times Charge Point must try to upload the diagnostics before giving up. If this field is not present, it is left to Charge Point to decide how many times it wants to retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charge Point to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    /// Optional. This contains the date and time of the oldest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "datetime_rfc3339::option")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional. This contains the date and time of the latest logging information to include in the diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "datetime_rfc3339::option")]
    pub stop_time: Option<DateTime<Utc>>,
}

/// This contains the field definition of the GetDiagnostics.conf PDU sent by the Charge Point to the Central System in response to a GetDiagnosticsRequest PDU. See also Get Diagnostics
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDiagnosticsResponse {
    /// Optional. This contains the name of the file with diagnostic information that will be uploaded. This field is not present when no diagnostic information is vailable.
    #[validate(length(min = 1, max = 255))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}
