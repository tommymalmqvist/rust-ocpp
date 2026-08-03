#[cfg(not(feature = "std"))]
use alloc::string::String;
use serde::{Deserialize, Serialize};

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::RequestStartStopStatusEnumType,
};

/// Request body for the RequestStopTransaction request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier of the transaction which the Charging Station is requested to stop.
    #[cfg_attr(feature = "std", validate(length(max = 36)))]
    pub transaction_id: String,
}

/// Response body for the RequestStopTransaction response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether Charging Station accepts the request to stop a transaction.
    pub status: RequestStartStopStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
