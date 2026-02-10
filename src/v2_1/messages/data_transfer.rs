use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::DataTransferStatusEnumType,
};

/// Request to transfer vendor-specific data between Charging Station and CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// Required. This identifies the vendor specific implementation.
    #[validate(length(max = 255))]
    pub vendor_id: String,

    /// Optional. May be used to indicate a specific message or implementation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub message_id: Option<String>,

    /// Optional. Data without specified length or format.
    /// This needs to be decided by both parties (Open to implementation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a DataTransferRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatusEnumType,

    /// Optional. Data without specified length or format, in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
