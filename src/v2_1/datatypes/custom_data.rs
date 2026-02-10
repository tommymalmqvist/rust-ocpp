use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

/// This class does not get 'AdditionalProperties = false' in the schema generation,
/// so it can be extended with arbitrary JSON properties to allow adding custom data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomDataType {
    /// Vendor-specific identifier
    #[validate(length(max = 255))]
    pub vendor_id: String,

    /// Additional vendor-specific properties
    #[serde(flatten)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    pub additional_properties: HashMap<String, Value>,
}

impl CustomDataType {
    /// Creates a new `CustomDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `vendor_id` - Vendor-specific identifier
    ///
    /// # Returns
    ///
    /// A new instance of `CustomDataType` with empty additional properties
    pub fn new(vendor_id: String) -> Self {
        Self {
            vendor_id,
            additional_properties: HashMap::new(),
        }
    }

    /// Adds a custom property to the additional properties.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the custom property
    /// * `value` - The value for the custom property
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_property(mut self, key: String, value: Value) -> Self {
        self.additional_properties.insert(key, value);
        self
    }

    /// Gets the vendor ID.
    ///
    /// # Returns
    ///
    /// A reference to the vendor-specific identifier
    pub fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    /// Sets the vendor ID.
    ///
    /// # Arguments
    ///
    /// * `vendor_id` - Vendor-specific identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_vendor_id(&mut self, vendor_id: String) -> &mut Self {
        self.vendor_id = vendor_id;
        self
    }

    /// Gets the additional properties.
    ///
    /// # Returns
    ///
    /// A reference to the additional vendor-specific properties
    pub fn additional_properties(&self) -> &HashMap<String, Value> {
        &self.additional_properties
    }

    /// Sets a custom property in the additional properties.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the custom property
    /// * `value` - The value for the custom property
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_property(&mut self, key: String, value: Value) -> &mut Self {
        self.additional_properties.insert(key, value);
        self
    }

    /// Removes a custom property from the additional properties.
    ///
    /// # Arguments
    ///
    /// * `key` - The key for the custom property to remove
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn remove_property(&mut self, key: &str) -> &mut Self {
        self.additional_properties.remove(key);
        self
    }

    /// Clears all additional properties.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn clear_properties(&mut self) -> &mut Self {
        self.additional_properties.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        assert_eq!(custom_data.vendor_id(), "VendorX");
        assert!(custom_data.additional_properties().is_empty());
    }

    #[test]
    fn test_with_property() {
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"))
            .with_property("features".to_string(), json!(["feature1", "feature2"]));

        assert_eq!(custom_data.vendor_id(), "VendorX");
        assert_eq!(custom_data.additional_properties().len(), 2);
        assert_eq!(
            custom_data.additional_properties().get("version"),
            Some(&json!("1.0"))
        );
        assert_eq!(
            custom_data.additional_properties().get("features"),
            Some(&json!(["feature1", "feature2"]))
        );
    }

    #[test]
    fn test_setter_methods() {
        let mut custom_data = CustomDataType::new("VendorX".to_string());

        custom_data
            .set_vendor_id("VendorY".to_string())
            .set_property("version".to_string(), json!("2.0"))
            .set_property("enabled".to_string(), json!(true));

        assert_eq!(custom_data.vendor_id(), "VendorY");
        assert_eq!(custom_data.additional_properties().len(), 2);
        assert_eq!(
            custom_data.additional_properties().get("version"),
            Some(&json!("2.0"))
        );
        assert_eq!(
            custom_data.additional_properties().get("enabled"),
            Some(&json!(true))
        );

        // Test removing a property
        custom_data.remove_property("version");
        assert_eq!(custom_data.additional_properties().len(), 1);
        assert!(custom_data.additional_properties().get("version").is_none());
        assert_eq!(
            custom_data.additional_properties().get("enabled"),
            Some(&json!(true))
        );

        // Test clearing all properties
        custom_data.clear_properties();
        assert!(custom_data.additional_properties().is_empty());
    }
}
