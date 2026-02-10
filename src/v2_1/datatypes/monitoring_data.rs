use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, variable::VariableType,
    variable_monitoring::VariableMonitoringType,
};

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// Required. Component for which a variable is monitored.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable that is monitored.
    #[validate(nested)]
    pub variable: VariableType,

    /// Required. The type of this monitor, e.g. a threshold, delta or periodic monitor.
    #[validate(length(min = 1), nested)]
    pub variable_monitoring: Vec<VariableMonitoringType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MonitoringDataType {
    /// Creates a new `MonitoringDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a variable is monitored
    /// * `variable` - Variable that is monitored
    /// * `variable_monitoring` - The type of this monitor (threshold, delta or periodic)
    ///
    /// # Returns
    ///
    /// A new instance of `MonitoringDataType` with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        variable_monitoring: Vec<VariableMonitoringType>,
    ) -> Self {
        Self {
            component,
            variable,
            variable_monitoring,
            custom_data: None,
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
    /// Self for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// Mutable reference to self for method chaining
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
    /// Mutable reference to self for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the variable monitoring types.
    ///
    /// # Returns
    ///
    /// A reference to the vector of variable monitoring types
    pub fn variable_monitoring(&self) -> &Vec<VariableMonitoringType> {
        &self.variable_monitoring
    }

    /// Sets the variable monitoring types.
    ///
    /// # Arguments
    ///
    /// * `variable_monitoring` - The vector of monitoring types (threshold, delta or periodic)
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_variable_monitoring(
        &mut self,
        variable_monitoring: Vec<VariableMonitoringType>,
    ) -> &mut Self {
        self.variable_monitoring = variable_monitoring;
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
    /// Mutable reference to self for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::MonitorEnumType;
    use rust_decimal_macros::dec;
    use validator::Validate;
    #[test]
    fn test_new_monitoring_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            false,
            dec!(80.0),
            MonitorEnumType::UpperThreshold,
            0,
            crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
        )];

        let monitoring_data = MonitoringDataType::new(
            component.clone(),
            variable.clone(),
            variable_monitoring.clone(),
        );

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            false,
            dec!(80.0),
            MonitorEnumType::UpperThreshold,
            0,
            crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
        )];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let monitoring_data = MonitoringDataType::new(
            component.clone(),
            variable.clone(),
            variable_monitoring.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(monitoring_data.component(), &component);
        assert_eq!(monitoring_data.variable(), &variable);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring);
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let variable1 =
            VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring1 = vec![VariableMonitoringType::new(
            1,
            false,
            dec!(80.0),
            MonitorEnumType::UpperThreshold,
            0,
            crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
        )];

        let component2 = ComponentType::new("Meter".to_string());
        let variable2 =
            VariableType::new_with_instance("Current".to_string(), "Output".to_string());
        let variable_monitoring2 = vec![
            VariableMonitoringType::new(
                2,
                false,
                dec!(5.0),
                MonitorEnumType::LowerThreshold,
                0,
                crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
            ),
            VariableMonitoringType::new(
                3,
                false,
                dec!(32.0),
                MonitorEnumType::UpperThreshold,
                0,
                crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
            ),
        ];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut monitoring_data =
            MonitoringDataType::new(component1, variable1, variable_monitoring1);

        monitoring_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_variable_monitoring(variable_monitoring2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(monitoring_data.component(), &component2);
        assert_eq!(monitoring_data.variable(), &variable2);
        assert_eq!(monitoring_data.variable_monitoring(), &variable_monitoring2);
        assert_eq!(monitoring_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        monitoring_data.set_custom_data(None);
        assert_eq!(monitoring_data.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // 有效的MonitoringDataType实例
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        let variable_monitoring = vec![VariableMonitoringType::new(
            1,
            false,
            dec!(80.0),
            MonitorEnumType::UpperThreshold,
            0,
            crate::v2_1::enumerations::event_notification::EventNotificationEnumType::CustomMonitor,
        )];

        let valid_monitoring_data = MonitoringDataType::new(
            component.clone(),
            variable.clone(),
            variable_monitoring.clone(),
        );
        assert!(
            valid_monitoring_data.validate().is_ok(),
            "有效的MonitoringDataType应通过验证"
        );

        // 测试空的variable_monitoring数组（应该失败，因为最小长度为1）
        let mut invalid_monitoring_data = valid_monitoring_data.clone();
        invalid_monitoring_data.variable_monitoring = vec![];
        assert!(
            invalid_monitoring_data.validate().is_err(),
            "空的variable_monitoring数组应验证失败"
        );

        // 测试嵌套验证 - 使用无效的ComponentType
        let mut invalid_component = ComponentType::new("Connector".to_string());
        invalid_component.name = "a".repeat(51); // 超过最大长度50

        let mut monitoring_data_with_invalid_component = valid_monitoring_data.clone();
        monitoring_data_with_invalid_component.component = invalid_component;
        assert!(
            monitoring_data_with_invalid_component.validate().is_err(),
            "包含无效Component的MonitoringDataType应验证失败"
        );

        // 测试嵌套验证 - 使用无效的VariableType
        let mut invalid_variable =
            VariableType::new_with_instance("Temperature".to_string(), "Outlet".to_string());
        invalid_variable.name = "a".repeat(51); // 超过最大长度50

        let mut monitoring_data_with_invalid_variable = valid_monitoring_data.clone();
        monitoring_data_with_invalid_variable.variable = invalid_variable;
        assert!(
            monitoring_data_with_invalid_variable.validate().is_err(),
            "包含无效Variable的MonitoringDataType应验证失败"
        );

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let mut monitoring_data_with_invalid_custom_data = valid_monitoring_data.clone();
        monitoring_data_with_invalid_custom_data.custom_data = Some(invalid_custom_data);
        assert!(
            monitoring_data_with_invalid_custom_data.validate().is_err(),
            "包含无效CustomData的MonitoringDataType应验证失败"
        );
    }
}
