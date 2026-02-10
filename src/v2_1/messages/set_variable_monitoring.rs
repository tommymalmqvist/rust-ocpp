use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    component::ComponentType, custom_data::CustomDataType,
    set_monitoring_data::SetMonitoringDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::monitor::MonitorEnumType;

/// Status returned in response to SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetMonitoringStatusEnumType {
    Accepted,
    UnknownComponent,
    UnknownVariable,
    UnsupportedMonitorType,
    Rejected,
    Duplicate,
}

/// Class to hold result of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringResultType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Id given to the VariableMonitor by the Charging Station.
    /// The Id is only returned when status is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub id: Option<i32>,

    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Required. The status of the monitoring setting.
    pub status: SetMonitoringStatusEnumType,

    /// Required. The type of this monitor.
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,

    /// Required. Component for which a variable is monitored.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    #[validate(nested)]
    pub variable: VariableType,

    /// Required. The severity that will be assigned to an event that is triggered by this monitor.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    #[validate(range(min = 0, max = 9))]
    pub severity: i32,
}

/// Request to set monitoring parameters for a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringRequest {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of monitoring settings to configure.
    #[validate(length(min = 1), nested)]
    pub set_monitoring_data: Vec<SetMonitoringDataType>,
}

/// Response to SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableMonitoringResponse {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of result statuses per monitoring setting.
    #[validate(length(min = 1), nested)]
    pub set_monitoring_result: Vec<SetMonitoringResultType>,
}

impl SetMonitoringResultType {
    /// Creates a new `SetMonitoringResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Status of the monitoring setting
    /// * `kind` - Type of the monitor
    /// * `component` - Component for which a variable is monitored
    /// * `variable` - Variable that is monitored
    /// * `severity` - Severity level assigned to events triggered by this monitor
    ///
    /// # Returns
    ///
    /// A new instance of `SetMonitoringResultType` with optional fields set to `None`
    pub fn new(
        status: SetMonitoringStatusEnumType,
        kind: MonitorEnumType,
        component: ComponentType,
        variable: VariableType,
        severity: i32,
    ) -> Self {
        Self {
            custom_data: None,
            id: None,
            status_info: None,
            status: status.clone(),
            kind,
            component,
            variable,
            severity,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the monitor ID.
    ///
    /// # Arguments
    ///
    /// * `id` - ID given to the variable monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
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
    /// * `custom_data` - Custom data for this result, or None to clear
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

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// Optional reference to the status info
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Status info to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// Status of the monitoring setting
    pub fn status(&self) -> &SetMonitoringStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Status of the monitoring setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: SetMonitoringStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the monitor type.
    ///
    /// # Returns
    ///
    /// Type of this monitor
    pub fn kind(&self) -> &MonitorEnumType {
        &self.kind
    }

    /// Sets the monitor type.
    ///
    /// # Arguments
    ///
    /// * `kind` - Type of this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_kind(&mut self, kind: MonitorEnumType) -> &mut Self {
        self.kind = kind;
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// Reference to the component for which a variable is monitored
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
    /// Reference to the variable that is monitored
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

    /// Gets the severity.
    ///
    /// # Returns
    ///
    /// Severity level assigned to events triggered by this monitor
    pub fn severity(&self) -> i32 {
        self.severity
    }

    /// Sets the severity.
    ///
    /// # Arguments
    ///
    /// * `severity` - Severity level to assign to events triggered by this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_severity(&mut self, severity: i32) -> &mut Self {
        self.severity = severity;
        self
    }
}

impl SetVariableMonitoringRequest {
    /// Creates a new `SetVariableMonitoringRequest` with required fields.
    ///
    /// # Arguments
    ///
    /// * `set_monitoring_data` - List of monitoring settings to configure
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariableMonitoringRequest` with optional fields set to `None`
    pub fn new(set_monitoring_data: Vec<SetMonitoringDataType>) -> Self {
        Self {
            custom_data: None,
            set_monitoring_data,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this request
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this request, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the monitoring data settings.
    ///
    /// # Returns
    ///
    /// Reference to the list of monitoring settings
    pub fn set_monitoring_data(&self) -> &Vec<SetMonitoringDataType> {
        &self.set_monitoring_data
    }

    /// Sets the monitoring data settings.
    ///
    /// # Arguments
    ///
    /// * `set_monitoring_data` - List of monitoring settings to configure
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_set_monitoring_data(
        &mut self,
        set_monitoring_data: Vec<SetMonitoringDataType>,
    ) -> &mut Self {
        self.set_monitoring_data = set_monitoring_data;
        self
    }
}

impl SetVariableMonitoringResponse {
    /// Creates a new `SetVariableMonitoringResponse` with required fields.
    ///
    /// # Arguments
    ///
    /// * `set_monitoring_result` - List of result statuses per monitoring setting
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariableMonitoringResponse` with optional fields set to `None`
    pub fn new(set_monitoring_result: Vec<SetMonitoringResultType>) -> Self {
        Self {
            custom_data: None,
            set_monitoring_result,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this response
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this response, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the monitoring result settings.
    ///
    /// # Returns
    ///
    /// Reference to the list of result statuses
    pub fn set_monitoring_result(&self) -> &Vec<SetMonitoringResultType> {
        &self.set_monitoring_result
    }

    /// Sets the monitoring result settings.
    ///
    /// # Arguments
    ///
    /// * `set_monitoring_result` - List of result statuses per monitoring setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_set_monitoring_result(
        &mut self,
        set_monitoring_result: Vec<SetMonitoringResultType>,
    ) -> &mut Self {
        self.set_monitoring_result = set_monitoring_result;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::monitor::MonitorEnumType;
    use rust_decimal::prelude::*;
    use serde_json::json;

    #[test]
    fn test_set_monitoring_result_type() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let status = SetMonitoringStatusEnumType::Accepted;

        let result = SetMonitoringResultType::new(
            status.clone(),
            kind.clone(),
            component.clone(),
            variable.clone(),
            severity,
        );

        assert_eq!(result.status(), &status);
        assert_eq!(result.kind(), &kind);
        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.severity(), severity);
        assert_eq!(result.id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_set_variable_monitoring_request() {
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

        let request = SetVariableMonitoringRequest::new(vec![monitoring_data.clone()]);

        assert_eq!(request.set_monitoring_data().len(), 1);
        assert_eq!(request.set_monitoring_data()[0], monitoring_data);
        assert_eq!(request.custom_data(), None);
    }

    #[test]
    fn test_set_variable_monitoring_response() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let status = SetMonitoringStatusEnumType::Accepted;

        let result = SetMonitoringResultType::new(
            status,
            kind.clone(),
            component.clone(),
            variable.clone(),
            severity,
        );

        let response = SetVariableMonitoringResponse::new(vec![result.clone()]);

        assert_eq!(response.set_monitoring_result().len(), 1);
        assert_eq!(response.set_monitoring_result()[0], result);
        assert_eq!(response.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let status = SetMonitoringStatusEnumType::Accepted;
        let id = 42;
        let custom_data = CustomDataType::new("VendorX".to_string());
        let status_info = StatusInfoType::new("Info".to_string());

        let result = SetMonitoringResultType::new(
            status.clone(),
            kind.clone(),
            component.clone(),
            variable.clone(),
            severity,
        )
        .with_id(id)
        .with_custom_data(custom_data.clone())
        .with_status_info(status_info.clone());

        assert_eq!(result.status(), &status);
        assert_eq!(result.kind(), &kind);
        assert_eq!(result.component(), &component);
        assert_eq!(result.variable(), &variable);
        assert_eq!(result.severity(), severity);
        assert_eq!(result.id(), Some(id));
        assert_eq!(result.status_info(), Some(&status_info));
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_serialization() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let value = Decimal::from_str("100.0").unwrap();
        let kind = MonitorEnumType::UpperThreshold;
        let severity = 2;
        let status = SetMonitoringStatusEnumType::Accepted;
        let id = 42;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let monitoring_data = SetMonitoringDataType::new(
            value,
            kind.clone(),
            severity,
            component.clone(),
            variable.clone(),
        );

        let request = SetVariableMonitoringRequest::new(vec![monitoring_data])
            .with_custom_data(custom_data.clone());

        let result = SetMonitoringResultType::new(status, kind, component, variable, severity)
            .with_id(id)
            .with_custom_data(custom_data);

        let response = SetVariableMonitoringResponse::new(vec![result]);

        let serialized_request = serde_json::to_string(&request).unwrap();
        let deserialized_request: SetVariableMonitoringRequest =
            serde_json::from_str(&serialized_request).unwrap();
        assert_eq!(request, deserialized_request);

        let serialized_response = serde_json::to_string(&response).unwrap();
        let deserialized_response: SetVariableMonitoringResponse =
            serde_json::from_str(&serialized_response).unwrap();
        assert_eq!(response, deserialized_response);
    }
}
