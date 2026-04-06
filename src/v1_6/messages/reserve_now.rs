use chrono::{DateTime, Utc};

use validator::Validate;

use crate::v1_6::helpers::datetime_rfc3339;
use crate::v1_6::types::ReservationStatus;

/// This contains the field definition of the ReserveNow.req PDU sent by the Central System to the Charge Point. See also Reserve Now
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    /// Required. This contains the id of the connector to be reserved. A value of 0 means that the reservation is not for a specific connector.
    pub connector_id: u32,
    /// Required. This contains the date and time when the reservation ends.
    #[serde(with = "datetime_rfc3339")]
    pub expiry_date: DateTime<Utc>,
    /// Required. The identifier for which the Charge Point has to reserve a connector.
    #[validate(length(min = 1, max = 20))]
    pub id_tag: String, // IdToken, should this be a type?
    /// Optional. The parent idTag.
    #[validate(length(min = 1, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>, // IdToken, shoult this be a type?
    /// Required. Unique id for this reservation.
    pub reservation_id: i32,
}

/// This contains the field definitions of the RemoteStopTransactionResponse PDU sent from Charge Point to Central System. See also Remote Stop Transaction
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowResponse {
    // Required. Status indicating whether Charge Point accepts the request to stop a transaction.
    pub status: ReservationStatus,
}
