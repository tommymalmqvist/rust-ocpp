use crate::v2_0_1::helpers::datetime_rfc3339;
use chrono::DateTime;
use chrono::Utc;

/// Represents a copy of the firmware that can be loaded/updated on the Charging Station.
/// FirmwareType is used by: UpdateFirmwareRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    pub location: String,
    #[serde(with = "datetime_rfc3339 ")]
    pub retrieve_date_time: DateTime<Utc>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub install_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}
