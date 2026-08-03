#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::v1_6::types::DataTransferStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    #[serde(rename = "vendorId")]
    pub vendor_string: String,
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 255)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    /// Required. This indicates the success or failure of the data transfer.
    pub status: DataTransferStatus,
    /// Optional. Data in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}
