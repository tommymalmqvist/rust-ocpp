//! # From OCPP Specification
//! Central System can request a Charge Point to unlock a connector. To do so, the Central System
//! SHALL send an UnlockConnector.req PDU.
//!
//! The purpose of this message: Help EV drivers that have problems unplugging their cable from the
//! Charge Point in case of malfunction of the Connector cable retention. When a EV driver calls
//! the CPO help-desk, an operator could manually trigger the sending of an UnlockConnector.req to
//! the Charge Point, forcing a new attempt to unlock the connector. Hopefully this time the
//! connector unlocks and the EV driver can unplug the cable and drive away.
//!  
//! The UnlockConnector.req SHOULD NOT be used to remotely stop a running transaction, use the
//! Remote Stop Transaction instead.
//!
//! Upon receipt of an UnlockConnector.req PDU, the Charge Point SHALL respond with a
//! UnlockConnector.conf PDU. The response PDU SHALL indicate whether the Charge Point was able to
//! unlock its connector.
//! If there was a transaction in progress on the specific connector, then Charge Point SHALL
//! finish the transaction first as described in Stop Transaction.
//!
//! UnlockConnector.req is intented only for unlocking the cable retention lock on the Connector,
//! not for unlocking a connector access door.

use crate::v1_6::types::UnlockStatus;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorRequest {
    /// # From OCPP Specification
    /// Required. This contains the identifier of the connector to be unlocked.
    #[cfg_attr(feature = "std", validate(range(min = 0, max = 20)))]
    pub connector_id: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct UnlockConnectorResponse {
    /// # From OCPP Specification
    /// Required. This indicates whether the Charge Point has unlocked the connector.
    pub status: UnlockStatus,
}
