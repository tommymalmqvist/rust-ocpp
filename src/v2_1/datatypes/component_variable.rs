use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{component::ComponentType, custom_data::CustomDataType, variable::VariableType};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentVariableType {
    /// Required. Component for which a report of Variable is requested.
    #[validate(nested)]
    pub component: ComponentType,

    /// Optional. Variable(s) for which the report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub variable: Option<VariableType>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ComponentVariableType {
    /// Creates a new `ComponentVariableType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    ///
    /// # Returns
    ///
    /// A new instance of `ComponentVariableType` with optional fields set to `None`
    pub fn new(component: ComponentType) -> Self {
        Self {
            component,
            custom_data: None,
            variable: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this component variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the report is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_variable(mut self, variable: VariableType) -> Self {
        self.variable = Some(variable);
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
    /// * `component` - Component for which a report of Variable is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
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
    /// * `custom_data` - Custom data for this component variable, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// An optional reference to the variable
    pub fn variable(&self) -> Option<&VariableType> {
        self.variable.as_ref()
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which the report is requested, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: Option<VariableType>) -> &mut Self {
        self.variable = variable;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::evse::EVSEType;
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_new_component_variable() {
        let component = ComponentType::new("Connector".to_string());
        let component_variable = ComponentVariableType::new(component.clone());

        assert_eq!(component_variable.component(), &component);
        assert_eq!(component_variable.custom_data(), None);
        assert_eq!(component_variable.variable(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());

        let component_variable = ComponentVariableType::new(component.clone())
            .with_custom_data(custom_data.clone())
            .with_variable(variable.clone());

        assert_eq!(component_variable.component(), &component);
        assert_eq!(component_variable.custom_data(), Some(&custom_data));
        assert_eq!(component_variable.variable(), Some(&variable));
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let component2 = ComponentType::new("Meter".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Secondary".to_string());

        let mut component_variable = ComponentVariableType::new(component1);

        component_variable
            .set_component(component2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_variable(Some(variable.clone()));

        assert_eq!(component_variable.component(), &component2);
        assert_eq!(component_variable.custom_data(), Some(&custom_data));
        assert_eq!(component_variable.variable(), Some(&variable));

        // Test clearing optional fields
        component_variable.set_custom_data(None).set_variable(None);

        assert_eq!(component_variable.custom_data(), None);
        assert_eq!(component_variable.variable(), None);
    }

    #[test]
    fn test_component_variable_with_evse() {
        // Test with EVSE in the component
        let evse = EVSEType {
            id: 1,
            connector_id: Some(2),
            custom_data: None,
        };

        let component = ComponentType::new("Connector".to_string()).with_evse(evse.clone());
        let component_variable = ComponentVariableType::new(component.clone());

        assert_eq!(component_variable.component().evse(), Some(&evse));
    }

    #[test]
    fn test_component_variable_with_instance() {
        // Test with instance in the component
        let component =
            ComponentType::new("Connector".to_string()).with_instance("Main".to_string());

        let component_variable = ComponentVariableType::new(component.clone());

        assert_eq!(component_variable.component().instance(), Some("Main"));
    }

    #[test]
    fn test_component_variable_serialization() {
        // Test serialization of ComponentVariableType
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let component_variable = ComponentVariableType::new(component)
            .with_variable(variable)
            .with_custom_data(custom_data);

        let serialized = serde_json::to_value(&component_variable).unwrap();

        // Check that the serialized JSON has the expected structure
        assert_eq!(serialized["component"]["name"], "Connector");
        assert_eq!(serialized["variable"]["name"], "CurrentLimit");
        assert_eq!(serialized["variable"]["instance"], "Main");
        assert_eq!(serialized["customData"]["vendorId"], "VendorX");
    }

    #[test]
    fn test_component_variable_deserialization() {
        // Test deserialization of ComponentVariableType
        let json = json!({
            "component": {
                "name": "Connector"
            },
            "variable": {
                "name": "CurrentLimit",
                "instance": "Main"
            },
            "customData": {
                "vendorId": "VendorX"
            }
        });

        let component_variable: ComponentVariableType = serde_json::from_value(json).unwrap();

        assert_eq!(component_variable.component().name(), "Connector");
        assert_eq!(
            component_variable.variable().unwrap().name(),
            "CurrentLimit"
        );
        assert_eq!(
            component_variable.variable().unwrap().instance(),
            Some("Main")
        );
        assert_eq!(
            component_variable.custom_data().unwrap().vendor_id(),
            "VendorX"
        );
    }

    #[test]
    fn test_component_variable_validation() {
        // Test validation of ComponentVariableType
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());

        let component_variable = ComponentVariableType::new(component).with_variable(variable);

        // This should validate successfully
        assert!(component_variable.validate().is_ok());

        // Test with a component name that's too long (>50 chars)
        let long_name = "A".repeat(51);
        let invalid_component = ComponentType::new(long_name);
        let invalid_component_variable = ComponentVariableType::new(invalid_component);

        // This should fail validation
        assert!(invalid_component_variable.validate().is_err());
    }

    #[test]
    fn test_component_variable_with_custom_data_properties() {
        // Test with custom data that has additional properties
        let mut custom_data = CustomDataType::new("VendorX".to_string());
        custom_data.set_property("version".to_string(), json!("1.0"));
        custom_data.set_property("features".to_string(), json!(["feature1", "feature2"]));

        let component = ComponentType::new("Connector".to_string());
        let component_variable =
            ComponentVariableType::new(component).with_custom_data(custom_data);

        let serialized = serde_json::to_value(&component_variable).unwrap();

        // Check that the custom properties are included in the serialized JSON
        assert_eq!(serialized["customData"]["vendorId"], "VendorX");
        assert_eq!(serialized["customData"]["version"], "1.0");
        assert_eq!(
            serialized["customData"]["features"],
            json!(["feature1", "feature2"])
        );
    }

    #[test]
    fn test_component_variable_schema_structure() {
        // Test that the structure matches the OCPP 2.1 schema definition
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let component_variable = ComponentVariableType::new(component)
            .with_variable(variable)
            .with_custom_data(custom_data);

        let serialized = serde_json::to_value(&component_variable).unwrap();

        // Check that the serialized JSON has only the expected fields
        let obj = serialized.as_object().unwrap();
        assert_eq!(obj.len(), 3); // Should have exactly component, variable, and customData
        assert!(obj.contains_key("component"));
        assert!(obj.contains_key("variable"));
        assert!(obj.contains_key("customData"));
    }

    #[test]
    fn test_component_variable_optional_fields() {
        // Test that optional fields are skipped when serializing if None
        let component = ComponentType::new("Connector".to_string());
        let component_variable = ComponentVariableType::new(component);

        let serialized = serde_json::to_value(&component_variable).unwrap();

        // Check that optional fields are not included
        let obj = serialized.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Should have only component
        assert!(obj.contains_key("component"));
        assert!(!obj.contains_key("variable"));
        assert!(!obj.contains_key("customData"));
    }
}
