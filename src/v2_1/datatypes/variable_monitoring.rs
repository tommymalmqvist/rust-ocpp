use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::event_notification::EventNotificationEnumType;
use crate::v2_1::enumerations::monitor::MonitorEnumType;

/// A monitoring setting for a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    /// Required. Identifies the monitor.
    pub id: i32,

    /// Required. Monitor only active when a transaction is ongoing on a component
    /// relevant to this transaction.
    pub transaction: bool,

    /// Required. Value for threshold or delta monitoring.
    /// For Periodic or PeriodicClockAligned this is the interval in seconds.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub value: Decimal,

    /// Required. Monitor type of the variable.
    #[serde(rename = "type")]
    pub type_: MonitorEnumType,

    /// Required. The severity that will be assigned to an event that is triggered
    /// by this monitor.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    pub severity: i32,

    /// Required. Specifies the event notification type of the message.
    pub event_notification_type: EventNotificationEnumType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VariableMonitoringType {
    /// Creates a new `VariableMonitoringType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifies the monitor
    /// * `transaction` - Monitor only active when a transaction is ongoing on a component relevant to this transaction
    /// * `value` - Value for threshold or delta monitoring
    /// * `type_` - Monitor type of the variable
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor
    /// * `event_notification_type` - Specifies the event notification type of the message
    ///
    /// # Returns
    ///
    /// A new `VariableMonitoringType` instance with optional fields set to `None`
    pub fn new(
        id: i32,
        transaction: bool,
        value: Decimal,
        type_: MonitorEnumType,
        severity: i32,
        event_notification_type: EventNotificationEnumType,
    ) -> Self {
        Self {
            id,
            transaction,
            value,
            type_,
            severity,
            event_notification_type,
            custom_data: None,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the id.
    ///
    /// # Returns
    ///
    /// The ID of the monitor
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID of the monitor.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the transaction flag.
    ///
    /// # Returns
    ///
    /// The transaction flag indicating if the monitor is only active during a transaction
    pub fn transaction(&self) -> bool {
        self.transaction
    }

    /// Sets the transaction flag.
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction flag indicating if the monitor is only active during a transaction
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_transaction(&mut self, transaction: bool) -> &mut Self {
        self.transaction = transaction;
        self
    }

    /// Gets the value for threshold or delta.
    ///
    /// # Returns
    ///
    /// The value for threshold or delta of the monitor
    pub fn value(&self) -> &Decimal {
        &self.value
    }

    /// Sets the value for threshold or delta.
    ///
    /// # Arguments
    ///
    /// * `value` - Value for threshold or delta of the monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_value(&mut self, value: Decimal) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the monitor type.
    ///
    /// # Returns
    ///
    /// The monitor type of the variable
    pub fn type_(&self) -> &MonitorEnumType {
        &self.type_
    }

    /// Sets the monitor type.
    ///
    /// # Arguments
    ///
    /// * `type_` - Monitor type of the variable
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_type(&mut self, type_: MonitorEnumType) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the severity level.
    ///
    /// # Returns
    ///
    /// The severity that will be assigned to an event that is triggered by this monitor
    pub fn severity(&self) -> i32 {
        self.severity
    }

    /// Sets the severity level.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_severity(&mut self, severity: i32) -> &mut Self {
        self.severity = severity;
        self
    }

    /// Gets the event notification type.
    ///
    /// # Returns
    ///
    /// The event notification type of the message
    pub fn event_notification_type(&self) -> &EventNotificationEnumType {
        &self.event_notification_type
    }

    /// Sets the event notification type.
    ///
    /// # Arguments
    ///
    /// * `event_notification_type` - The event notification type of the message
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_event_notification_type(
        &mut self,
        event_notification_type: EventNotificationEnumType,
    ) -> &mut Self {
        self.event_notification_type = event_notification_type;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableMonitoringType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_variable_monitoring_new() {
        let id = 42;
        let transaction = true;
        let value = dec!(100.0);
        let monitor_type = MonitorEnumType::UpperThreshold;
        let severity = 5;
        let event_notification_type = EventNotificationEnumType::CustomMonitor;

        let monitoring = VariableMonitoringType::new(
            id,
            transaction,
            value.clone(),
            monitor_type.clone(),
            severity,
            event_notification_type.clone(),
        );

        assert_eq!(monitoring.id(), id);
        assert_eq!(monitoring.transaction(), transaction);
        assert_eq!(monitoring.value(), &value);
        assert_eq!(monitoring.type_(), &monitor_type);
        assert_eq!(monitoring.severity(), severity);
        assert_eq!(
            monitoring.event_notification_type(),
            &event_notification_type
        );
        assert_eq!(monitoring.custom_data(), None);
    }

    #[test]
    fn test_variable_monitoring_with_custom_data() {
        let id = 42;
        let transaction = true;
        let value = dec!(100.0);
        let monitor_type = MonitorEnumType::UpperThreshold;
        let severity = 5;
        let event_notification_type = EventNotificationEnumType::CustomMonitor;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let monitoring = VariableMonitoringType::new(
            id,
            transaction,
            value.clone(),
            monitor_type.clone(),
            severity,
            event_notification_type.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(monitoring.id(), id);
        assert_eq!(monitoring.transaction(), transaction);
        assert_eq!(monitoring.value(), &value);
        assert_eq!(monitoring.type_(), &monitor_type);
        assert_eq!(monitoring.severity(), severity);
        assert_eq!(
            monitoring.event_notification_type(),
            &event_notification_type
        );
        assert_eq!(monitoring.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_variable_monitoring_setters() {
        let id1 = 42;
        let transaction1 = true;
        let value1 = dec!(100.0);
        let monitor_type1 = MonitorEnumType::UpperThreshold;
        let severity1 = 5;
        let event_notification_type1 = EventNotificationEnumType::CustomMonitor;

        let mut monitoring = VariableMonitoringType::new(
            id1,
            transaction1,
            value1,
            monitor_type1.clone(),
            severity1,
            event_notification_type1.clone(),
        );

        let id2 = 43;
        let transaction2 = false;
        let value2 = dec!(50.0);
        let monitor_type2 = MonitorEnumType::LowerThreshold;
        let severity2 = 3;
        let event_notification_type2 = EventNotificationEnumType::HardWiredMonitor;
        let custom_data = CustomDataType::new("VendorX".to_string());

        monitoring
            .set_id(id2)
            .set_transaction(transaction2)
            .set_value(value2.clone())
            .set_type(monitor_type2.clone())
            .set_severity(severity2)
            .set_event_notification_type(event_notification_type2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring.id(), id2);
        assert_eq!(monitoring.transaction(), transaction2);
        assert_eq!(monitoring.value(), &value2);
        assert_eq!(monitoring.type_(), &monitor_type2);
        assert_eq!(monitoring.severity(), severity2);
        assert_eq!(
            monitoring.event_notification_type(),
            &event_notification_type2
        );
        assert_eq!(monitoring.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring.set_custom_data(None);
        assert_eq!(monitoring.custom_data(), None);
    }
}
