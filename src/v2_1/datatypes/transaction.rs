use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::{ChargingStateEnumType, ReasonEnumType};

/// Transaction
/// urn:x-oca:ocpp:uid:2:233318
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionType {
    /// This contains the Id of the transaction.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Optional. The identifier that identifies the current charging state of the charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnumType>,

    /// Transaction. Time_ Spent_ Charging. Elapsed_ Time
    /// urn:x-oca:ocpp:uid:1:569415
    /// Contains the total time that energy flowed from EVSE to EV during the transaction (in seconds).
    /// Note that timeSpentCharging is smaller or equal to the duration of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i32>,

    /// Optional. Reason why the transaction was stopped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<ReasonEnumType>,

    /// The ID given to remote start request (RequestStartTransactionRequest).
    /// This enables to CSMS to match the started transaction to the given start request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TransactionType {
    /// Creates a new `TransactionType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The identifier by which the Charging Station and the CSMS identify the transaction
    ///
    /// # Returns
    ///
    /// A new instance of `TransactionType` with optional fields set to `None`
    pub fn new(transaction_id: String) -> Self {
        Self {
            transaction_id,
            charging_state: None,
            time_spent_charging: None,
            stopped_reason: None,
            remote_start_id: None,
            custom_data: None,
        }
    }

    /// Gets the transaction ID.
    pub fn transaction_id(&self) -> &str {
        &self.transaction_id
    }

    /// Gets the charging state.
    pub fn charging_state(&self) -> Option<&ChargingStateEnumType> {
        self.charging_state.as_ref()
    }

    /// Gets the time spent charging in seconds.
    pub fn time_spent_charging(&self) -> Option<i32> {
        self.time_spent_charging
    }

    /// Gets the reason why the transaction was stopped.
    pub fn stopped_reason(&self) -> Option<&ReasonEnumType> {
        self.stopped_reason.as_ref()
    }

    /// Gets the remote start request ID.
    pub fn remote_start_id(&self) -> Option<i32> {
        self.remote_start_id
    }

    /// Gets the custom data.
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the transaction ID.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction_id(&mut self, transaction_id: String) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Sets the charging state.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_state(
        &mut self,
        charging_state: Option<ChargingStateEnumType>,
    ) -> &mut Self {
        self.charging_state = charging_state;
        self
    }

    /// Sets the time spent charging in seconds.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_spent_charging(&mut self, time_spent_charging: Option<i32>) -> &mut Self {
        self.time_spent_charging = time_spent_charging;
        self
    }

    /// Sets the reason why the transaction was stopped.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stopped_reason(&mut self, stopped_reason: Option<ReasonEnumType>) -> &mut Self {
        self.stopped_reason = stopped_reason;
        self
    }

    /// Sets the remote start request ID.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_remote_start_id(&mut self, remote_start_id: Option<i32>) -> &mut Self {
        self.remote_start_id = remote_start_id;
        self
    }

    /// Sets the custom data.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the charging state using the builder pattern.
    ///
    /// # Returns
    ///
    /// Self with charging state set
    pub fn with_charging_state(mut self, charging_state: ChargingStateEnumType) -> Self {
        self.charging_state = Some(charging_state);
        self
    }

    /// Sets the time spent charging using the builder pattern.
    ///
    /// # Returns
    ///
    /// Self with time spent charging set
    pub fn with_time_spent_charging(mut self, time_spent_charging: i32) -> Self {
        self.time_spent_charging = Some(time_spent_charging);
        self
    }

    /// Sets the reason why the transaction was stopped using the builder pattern.
    ///
    /// # Returns
    ///
    /// Self with stopped reason set
    pub fn with_stopped_reason(mut self, stopped_reason: ReasonEnumType) -> Self {
        self.stopped_reason = Some(stopped_reason);
        self
    }

    /// Sets the remote start request ID using the builder pattern.
    ///
    /// # Returns
    ///
    /// Self with remote start ID set
    pub fn with_remote_start_id(mut self, remote_start_id: i32) -> Self {
        self.remote_start_id = Some(remote_start_id);
        self
    }

    /// Sets the custom data using the builder pattern.
    ///
    /// # Returns
    ///
    /// Self with custom data set
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction() {
        let transaction_id = "TX12345".to_string();
        let transaction = TransactionType::new(transaction_id.clone());

        assert_eq!(transaction.transaction_id(), transaction_id);
        assert_eq!(transaction.charging_state(), None);
        assert_eq!(transaction.time_spent_charging(), None);
        assert_eq!(transaction.stopped_reason(), None);
        assert_eq!(transaction.remote_start_id(), None);
        assert_eq!(transaction.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let transaction_id = "TX12345".to_string();
        let charging_state = ChargingStateEnumType::Charging;
        let time_spent_charging = 3600;
        let stopped_reason = ReasonEnumType::EVDisconnected;
        let remote_start_id = 123;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let transaction = TransactionType::new(transaction_id.clone())
            .with_charging_state(charging_state.clone())
            .with_time_spent_charging(time_spent_charging)
            .with_stopped_reason(stopped_reason.clone())
            .with_remote_start_id(remote_start_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(transaction.transaction_id(), transaction_id);
        assert_eq!(transaction.charging_state(), Some(&charging_state));
        assert_eq!(transaction.time_spent_charging(), Some(time_spent_charging));
        assert_eq!(transaction.stopped_reason(), Some(&stopped_reason));
        assert_eq!(transaction.remote_start_id(), Some(remote_start_id));
        assert_eq!(transaction.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let transaction_id1 = "TX12345".to_string();
        let transaction_id2 = "TX67890".to_string();
        let charging_state = ChargingStateEnumType::Charging;
        let time_spent_charging = 3600;
        let stopped_reason = ReasonEnumType::EVDisconnected;
        let remote_start_id = 123;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut transaction = TransactionType::new(transaction_id1);

        transaction
            .set_transaction_id(transaction_id2.clone())
            .set_charging_state(Some(charging_state.clone()))
            .set_time_spent_charging(Some(time_spent_charging))
            .set_stopped_reason(Some(stopped_reason.clone()))
            .set_remote_start_id(Some(remote_start_id))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(transaction.transaction_id(), transaction_id2);
        assert_eq!(transaction.charging_state(), Some(&charging_state));
        assert_eq!(transaction.time_spent_charging(), Some(time_spent_charging));
        assert_eq!(transaction.stopped_reason(), Some(&stopped_reason));
        assert_eq!(transaction.remote_start_id(), Some(remote_start_id));
        assert_eq!(transaction.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        transaction
            .set_charging_state(None)
            .set_time_spent_charging(None)
            .set_stopped_reason(None)
            .set_remote_start_id(None)
            .set_custom_data(None);

        assert_eq!(transaction.transaction_id(), transaction_id2);
        assert_eq!(transaction.charging_state(), None);
        assert_eq!(transaction.time_spent_charging(), None);
        assert_eq!(transaction.stopped_reason(), None);
        assert_eq!(transaction.remote_start_id(), None);
        assert_eq!(transaction.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let transaction = TransactionType::new("TX12345".to_string())
            .with_charging_state(ChargingStateEnumType::Charging)
            .with_time_spent_charging(3600)
            .with_stopped_reason(ReasonEnumType::EVDisconnected)
            .with_remote_start_id(123);

        let json = serde_json::to_string(&transaction).unwrap();
        let deserialized: TransactionType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, transaction);
    }
}
