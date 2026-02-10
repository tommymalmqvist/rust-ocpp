use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::{MonitorEnumType, SetMonitoringStatusEnumType};

/// Class to hold result of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    /// Required. Status indicating whether the Charging Station accepts the monitoring request.
    pub status: SetMonitoringStatusEnumType,

    /// Required. Component for which the monitoring status is returned.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable for which the monitoring status is returned.
    #[validate(nested)]
    pub variable: VariableType,

    /// Id given to the VariableMonitor by the Charging Station. The Id is only returned when status is accepted.
    /// Installed VariableMonitors should have unique id's but the id's of removed Installed monitors
    /// should have unique id's but the id's of removed monitors MAY be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub id: Option<i32>,

    /// Required. Type of monitor that was set.
    #[serde(rename = "type")]
    pub type_: MonitorEnumType,

    /// Required. The severity that will be assigned to an event that is triggered by this monitor.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
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
    #[validate(range(min = 0))]
    pub severity: i32,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetMonitoringResultType {
    /// Creates a new `SetMonitoringResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the Charging Station accepts the monitoring request
    /// * `component` - Component for which the monitoring status is returned
    /// * `variable` - Variable for which the monitoring status is returned
    /// * `type_` - Type of monitor that was set
    /// * `severity` - The severity that will be assigned to an event that is triggered by this monitor
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringResultType` with optional fields set to `None`
    pub fn new(
        status: SetMonitoringStatusEnumType,
        component: ComponentType,
        variable: VariableType,
        type_: MonitorEnumType,
        severity: i32,
    ) -> Self {
        Self {
            custom_data: None,
            status,
            component,
            variable,
            id: None,
            type_,
            severity,
            status_info: None,
        }
    }

    /// Sets the id.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the monitor that was set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this monitoring result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Detailed status information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the type of monitor.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_type(mut self, type_: MonitorEnumType) -> Self {
        self.type_ = type_;
        self
    }

    /// Sets the severity of the monitor.
    ///
    /// # Arguments
    ///
    /// * `severity` - Severity level
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_severity(mut self, severity: i32) -> Self {
        self.severity = severity;
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// The status indicating whether the Charging Station accepts the monitoring request
    pub fn status(&self) -> &SetMonitoringStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the Charging Station accepts the monitoring request
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: SetMonitoringStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which the monitoring status is returned
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the monitoring status is returned
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
    /// A reference to the variable for which the monitoring status is returned
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the monitoring status is returned
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the id of the monitor.
    ///
    /// # Returns
    ///
    /// The optional id of the monitor that was set
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Sets the id of the monitor.
    ///
    /// # Arguments
    ///
    /// * `id` - Optional id of the monitor that was set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: Option<i32>) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the type of monitor.
    ///
    /// # Returns
    ///
    /// The type of monitor that was set
    pub fn type_(&self) -> &MonitorEnumType {
        &self.type_
    }

    /// Sets the type of monitor.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of monitor to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: MonitorEnumType) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the severity of the monitor.
    ///
    /// # Returns
    ///
    /// The severity that will be assigned to an event triggered by this monitor
    pub fn severity(&self) -> i32 {
        self.severity
    }

    /// Sets the severity of the monitor.
    ///
    /// # Arguments
    ///
    /// * `severity` - Severity to assign to events triggered by this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_severity(&mut self, severity: i32) -> &mut Self {
        self.severity = severity;
        self
    }

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// An optional reference to detailed status information
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Detailed status information, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
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
    /// * `custom_data` - Custom data for this monitoring result, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_monitoring_result() {
        let status = SetMonitoringStatusEnumType::Accepted;
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let monitor_type = MonitorEnumType::UpperThreshold;
        let severity = 5;

        let result = SetMonitoringResultType::new(
            status.clone(),
            component.clone(),
            variable.clone(),
            monitor_type.clone(),
            severity,
        );

        assert_eq!(result.status(), &status);
        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.id(), None);
        assert_eq!(result.type_(), &monitor_type);
        assert_eq!(result.severity(), severity);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let status = SetMonitoringStatusEnumType::Accepted;
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let id = 42;
        let monitor_type = MonitorEnumType::Delta;
        let severity = 3;
        let new_monitor_type = MonitorEnumType::Periodic;
        let new_severity = 7;
        let custom_data = CustomDataType::new("VendorX".to_string());
        let status_info = StatusInfoType::new("SomeReason".to_string());

        let result = SetMonitoringResultType::new(
            status.clone(),
            component.clone(),
            variable.clone(),
            monitor_type,
            severity,
        )
        .with_id(id)
        .with_custom_data(custom_data.clone())
        .with_status_info(status_info.clone())
        .with_type(new_monitor_type.clone())
        .with_severity(new_severity);

        assert_eq!(result.status(), &status);
        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.id(), Some(id));
        assert_eq!(result.type_(), &new_monitor_type);
        assert_eq!(result.severity(), new_severity);
        assert_eq!(result.status_info(), Some(&status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let status1 = SetMonitoringStatusEnumType::Accepted;
        let component1 = ComponentType::new("component1".to_string());
        let variable1 =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let type1 = MonitorEnumType::UpperThreshold;
        let severity1 = 2;

        let mut result =
            SetMonitoringResultType::new(status1, component1, variable1, type1, severity1);

        let status2 = SetMonitoringStatusEnumType::UnknownVariable;
        let component2 = ComponentType::new("component2".to_string());
        let variable2 =
            VariableType::new_with_instance("variable2".to_string(), "instance2".to_string());
        let id2 = 43;
        let type2 = MonitorEnumType::PeriodicClockAligned;
        let severity2 = 9;
        let custom_data = CustomDataType::new("VendorX".to_string());
        let status_info = StatusInfoType::new("NotFound".to_string());

        result
            .set_status(status2.clone())
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_id(Some(id2))
            .set_type(type2.clone())
            .set_severity(severity2)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.status(), &status2);
        assert_eq!(result.component(), &component2);
        assert_eq!(result.variable(), &variable2);
        assert_eq!(result.id(), Some(id2));
        assert_eq!(result.type_(), &type2);
        assert_eq!(result.severity(), severity2);
        assert_eq!(result.status_info(), Some(&status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_id(None)
            .set_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }
}
