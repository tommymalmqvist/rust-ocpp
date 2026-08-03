#[cfg(not(feature = "std"))]
use alloc::string::String;
use chrono::{DateTime, Utc};

/// This contains the field definition of the UpdateFirmware.req PDU sent by the Central System to the Charge Point. See also Update Firmware
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareRequest {
    /// Required. This contains a string containing a URI pointing to a location from which to retrieve the firmware.
    pub location: String,
    /// Optional. This specifies how many times Charge Point must try to download the firmware before giving up. If this field is not present, it is left to Charge Point to decide how many times it wants to retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// Required. This contains the date and time after which the Charge Point is allowed to retrieve the (new) firmware.
    pub retrieve_date: DateTime<Utc>,
    /// Optional. The interval in seconds after which a retry may be attempted. If this field is not present, it is left to Charge Point to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

/// This contains the field definition of the UpdateFirmware.conf PDU sent by the Charge Point to the Central System in response to a UpdateFirmware.req PDU. See also Update Firmware
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFirmwareResponse {
    // No fields are defined.
}
