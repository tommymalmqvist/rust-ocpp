use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::request_start_stop_status_enum_type::RequestStartStopStatusEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// This contains the field definitions of the RequestStopTransactionRequest PDU sent to Charging Station by CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionRequest {
    pub transaction_id: String,
}

/// This contains the field definitions of the RequestStopTransactionResponse PDU sent from Charging Station to CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestStopTransactionResponse {
    pub status: RequestStartStopStatusEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
