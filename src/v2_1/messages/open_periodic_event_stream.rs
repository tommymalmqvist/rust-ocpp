use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ConstantStreamDataType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::generic_status::GenericStatusEnumType;

/// Request to open a periodic event stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamRequest {
    /// Required. Data for the constant stream.
    pub constant_stream_data: ConstantStreamDataType,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to an OpenPeriodicEventStreamRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OpenPeriodicEventStreamResponse {
    /// Required. Result of the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
