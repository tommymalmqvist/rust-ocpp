use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Request to notify the CSMS that a web payment has been started.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedRequest {
    /// Required. EVSE id for which transaction is requested.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. Timeout value in seconds after which no result of web payment process (e.g. QR code scanning) is to be expected anymore.
    pub timeout: i32,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyWebPaymentStartedRequest. This message has no fields except for optional custom data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyWebPaymentStartedResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
