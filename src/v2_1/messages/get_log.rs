use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, LogParametersType, StatusInfoType},
    enumerations::LogEnumType,
    enumerations::LogStatusEnumType,
};

/// Request to get logging information from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    /// Required. This contains the type of log file that the Charging Station should send.
    pub log_type: LogEnumType,

    /// Required. The Id of this request.
    pub request_id: i32,

    /// Required. This field specifies the requested log and the location to which the log should be sent.
    pub log: LogParametersType,

    /// Optional. This specifies how many times the Charging Station must retry to upload the log before giving up.
    /// If this field is not present, it is left to Charging Station to decide how many times it wants to retry.
    /// If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub retries: Option<i32>,

    /// Optional. The interval in seconds after which a retry may be attempted.
    /// If this field is not present, it is left to Charging Station to decide how long to wait between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetLogRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    /// Required. This field indicates whether the Charging Station was able to accept the request.
    pub status: LogStatusEnumType,

    /// Optional. This contains the name of the log file that will be uploaded.
    /// This field is not present when no logging information is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 255))]
    pub filename: Option<String>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
