use crate::v1_6::helpers::datetime_rfc3339;
use crate::v1_6::types::{IdTagInfo, MeterValue, Reason};

use chrono::{DateTime, Utc};
use validator::Validate;

/// This contains the field definition of the StopTransaction.req PDU sent by the Charge Point to the Central System. See also Stop Transaction
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionRequest {
    /// Required.
    #[validate(length(min = 1, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag: Option<String>, // IdToken, should this be a type?
    /// Optional. Only filled in when request applies to a specific connector.
    pub meter_stop: i32,
    /// Required. This contains the date and time on which the transaction is stopped.
    #[serde(with = "datetime_rfc3339")]
    pub timestamp: DateTime<Utc>,
    /// Required. This contains the transaction-id as received by the StartTransactionResponse
    pub transaction_id: i32,
    /// Optional. This contains the reason why the transaction was stopped. MAY only be omitted when the Reason is "Local".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    /// Optional. This contains transaction usage details relevant for billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_data: Option<Vec<MeterValue>>,
}

/// This contains the field definition of the TriggerMessage.req PDU sent by the Central System to the Charge Point. See also Trigger Message
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse {
    /// Optional. This contains information about authorization status, expiry and parent id. It is optional, because a transaction may have been stopped without an identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}
