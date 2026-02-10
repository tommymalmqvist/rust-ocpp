use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType,
    periodic_event_stream_params::PeriodicEventStreamParamsType, variable::VariableType,
};
use crate::v2_1::enumerations::monitor::MonitorEnumType;

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    /// An id SHALL only be given to replace an existing monitor. The Charging Station handles the generation of id's for new monitors.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub id: Option<i32>,

    /// Parameters for periodic event stream configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub periodic_event_stream: Option<PeriodicEventStreamParamsType>,

    /// Monitor only active when a transaction is ongoing on a component relevant to this transaction. Default = false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,

    /// Value for threshold or delta monitoring.
    /// For Periodic or PeriodicClockAligned this is the interval in seconds.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub value: Decimal,

    /// The type of this monitor, e.g. a threshold, delta or periodic monitor.
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,

    /// The severity that will be assigned to an event that is triggered by this monitor. The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    ///
    /// The severity levels have the following meaning:
    /// *0-Danger*
    /// Indicates lives are potentially in danger. Urgent attention is needed and action should be taken immediately.
    /// *1-Hardware Failure*
    /// Indicates that the Charging Station is unable to continue regular operations due to Hardware issues. Action is required.
    /// *2-System Failure*
    /// Indicates that the Charging Station is unable to continue regular operations due to software or minor hardware issues. Action is required.
    /// *3-Critical*
    /// Indicates a critical error. Action is required.
    /// *4-Error*
    /// Indicates a non-urgent error. Action is required.
    /// *5-Alert*
    /// Indicates an alert event. Default severity for any type of monitoring event.
    /// *6-Warning*
    /// Indicates a warning event. Action may be required.
    /// *7-Notice*
    /// Indicates an unusual event. No immediate action is required.
    /// *8-Informational*
    /// Indicates a regular operational event. May be used for reporting, measuring throughput, etc. No action is required.
    /// *9-Debug*
    /// Indicates information useful to developers for debugging, not useful during operations.
    #[validate(range(min = 0, max = 9))]
    pub severity: i32,

    /// Required. Component for which a variable is monitored.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    #[validate(nested)]
    pub variable: VariableType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringDataType {
    /// Creates a new `SetMonitoringDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `value` - Value for threshold or delta monitoring
    /// * `kind` - The type of this monitor
    /// * `severity` - The severity level assigned to triggered events
    /// * `component` - Component for which a variable is monitored
    /// * `variable` - Variable that is monitored
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringDataType` with optional fields set to `None`
    pub fn new(
        value: Decimal,
        kind: MonitorEnumType,
        severity: i32,
        component: ComponentType,
        variable: VariableType,
    ) -> Self {
        Self {
            custom_data: None,
            id: None,
            periodic_event_stream: None,
            transaction: None,
            value,
            kind,
            severity,
            component,
            variable,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this monitoring data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - An id to replace an existing monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the periodic event stream parameters.
    ///
    /// # Arguments
    ///
    /// * `periodic_event_stream` - Parameters for periodic event stream configuration
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_periodic_event_stream(
        mut self,
        periodic_event_stream: PeriodicEventStreamParamsType,
    ) -> Self {
        self.periodic_event_stream = Some(periodic_event_stream);
        self
    }

    /// Sets if the monitor is only active during transactions.
    ///
    /// # Arguments
    ///
    /// * `transaction` - If the monitor is only active during transactions
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_transaction(mut self, transaction: bool) -> Self {
        self.transaction = Some(transaction);
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
    /// * `custom_data` - Custom data for this monitoring data, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the ID of the monitor.
    ///
    /// # Returns
    ///
    /// The optional ID of the monitor
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Sets the ID of the monitor.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the monitor, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: Option<i32>) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the periodic event stream parameters.
    ///
    /// # Returns
    ///
    /// Optional reference to the periodic event stream parameters
    pub fn periodic_event_stream(&self) -> Option<&PeriodicEventStreamParamsType> {
        self.periodic_event_stream.as_ref()
    }

    /// Sets the periodic event stream parameters.
    ///
    /// # Arguments
    ///
    /// * `periodic_event_stream` - Parameters for periodic event stream configuration, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_periodic_event_stream(
        &mut self,
        periodic_event_stream: Option<PeriodicEventStreamParamsType>,
    ) -> &mut Self {
        self.periodic_event_stream = periodic_event_stream;
        self
    }

    /// Gets whether the monitor is only active during transactions.
    ///
    /// # Returns
    ///
    /// Whether the monitor is only active during transactions
    pub fn transaction(&self) -> Option<bool> {
        self.transaction
    }

    /// Sets whether the monitor is only active during transactions.
    ///
    /// # Arguments
    ///
    /// * `transaction` - Whether the monitor is only active during transactions, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction(&mut self, transaction: Option<bool>) -> &mut Self {
        self.transaction = transaction;
        self
    }

    /// Gets the value for threshold or delta monitoring.
    ///
    /// # Returns
    ///
    /// The value for threshold or delta monitoring
    pub fn value(&self) -> Decimal {
        self.value
    }

    /// Sets the value for threshold or delta monitoring.
    ///
    /// # Arguments
    ///
    /// * `value` - Value for threshold or delta monitoring
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: Decimal) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the monitor type.
    ///
    /// # Returns
    ///
    /// The type of this monitor
    pub fn kind(&self) -> &MonitorEnumType {
        &self.kind
    }

    /// Sets the monitor type.
    ///
    /// # Arguments
    ///
    /// * `kind` - The type of this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_kind(&mut self, kind: MonitorEnumType) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Gets the severity.
    ///
    /// # Returns
    ///
    /// The severity that will be assigned to an event
    pub fn severity(&self) -> i32 {
        self.severity
    }

    /// Sets the severity.
    ///
    /// # Arguments
    ///
    /// * `severity` - The severity that will be assigned to an event
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_severity(&mut self, severity: i32) -> &mut Self {
        self.severity = severity;
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which a variable is monitored
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a variable is monitored
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable that is monitored
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable that is monitored
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::*;
    use serde_json::json;

    #[test]
    fn test_new_set_monitoring_data() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let value = Decimal::from_str("100.0").unwrap();
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;

        let monitoring_data = SetMonitoringDataType::new(
            value,
            kind.clone(),
            severity,
            component.clone(),
            variable.clone(),
        );

        assert_eq!(monitoring_data.value(), value);
        assert_eq!(monitoring_data.kind(), &kind);
        assert_eq!(monitoring_data.severity(), severity);
        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.id(), None);
        assert_eq!(monitoring_data.transaction(), None);
        assert_eq!(monitoring_data.periodic_event_stream(), None);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let value = Decimal::from_str("100.0").unwrap();
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let id = 42;
        let transaction = true;
        let periodic_params = PeriodicEventStreamParamsType::new(60, 10);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let monitoring_data = SetMonitoringDataType::new(
            value,
            kind.clone(),
            severity,
            component.clone(),
            variable.clone(),
        )
        .with_id(id)
        .with_transaction(transaction)
        .with_periodic_event_stream(periodic_params.clone())
        .with_custom_data(custom_data.clone());

        assert_eq!(monitoring_data.value(), value);
        assert_eq!(monitoring_data.kind(), &kind);
        assert_eq!(monitoring_data.severity(), severity);
        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.id(), Some(id));
        assert_eq!(monitoring_data.transaction(), Some(transaction));
        assert_eq!(
            monitoring_data.periodic_event_stream(),
            Some(&periodic_params)
        );
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("component1".to_string());
        let variable1 =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let value1 = Decimal::from_str("100.0").unwrap();
        let kind1 = MonitorEnumType::UpperThreshold;
        let severity1 = 2;

        let mut monitoring_data =
            SetMonitoringDataType::new(value1, kind1, severity1, component1, variable1);

        let component2 = ComponentType::new("component2".to_string());
        let variable2 =
            VariableType::new_with_instance("variable2".to_string(), "instance2".to_string());
        let value2 = Decimal::from_str("50.0").unwrap();
        let kind2 = MonitorEnumType::LowerThreshold;
        let severity2 = 3;
        let id = 42;
        let transaction = true;
        let periodic_params = PeriodicEventStreamParamsType::new(60, 10);
        let custom_data = CustomDataType::new("VendorX".to_string());

        monitoring_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_value(value2)
            .set_kind(kind2.clone())
            .set_severity(severity2)
            .set_id(Some(id))
            .set_transaction(Some(transaction))
            .set_periodic_event_stream(Some(periodic_params.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring_data.value(), value2);
        assert_eq!(monitoring_data.kind(), &kind2);
        assert_eq!(monitoring_data.severity(), severity2);
        assert_eq!(monitoring_data.component(), &component2);
        assert_eq!(monitoring_data.variable(), &variable2);
        assert_eq!(monitoring_data.id(), Some(id));
        assert_eq!(monitoring_data.transaction(), Some(transaction));
        assert_eq!(
            monitoring_data.periodic_event_stream(),
            Some(&periodic_params)
        );
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring_data
            .set_id(None)
            .set_transaction(None)
            .set_periodic_event_stream(None)
            .set_custom_data(None);

        assert_eq!(monitoring_data.id(), None);
        assert_eq!(monitoring_data.transaction(), None);
        assert_eq!(monitoring_data.periodic_event_stream(), None);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let value = Decimal::from_str("100.0").unwrap();
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let id = 42;
        let transaction = true;
        let periodic_params = PeriodicEventStreamParamsType::new(60, 10);
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let monitoring_data =
            SetMonitoringDataType::new(value, kind, severity, component, variable)
                .with_id(id)
                .with_transaction(transaction)
                .with_periodic_event_stream(periodic_params)
                .with_custom_data(custom_data);

        let serialized = serde_json::to_string(&monitoring_data).unwrap();
        let deserialized: SetMonitoringDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(monitoring_data, deserialized);
    }
}
