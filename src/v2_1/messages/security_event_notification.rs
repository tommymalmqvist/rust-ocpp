use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Request sent by the Charging Station to the CSMS in case of a security event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of the security event. This value should be taken from the Security events list.
    #[validate(length(max = 50))]
    #[serde(rename = "type")]
    pub kind: String,

    /// Required. Date and time at which the event occurred.
    pub timestamp: DateTime<Utc>,

    /// Optional. Additional information about the occurred security event.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub tech_info: Option<String>,
}

/// Response to a SecurityEventNotificationRequest. This message is typically used to acknowledge
/// the receipt of a security event notification. No fields are required beyond the optional CustomData.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SecurityEventNotificationResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
