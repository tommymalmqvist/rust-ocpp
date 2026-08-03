//! DataTransfer
#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::data_transfer_status_enum_type::DataTransferStatusEnumType;

/// DataTransferRequest, sent either by the CSMS to the Charging Station or vice versa.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct DataTransferRequest {
    /// May be used to indicate a specific message or implementation.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// Data without specified length or format. This needs to be decided by both parties (Open to implementation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// This identifies the Vendor specific     implementation
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 255)))]
    pub vendor_id: String,
}

/// This contains the field definition of the DataTransferResponse PDU sent by the Charging Station to the CSMS or vice versa in response to a DataTransferRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DataTransferResponse {
    /// This indicates the success or failure of the data transfer.
    pub status: DataTransferStatusEnumType,
    /// Data without specified length or format, in response to request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
