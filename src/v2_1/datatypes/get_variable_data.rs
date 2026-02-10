use super::super::enumerations::AttributeEnumType;
use super::component::ComponentType;
use super::custom_data::CustomDataType;
use super::variable::VariableType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Class to hold parameters for GetVariables request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetVariableDataType {
    /// Required. Component for which the Variable is requested.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable for which the attribute value is requested.
    #[validate(nested)]
    pub variable: VariableType,

    /// Optional. If the variable is attribute-based, this field specifies the attribute type for which the value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GetVariableDataType {
    /// Creates a new `GetVariableDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which the Variable is requested
    /// * `variable` - Variable for which the attribute value is requested
    ///
    /// # Returns
    ///
    /// A new instance of `GetVariableDataType` with optional fields set to `None`
    pub fn new(component: ComponentType, variable: VariableType) -> Self {
        Self {
            component,
            variable,
            attribute_type: None,
            custom_data: None,
        }
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Attribute type for which the value is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_attribute_type(mut self, attribute_type: AttributeEnumType) -> Self {
        self.attribute_type = Some(attribute_type);
        self
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

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component for which the Variable is requested
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
    /// Self reference for method chaining
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable for which the attribute value is requested
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
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// An optional reference to the attribute type for which the value is requested
    pub fn attribute_type(&self) -> Option<&AttributeEnumType> {
        self.attribute_type.as_ref()
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `attribute_type` - Attribute type for which the value is requested, or None to clear
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
    /// * `custom_data` - Custom data for this request, or None to clear
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
    fn test_new_get_variable_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());

        let get_variable_data = GetVariableDataType::new(component.clone(), variable.clone());

        assert_eq!(get_variable_data.component(), &component);
        assert_eq!(get_variable_data.variable(), &variable);
        assert_eq!(get_variable_data.attribute_type(), None);
        assert_eq!(get_variable_data.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let attribute_type = AttributeEnumType::Actual;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let get_variable_data = GetVariableDataType::new(component.clone(), variable.clone())
            .with_attribute_type(attribute_type.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(get_variable_data.component(), &component);
        assert_eq!(get_variable_data.variable(), &variable);
        assert_eq!(get_variable_data.attribute_type(), Some(&attribute_type));
        assert_eq!(get_variable_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let variable1 =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());

        let component2 = ComponentType::new("Meter".to_string());
        let variable2 =
            VariableType::new_with_instance("Voltage".to_string(), "Secondary".to_string());
        let attribute_type = AttributeEnumType::Target;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut get_variable_data = GetVariableDataType::new(component1, variable1);

        get_variable_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_attribute_type(Some(attribute_type.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(get_variable_data.component(), &component2);
        assert_eq!(get_variable_data.variable(), &variable2);
        assert_eq!(get_variable_data.attribute_type(), Some(&attribute_type));
        assert_eq!(get_variable_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        get_variable_data
            .set_attribute_type(None)
            .set_custom_data(None);

        assert_eq!(get_variable_data.attribute_type(), None);
        assert_eq!(get_variable_data.custom_data(), None);
    }

    #[test]
    fn test_serde_serialization() {
        use serde_json;
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let attribute_type = AttributeEnumType::Actual;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let get_variable_data = GetVariableDataType::new(component.clone(), variable.clone())
            .with_attribute_type(attribute_type.clone())
            .with_custom_data(custom_data.clone());

        let serialized = serde_json::to_string(&get_variable_data).unwrap();
        let deserialized: GetVariableDataType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.component(), &component);
        assert_eq!(deserialized.variable(), &variable);
        assert_eq!(deserialized.attribute_type(), Some(&attribute_type));
        assert_eq!(deserialized.custom_data().unwrap().vendor_id, "VendorX");
    }
}
