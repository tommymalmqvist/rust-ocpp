use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::FirmwareStatusEnumType;

/// Request to notify the CSMS of the status of a firmware update.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationRequest {
    /// Required. This contains the progress status of the firmware installation.
    pub status: FirmwareStatusEnumType,

    /// Optional. The request id that was provided in the UpdateFirmwareRequest
    /// that started this firmware update. This field is mandatory, unless the
    /// message was triggered by a TriggerMessageRequest AND there is no firmware
    /// update ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a FirmwareStatusNotificationRequest.
/// This response contains no fields other than the optional customData field,
/// because the request cannot be denied by the CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareStatusNotificationResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
