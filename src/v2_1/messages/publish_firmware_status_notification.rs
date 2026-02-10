use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::publish_firmware_status::PublishFirmwareStatusEnumType;

/// Request to notify the CSMS about the status of a firmware publication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationRequest {
    /// Required. This contains the progress status of the publishfirmware
    /// installation.
    pub status: PublishFirmwareStatusEnumType,

    /// Required if status is Published. Can be multiple URI's, if the Local Controller supports e.g. HTTP, HTTPS, and FTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub location: Option<Vec<String>>,

    /// The request id that was provided in the PublishFirmwareRequest which
    /// triggered this action.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub request_id: Option<i32>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a PublishFirmwareStatusNotificationRequest.
/// This response contains no fields other than the optional customData field,
/// because the request cannot be denied by the CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareStatusNotificationResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
