use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};
use crate::v2_1::enumerations::attribute::AttributeEnumType;

/// Class to hold parameters of SetVariable request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    /// Required. Component for which the variable is set.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable which holds the attribute value.
    #[validate(nested)]
    pub variable: VariableType,

    /// Required. Value to be assigned to attribute of variable.
    /// This value is allowed to be an empty string ("").
    ///
    /// The Configuration Variable <<configkey-configuration-value-size,ConfigurationValueSize>>
    /// can be used to limit SetVariableData.attributeValue and VariableCharacteristics.valuesList.
    /// The max size of these values will always remain equal.
    #[validate(length(max = 2500))]
    pub attribute_value: String,

    /// Optional. Type of attribute that is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SetVariableDataType {
    /// Creates a new `SetVariableDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the variable is set
    /// * `variable` - Variable which holds the attribute value
    /// * `attribute_value` - Value to be assigned to attribute of variable
    ///
    /// # Returns
    ///
    /// A new instance of `SetVariableDataType` with optional fields set to `None`
    pub fn new(component: ComponentType, variable: VariableType, attribute_value: String) -> Self {
        Self {
            custom_data: None,
            component,
            variable,
            attribute_value,
            attribute_type: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this variable data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Type of attribute that is set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_attribute_type(mut self, attribute_type: AttributeEnumType) -> Self {
        self.attribute_type = Some(attribute_type);
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which the variable is set
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the variable is set
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
    /// A reference to the variable which holds the attribute value
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable which holds the attribute value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the attribute value.
    ///
    /// # Returns
    ///
    /// The value to be assigned to attribute of variable
    pub fn attribute_value(&self) -> &str {
        &self.attribute_value
    }

    /// Sets the attribute value.
    ///
    /// # Arguments
    ///
    /// * `attribute_value` - Value to be assigned to attribute of variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_attribute_value(&mut self, attribute_value: String) -> &mut Self {
        self.attribute_value = attribute_value;
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// An optional type of attribute that is set
    pub fn attribute_type(&self) -> Option<&AttributeEnumType> {
        self.attribute_type.as_ref()
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Type of attribute that is set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_attribute_type(&mut self, attribute_type: Option<AttributeEnumType>) -> &mut Self {
        self.attribute_type = attribute_type;
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
    /// * `custom_data` - Custom data for this variable data, or None to clear
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
    fn test_new_set_variable_data() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let attribute_value = "value1".to_string();

        let data =
            SetVariableDataType::new(component.clone(), variable.clone(), attribute_value.clone());

        assert_eq!(data.component(), &component);
        assert_eq!(data.variable(), &variable);
        assert_eq!(data.attribute_value(), attribute_value);
        assert_eq!(data.attribute_type(), None);
        assert_eq!(data.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("component1".to_string());
        let variable =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let attribute_value = "value1".to_string();
        let attribute_type = AttributeEnumType::Actual;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let data =
            SetVariableDataType::new(component.clone(), variable.clone(), attribute_value.clone())
                .with_attribute_type(attribute_type.clone())
                .with_custom_data(custom_data.clone());

        assert_eq!(data.component(), &component);
        assert_eq!(data.variable(), &variable);
        assert_eq!(data.attribute_value(), attribute_value);
        assert_eq!(data.attribute_type(), Some(&attribute_type));
        assert_eq!(data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("component1".to_string());
        let variable1 =
            VariableType::new_with_instance("variable1".to_string(), "instance1".to_string());
        let attribute_value1 = "value1".to_string();

        let mut data = SetVariableDataType::new(component1, variable1, attribute_value1);

        let component2 = ComponentType::new("component2".to_string());
        let variable2 =
            VariableType::new_with_instance("variable2".to_string(), "instance2".to_string());
        let attribute_value2 = "value2".to_string();
        let attribute_type = AttributeEnumType::MinSet;
        let custom_data = CustomDataType::new("VendorX".to_string());

        data.set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_attribute_value(attribute_value2.clone())
            .set_attribute_type(Some(attribute_type.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(data.component(), &component2);
        assert_eq!(data.variable(), &variable2);
        assert_eq!(data.attribute_value(), attribute_value2);
        assert_eq!(data.attribute_type(), Some(&attribute_type));
        assert_eq!(data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        data.set_attribute_type(None).set_custom_data(None);

        assert_eq!(data.attribute_type(), None);
        assert_eq!(data.custom_data(), None);
    }
}
