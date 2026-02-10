use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingProfileType, CustomDataType, IdTokenType, StatusInfoType},
    enumerations::RequestStartStopStatusEnumType,
};

/// Request body for the RequestStartTransaction request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Number of the EVSE on which to start the transaction. EvseId SHALL be > 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1))]
    pub evse_id: Option<i32>,

    /// Optional. Group authorization reference to use when starting a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,

    /// Required. Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Required. Id given by the server to this start request. The Charging Station will return this in the TransactionEventRequest, letting the server know which transaction was started for this request.
    pub remote_start_id: i32,

    /// Optional. Charging profile to be used for this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile: Option<ChargingProfileType>,
}

/// Response body for the RequestStartTransaction response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RequestStartTransactionResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request to start a transaction.
    pub status: RequestStartStopStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. When the transaction was already started by the Charging Station before the RequestStartTransactionRequest was received, for example: cable plugged in first. This contains the transactionId of the already started transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,
}
