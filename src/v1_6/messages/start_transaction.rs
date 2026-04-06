use crate::v1_6::helpers::datetime_rfc3339;
use crate::v1_6::types::IdTagInfo;

use chrono::{DateTime, Utc};
use validator::Validate;

/// This section contains the field definition of the StartTransaction.req PDU sent by the Charge Point to the Central System. See also Start Transaction
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest {
    /// Required. This identifies which connector of the Charge Point is used.
    pub connector_id: u32,
    /// Required. This contains the identifier for which a transaction has to be started.
    #[validate(length(min = 1, max = 20))]
    pub id_tag: String, // IdToken, should this be a type?
    /// Required. This contains the meter value in Wh for the connector at start of the transaction.
    pub meter_start: i32,
    /// Optional. This contains the id of the reservation that terminates as a result of this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    /// Required. This contains the date and time on which the transaction is started.
    #[serde(with = "datetime_rfc3339")]
    pub timestamp: DateTime<Utc>,
}

/// This contains the field definition of the StartTransaction.conf PDU sent by the Central System to the Charge Point in response to a StartTransaction.req PDU. See also Start Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse {
    /// Required. This contains information about authorization status, expiry and parent id
    pub id_tag_info: IdTagInfo,
    /// Required. This contains the transaction id supplied by the Central System.
    pub transaction_id: i32,
}
