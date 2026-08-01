#[cfg(not(feature = "std"))]
use alloc::string::String;
use serde::{Deserialize, Serialize};

use crate::v2_1::datatypes::CustomDataType;

/// Request to get the status of a transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    /// Optional. The Id of the transaction for which the status is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "std", validate(length(max = 36)))]
    pub transaction_id: Option<String>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetTransactionStatusRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusResponse {
    /// Optional. Whether the transaction is still ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,

    /// Required. Whether there are still messages to be delivered.
    pub messages_in_queue: bool,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
