#[cfg(not(feature = "std"))]
use alloc::string::String;
use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::helpers::datetime_rfc3339;

/// Sent by the Charging Station to the CSMS in case of a security event.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(with = "datetime_rfc3339 ")]
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
}

/// Sent by the CSMS to the Charging Station to confirm the receipt of a SecurityEventNotificationRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationResponse {}
