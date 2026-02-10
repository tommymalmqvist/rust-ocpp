use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Reference key to a component-variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableType {
    /// Name of the variable. Name should be taken from the list of standardized variable names whenever possible. Case Insensitive. strongly advised to use Camel Case.
    #[validate(length(max = 50))]
    pub name: String,

    /// Name of instance in case the variable exists as multiple instances. Case Insensitive. strongly advised to use Camel Case.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub instance: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VariableType {
    /// Creates a new `VariableType` with required name field.
    pub fn new(name: String) -> Self {
        Self {
            name,
            instance: None,
            custom_data: None,
        }
    }

    /// Creates a new `VariableType` with name and instance.
    pub fn new_with_instance(name: String, instance: String) -> Self {
        Self {
            name,
            instance: Some(instance),
            custom_data: None,
        }
    }

    /// Gets the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Gets the instance.
    pub fn instance(&self) -> Option<&str> {
        self.instance.as_deref()
    }

    /// Sets the instance.
    pub fn set_instance(&mut self, instance: Option<String>) -> &mut Self {
        self.instance = instance;
        self
    }

    /// Gets the custom data.
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the instance using the builder pattern.
    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    /// Sets the custom data using the builder pattern.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_variable() {
        let variable = VariableType::new("CurrentLimit".to_string());

        assert_eq!(variable.name(), "CurrentLimit");
        assert_eq!(variable.instance(), None);
        assert_eq!(variable.custom_data(), None);
    }

    #[test]
    fn test_new_with_instance() {
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());

        assert_eq!(variable.name(), "CurrentLimit");
        assert_eq!(variable.instance(), Some("Main"));
        assert_eq!(variable.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let variable = VariableType::new("CurrentLimit".to_string())
            .with_instance("Main".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(variable.name(), "CurrentLimit");
        assert_eq!(variable.instance(), Some("Main"));
        assert_eq!(variable.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut variable = VariableType::new("CurrentLimit".to_string());

        variable
            .set_name("VoltageLimit".to_string())
            .set_instance(Some("Secondary".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(variable.name(), "VoltageLimit");
        assert_eq!(variable.instance(), Some("Secondary"));
        assert_eq!(variable.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        variable.set_instance(None).set_custom_data(None);

        assert_eq!(variable.instance(), None);
        assert_eq!(variable.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let variable = VariableType::new("CurrentLimit".to_string())
            .with_instance("Main".to_string())
            .with_custom_data(custom_data);

        let json = serde_json::to_string(&variable).unwrap();
        let deserialized: VariableType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, variable);
    }
}
