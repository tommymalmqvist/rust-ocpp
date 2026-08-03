//! GetTransactionStatus

/// With this message, the CSMS can ask the Charging Station whether it has transaction-related messages waiting to be delivered to the CSMS. When a transactionId is provided, only messages for a specific transaction are asked for.
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusRequest {
    /// The Id of the transaction for which the status isrequested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// This contains the field definition of the GetReportRequest, PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionStatusResponse {
    /// Whether the transaction is still ongoing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ongoing_indicator: Option<bool>,
    /// Whether there are still message to be delivered.
    pub messages_in_queue: bool,
}
