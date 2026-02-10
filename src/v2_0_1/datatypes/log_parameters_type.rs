use crate::v2_0_1::helpers::datetime_rfc3339;
use chrono::DateTime;
use chrono::Utc;

/// Generic class for the configuration of logging entries.
/// LogParametersType is used by: GetLogRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    pub remote_location: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub oldest_timestamp: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub latest_timestamp: Option<DateTime<Utc>>,
}
