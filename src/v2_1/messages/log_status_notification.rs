//! LogStatusNotification
use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::datatypes::StatusInfoType;
use crate::v2_1::enumerations::upload_log_status::UploadLogStatusEnumType;
use validator::Validate;

/// LogStatusNotificationRequest, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationRequest {
    /// This contains the status of the log upload.
    pub status: UploadLogStatusEnumType,

    /// The request id that was provided in GetLogRequest that started this log upload.
    /// This field is mandatory, unless the message was triggered by a TriggerMessageRequest
    /// AND there is no log upload ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /// Optional status information to provide more details about the log upload status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// LogStatusNotificationResponse, sent by the CSMS to the Charging Station in response to LogStatusNotificationRequest.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogStatusNotificationResponse {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
