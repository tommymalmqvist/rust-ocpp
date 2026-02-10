use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, status_info::StatusInfoType,
    variable::VariableType,
};
use crate::v2_1::enumerations::{AttributeEnumType, GetVariableStatusEnumType};

/// Class to hold results of GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableResultType {
    /// Required. Component for which the Variable is requested.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable for which the attribute value is requested.
    #[validate(nested)]
    pub variable: VariableType,

    /// Optional. If the variable is attribute-based, this field specifies the attribute type for which the value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,

    /// Optional. Value of the requested attribute if status is Accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2500))]
    pub attribute_value: Option<String>,

    /// Required. Result status of getting the variable.
    pub attribute_status: GetVariableStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub attribute_status_info: Option<StatusInfoType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariableResultType {
    /// Creates a new `GetVariableResultType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the Variable is requested
    /// * `variable` - Variable for which the attribute value is requested
    /// * `attribute_status` - Result status of getting the variable
    ///
    /// # Returns
    ///
    /// A new `GetVariableResultType` instance with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        attribute_status: GetVariableStatusEnumType,
    ) -> Self {
        Self {
            custom_data: None,
            component,
            variable,
            attribute_type: None,
            attribute_value: None,
            attribute_status,
            attribute_status_info: None,
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
    /// The modified `GetVariableResultType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the attribute type field.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - The attribute type for which the value is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_type(mut self, attribute_type: AttributeEnumType) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }

    /// Sets the attribute value field.
    ///
    /// # Arguments
    ///
    /// * `attribute_value` - Value of the requested attribute
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_value(mut self, attribute_value: String) -> Self {
        self.attribute_value = Some(attribute_value);
        self
    }

    /// Sets the attribute status info field.
    ///
    /// # Arguments
    ///
    /// * `attribute_status_info` - Detailed status information
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn with_attribute_status_info(mut self, attribute_status_info: StatusInfoType) -> Self {
        self.attribute_status_info = Some(attribute_status_info);
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
    /// The modified `GetVariableResultType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the Variable is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the attribute value is requested
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute type
    pub fn attribute_type(&self) -> Option<&AttributeEnumType> {
        self.attribute_type.as_ref()
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - The attribute type for which the value is requested, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_type(&mut self, attribute_type: Option<AttributeEnumType>) -> &mut Self {
        self.attribute_type = attribute_type;
        self
    }

    /// Gets the attribute value.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute value
    pub fn attribute_value(&self) -> Option<&str> {
        self.attribute_value.as_deref()
    }

    /// Sets the attribute value.
    ///
    /// # Arguments
    ///
    /// * `attribute_value` - Value of the requested attribute, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_value(&mut self, attribute_value: Option<String>) -> &mut Self {
        self.attribute_value = attribute_value;
        self
    }

    /// Gets the attribute status.
    ///
    /// # Returns
    ///
    /// The attribute status
    pub fn attribute_status(&self) -> &GetVariableStatusEnumType {
        &self.attribute_status
    }

    /// Sets the attribute status.
    ///
    /// # Arguments
    ///
    /// * `attribute_status` - Result status of getting the variable
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_status(
        &mut self,
        attribute_status: GetVariableStatusEnumType,
    ) -> &mut Self {
        self.attribute_status = attribute_status;
        self
    }

    /// Gets the attribute status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute status info
    pub fn attribute_status_info(&self) -> Option<&StatusInfoType> {
        self.attribute_status_info.as_ref()
    }

    /// Sets the attribute status info.
    ///
    /// # Arguments
    ///
    /// * `attribute_status_info` - Detailed status information, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GetVariableResultType` instance
    pub fn set_attribute_status_info(
        &mut self,
        attribute_status_info: Option<StatusInfoType>,
    ) -> &mut Self {
        self.attribute_status_info = attribute_status_info;
        self
    }
}

/// Trait for GetVariableResult operations
pub trait GetVariableResult {
    fn new(
        component: ComponentType,
        variable: VariableType,
        attribute_status: GetVariableStatusEnumType,
    ) -> Self;
    fn with_custom_data(self, custom_data: CustomDataType) -> Self;
    fn with_attribute_type(self, attribute_type: AttributeEnumType) -> Self;
    fn with_attribute_value(self, attribute_value: String) -> Self;
    fn with_attribute_status_info(self, attribute_status_info: StatusInfoType) -> Self;
    fn custom_data(&self) -> Option<&CustomDataType>;
    fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self;
    fn component(&self) -> &ComponentType;
    fn set_component(&mut self, component: ComponentType) -> &mut Self;
    fn variable(&self) -> &VariableType;
    fn set_variable(&mut self, variable: VariableType) -> &mut Self;
    fn attribute_type(&self) -> Option<&AttributeEnumType>;
    fn set_attribute_type(&mut self, attribute_type: Option<AttributeEnumType>) -> &mut Self;
    fn attribute_value(&self) -> Option<&str>;
    fn set_attribute_value(&mut self, attribute_value: Option<String>) -> &mut Self;
    fn attribute_status(&self) -> &GetVariableStatusEnumType;
    fn set_attribute_status(&mut self, attribute_status: GetVariableStatusEnumType) -> &mut Self;
    fn attribute_status_info(&self) -> Option<&StatusInfoType>;
    fn set_attribute_status_info(
        &mut self,
        attribute_status_info: Option<StatusInfoType>,
    ) -> &mut Self;
}

impl GetVariableResult for GetVariableResultType {
    fn new(
        component: ComponentType,
        variable: VariableType,
        attribute_status: GetVariableStatusEnumType,
    ) -> Self {
        GetVariableResultType::new(component, variable, attribute_status)
    }
    fn with_custom_data(self, custom_data: CustomDataType) -> Self {
        self.with_custom_data(custom_data)
    }
    fn with_attribute_type(self, attribute_type: AttributeEnumType) -> Self {
        self.with_attribute_type(attribute_type)
    }
    fn with_attribute_value(self, attribute_value: String) -> Self {
        self.with_attribute_value(attribute_value)
    }
    fn with_attribute_status_info(self, attribute_status_info: StatusInfoType) -> Self {
        self.with_attribute_status_info(attribute_status_info)
    }
    fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data()
    }
    fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.set_custom_data(custom_data)
    }
    fn component(&self) -> &ComponentType {
        self.component()
    }
    fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.set_component(component)
    }
    fn variable(&self) -> &VariableType {
        self.variable()
    }
    fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.set_variable(variable)
    }
    fn attribute_type(&self) -> Option<&AttributeEnumType> {
        self.attribute_type()
    }
    fn set_attribute_type(&mut self, attribute_type: Option<AttributeEnumType>) -> &mut Self {
        self.set_attribute_type(attribute_type)
    }
    fn attribute_value(&self) -> Option<&str> {
        self.attribute_value()
    }
    fn set_attribute_value(&mut self, attribute_value: Option<String>) -> &mut Self {
        self.set_attribute_value(attribute_value)
    }
    fn attribute_status(&self) -> &GetVariableStatusEnumType {
        self.attribute_status()
    }
    fn set_attribute_status(&mut self, attribute_status: GetVariableStatusEnumType) -> &mut Self {
        self.set_attribute_status(attribute_status)
    }
    fn attribute_status_info(&self) -> Option<&StatusInfoType> {
        self.attribute_status_info()
    }
    fn set_attribute_status_info(
        &mut self,
        attribute_status_info: Option<StatusInfoType>,
    ) -> &mut Self {
        self.set_attribute_status_info(attribute_status_info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::component::ComponentType;
    use crate::v2_1::datatypes::custom_data::CustomDataType;
    use crate::v2_1::datatypes::status_info::StatusInfoType;
    use crate::v2_1::datatypes::variable::VariableType;
    use crate::v2_1::enumerations::GetVariableStatusEnumType;

    #[test]
    fn test_new_get_variable_result() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable =
            VariableType::new_with_instance("TestVariable".to_string(), "instance1".to_string());
        let status = GetVariableStatusEnumType::Accepted;

        let result =
            GetVariableResultType::new(component.clone(), variable.clone(), status.clone());
        assert_eq!(result.component().name(), "TestComponent");
        assert_eq!(result.variable().name(), "TestVariable");
        assert_eq!(
            *result.attribute_status(),
            GetVariableStatusEnumType::Accepted
        );
        assert!(result.custom_data().is_none());
        assert!(result.attribute_type().is_none());
        assert!(result.attribute_value().is_none());
        assert!(result.attribute_status_info().is_none());
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable =
            VariableType::new_with_instance("TestVariable".to_string(), "instance1".to_string());
        let status = GetVariableStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("Vendor".to_string());
        let status_info = StatusInfoType {
            custom_data: None,
            reason_code: "Reason".to_string(),
            additional_info: None,
        };

        let result = GetVariableResultType::new(component, variable, status)
            .with_custom_data(custom_data.clone())
            .with_attribute_type(AttributeEnumType::Actual)
            .with_attribute_value("Value".to_string())
            .with_attribute_status_info(status_info.clone());

        assert_eq!(result.custom_data().unwrap().vendor_id(), "Vendor");
        assert_eq!(result.attribute_type().unwrap(), &AttributeEnumType::Actual);
        assert_eq!(result.attribute_value().unwrap(), "Value");
        assert_eq!(
            result.attribute_status_info().unwrap().reason_code,
            "Reason"
        );
    }

    #[test]
    fn test_set_methods() {
        let component = ComponentType::new("TestComponent".to_string());
        let variable =
            VariableType::new_with_instance("TestVariable".to_string(), "instance1".to_string());
        let status = GetVariableStatusEnumType::Accepted;
        let custom_data = CustomDataType::new("Vendor".to_string());
        let status_info = StatusInfoType {
            custom_data: None,
            reason_code: "Reason".to_string(),
            additional_info: None,
        };
        let new_component = ComponentType::new("NewComponent".to_string());
        let new_variable =
            VariableType::new_with_instance("NewVariable".to_string(), "instance2".to_string());
        let new_status = GetVariableStatusEnumType::Rejected;

        let mut result = GetVariableResultType::new(component, variable, status);
        result.set_custom_data(Some(custom_data.clone()));
        result.set_attribute_type(Some(AttributeEnumType::Target));
        result.set_attribute_value(Some("Value".to_string()));
        result.set_attribute_status_info(Some(status_info.clone()));
        result.set_component(new_component.clone());
        result.set_variable(new_variable.clone());
        result.set_attribute_status(new_status.clone());

        assert_eq!(result.custom_data().unwrap().vendor_id(), "Vendor");
        assert_eq!(result.attribute_type().unwrap(), &AttributeEnumType::Target);
        assert_eq!(result.attribute_value().unwrap(), "Value");
        assert_eq!(
            result.attribute_status_info().unwrap().reason_code,
            "Reason"
        );
        assert_eq!(result.component().name(), "NewComponent");
        assert_eq!(result.variable().name(), "NewVariable");
        assert_eq!(
            *result.attribute_status(),
            GetVariableStatusEnumType::Rejected
        );
    }
}
