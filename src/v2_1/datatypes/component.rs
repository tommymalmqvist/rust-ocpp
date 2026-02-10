use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, evse::EVSEType};

/// A physical or logical component
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,

    /// Specifies the EVSE when component is located at EVSE level, also specifies the connector when component is located at Connector level.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub evse: Option<EVSEType>,

    /// Name of the component. Name should be taken from the list of standardized component names whenever possible.
    /// Case Insensitive. strongly advised to use Camel Case.
    #[validate(length(max = 50))]
    pub name: String,

    /// Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub instance: Option<String>,
}

impl ComponentType {
    /// Creates a new `ComponentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the component
    ///
    /// # Returns
    ///
    /// A new instance of `ComponentType` with optional fields set to `None`
    pub fn new(name: String) -> Self {
        Self {
            name,
            custom_data: None,
            evse: None,
            instance: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the EVSE.
    ///
    /// # Arguments
    ///
    /// * `evse` - EVSE for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse(mut self, evse: EVSEType) -> Self {
        self.evse = Some(evse);
        self
    }

    /// Sets the instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Instance name for this component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    /// Gets the name.
    ///
    /// # Returns
    ///
    /// A reference to the name of the component
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the component
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
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
    /// * `custom_data` - Custom data for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the EVSE.
    ///
    /// # Returns
    ///
    /// An optional reference to the EVSE
    pub fn evse(&self) -> Option<&EVSEType> {
        self.evse.as_ref()
    }

    /// Sets the EVSE.
    ///
    /// # Arguments
    ///
    /// * `evse` - EVSE for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse(&mut self, evse: Option<EVSEType>) -> &mut Self {
        self.evse = evse;
        self
    }

    /// Gets the instance.
    ///
    /// # Returns
    ///
    /// An optional reference to the instance name
    pub fn instance(&self) -> Option<&str> {
        self.instance.as_deref()
    }

    /// Sets the instance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Instance name for this component, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_instance(&mut self, instance: Option<String>) -> &mut Self {
        self.instance = instance;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_new_component() {
        let component = ComponentType::new("Connector".to_string());

        assert_eq!(component.name(), "Connector");
        assert_eq!(component.custom_data(), None);
        assert_eq!(component.evse(), None);
        assert_eq!(component.instance(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let evse = EVSEType {
            id: 1,
            connector_id: Some(2),
            custom_data: None,
        };

        let component = ComponentType::new("Connector".to_string())
            .with_custom_data(custom_data.clone())
            .with_evse(evse.clone())
            .with_instance("Main".to_string());

        assert_eq!(component.name(), "Connector");
        assert_eq!(component.custom_data(), Some(&custom_data));
        assert_eq!(component.evse(), Some(&evse));
        assert_eq!(component.instance(), Some("Main"));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let evse = EVSEType {
            id: 1,
            connector_id: Some(2),
            custom_data: None,
        };

        let mut component = ComponentType::new("Connector".to_string());

        component
            .set_name("Meter".to_string())
            .set_custom_data(Some(custom_data.clone()))
            .set_evse(Some(evse.clone()))
            .set_instance(Some("Secondary".to_string()));

        assert_eq!(component.name(), "Meter");
        assert_eq!(component.custom_data(), Some(&custom_data));
        assert_eq!(component.evse(), Some(&evse));
        assert_eq!(component.instance(), Some("Secondary"));

        // Test clearing optional fields
        component
            .set_custom_data(None)
            .set_evse(None)
            .set_instance(None);

        assert_eq!(component.custom_data(), None);
        assert_eq!(component.evse(), None);
        assert_eq!(component.instance(), None);
    }

    #[test]
    fn test_component_serialization() {
        // Test serialization of ComponentType
        let custom_data = CustomDataType::new("VendorX".to_string());
        let evse = EVSEType::new(1).with_connector_id(2);

        let component = ComponentType::new("Connector".to_string())
            .with_custom_data(custom_data)
            .with_evse(evse)
            .with_instance("Main".to_string());

        let serialized = serde_json::to_value(&component).unwrap();

        // Check that the serialized JSON has the expected structure
        assert_eq!(serialized["name"], "Connector");
        assert_eq!(serialized["instance"], "Main");
        assert_eq!(serialized["customData"]["vendorId"], "VendorX");
        assert_eq!(serialized["evse"]["id"], 1);
        assert_eq!(serialized["evse"]["connectorId"], 2);
    }

    #[test]
    fn test_component_deserialization() {
        // Test deserialization of ComponentType
        let json = json!({
            "name": "Connector",
            "instance": "Main",
            "customData": {
                "vendorId": "VendorX"
            },
            "evse": {
                "id": 1,
                "connectorId": 2
            }
        });

        let component: ComponentType = serde_json::from_value(json).unwrap();

        assert_eq!(component.name(), "Connector");
        assert_eq!(component.instance(), Some("Main"));
        assert_eq!(component.custom_data().unwrap().vendor_id(), "VendorX");
        assert_eq!(component.evse().unwrap().id(), 1);
        assert_eq!(component.evse().unwrap().connector_id(), Some(2));
    }

    #[test]
    fn test_component_validation() {
        // Test validation of ComponentType
        let component = ComponentType::new("Connector".to_string());

        // This should validate successfully
        assert!(component.validate().is_ok());

        // Test with a name that's too long (>50 chars)
        let long_name = "A".repeat(51);
        let invalid_component = ComponentType::new(long_name);

        // This should fail validation
        assert!(invalid_component.validate().is_err());

        // Test with an instance that's too long (>50 chars)
        let long_instance = "A".repeat(51);
        let invalid_component =
            ComponentType::new("Connector".to_string()).with_instance(long_instance);

        // This should fail validation
        assert!(invalid_component.validate().is_err());
    }

    #[test]
    fn test_component_with_custom_data_properties() {
        // Test with custom data that has additional properties
        let mut custom_data = CustomDataType::new("VendorX".to_string());
        custom_data.set_property("version".to_string(), json!("1.0"));
        custom_data.set_property("features".to_string(), json!(["feature1", "feature2"]));

        let component = ComponentType::new("Connector".to_string()).with_custom_data(custom_data);

        let serialized = serde_json::to_value(&component).unwrap();

        // Check that the custom properties are included in the serialized JSON
        assert_eq!(serialized["customData"]["vendorId"], "VendorX");
        assert_eq!(serialized["customData"]["version"], "1.0");
        assert_eq!(
            serialized["customData"]["features"],
            json!(["feature1", "feature2"])
        );
    }

    #[test]
    fn test_component_optional_fields() {
        // Test that optional fields are skipped when serializing if None
        let component = ComponentType::new("Connector".to_string());

        let serialized = serde_json::to_value(&component).unwrap();

        // Check that optional fields are not included
        let obj = serialized.as_object().unwrap();
        assert_eq!(obj.len(), 1); // Should have only name
        assert!(obj.contains_key("name"));
        assert!(!obj.contains_key("instance"));
        assert!(!obj.contains_key("customData"));
        assert!(!obj.contains_key("evse"));
    }

    #[test]
    fn test_component_with_evse_custom_data() {
        // Test with EVSE that has custom data
        let evse_custom_data = CustomDataType::new("EVSEVendor".to_string());
        let evse = EVSEType::new(1)
            .with_connector_id(2)
            .with_custom_data(evse_custom_data);

        let component = ComponentType::new("Connector".to_string()).with_evse(evse);

        let serialized = serde_json::to_value(&component).unwrap();

        // Check that the EVSE custom data is included
        assert_eq!(serialized["evse"]["customData"]["vendorId"], "EVSEVendor");
    }

    #[test]
    fn test_component_equality() {
        // Test equality of ComponentType instances
        let component1 =
            ComponentType::new("Connector".to_string()).with_instance("Main".to_string());

        let component2 =
            ComponentType::new("Connector".to_string()).with_instance("Main".to_string());

        let component3 = ComponentType::new("Meter".to_string()).with_instance("Main".to_string());

        assert_eq!(component1, component2);
        assert_ne!(component1, component3);
    }

    #[test]
    fn test_component_clone() {
        // Test cloning of ComponentType
        let original =
            ComponentType::new("Connector".to_string()).with_instance("Main".to_string());

        let cloned = original.clone();

        assert_eq!(original, cloned);

        // Modify the clone and verify the original is unchanged
        let mut modified = cloned;
        modified.set_name("Modified".to_string());

        assert_ne!(original, modified);
        assert_eq!(original.name(), "Connector");
        assert_eq!(modified.name(), "Modified");
    }
}
