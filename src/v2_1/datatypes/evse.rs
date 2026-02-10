use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// EVSE object with properties common to OCPP 2.0.1 and OCPP 2.1.0.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    /// Identified Object. MRID. Numeric ID of the EVSE within the Charging Station.
    #[validate(range(min = 0))]
    pub id: i32,

    /// An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub connector_id: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EVSEType {
    /// Creates a new `EVSEType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Numeric ID of the EVSE within the Charging Station
    ///
    /// # Returns
    ///
    /// A new instance of `EVSEType` with optional fields set to `None`
    pub fn new(id: i32) -> Self {
        Self {
            id,
            connector_id: None,
            custom_data: None,
        }
    }

    /// Sets the connector ID.
    ///
    /// # Arguments
    ///
    /// * `connector_id` - An id to designate a specific connector by connector index number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_connector_id(mut self, connector_id: i32) -> Self {
        self.connector_id = Some(connector_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this EVSE
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The numeric ID of the EVSE within the Charging Station
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Numeric ID of the EVSE within the Charging Station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the connector ID.
    ///
    /// # Returns
    ///
    /// An optional connector ID
    pub fn connector_id(&self) -> Option<i32> {
        self.connector_id
    }

    /// Sets the connector ID.
    ///
    /// # Arguments
    ///
    /// * `connector_id` - An id to designate a specific connector, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_connector_id(&mut self, connector_id: Option<i32>) -> &mut Self {
        self.connector_id = connector_id;
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
    /// * `custom_data` - Custom data for this EVSE, or None to clear
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
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_new_evse() {
        let id = 42;

        let evse = EVSEType::new(id);

        assert_eq!(evse.id(), id);
        assert_eq!(evse.connector_id(), None);
        assert_eq!(evse.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = 42;
        let connector_id = 1;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let evse = EVSEType::new(id)
            .with_connector_id(connector_id)
            .with_custom_data(custom_data.clone());

        assert_eq!(evse.id(), id);
        assert_eq!(evse.connector_id(), Some(connector_id));
        assert_eq!(evse.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = 42;
        let id2 = 43;
        let connector_id = 1;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut evse = EVSEType::new(id1);

        evse.set_id(id2)
            .set_connector_id(Some(connector_id))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(evse.id(), id2);
        assert_eq!(evse.connector_id(), Some(connector_id));
        assert_eq!(evse.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        evse.set_connector_id(None).set_custom_data(None);

        assert_eq!(evse.connector_id(), None);
        assert_eq!(evse.custom_data(), None);
    }

    #[test]
    fn test_validation_basic() {
        // Valid EVSE with minimum requirements
        let evse = EVSEType::new(1);
        assert!(evse.validate().is_ok(), "Valid EVSE should pass validation");

        // Valid EVSE with all fields
        let evse_with_all = EVSEType::new(1)
            .with_connector_id(2)
            .with_custom_data(CustomDataType::new("VendorX".to_string()));

        assert!(
            evse_with_all.validate().is_ok(),
            "EVSE with all fields should pass validation"
        );

        // Valid EVSE with zero ID (minimum allowed value)
        let evse_zero_id = EVSEType::new(0);
        assert!(
            evse_zero_id.validate().is_ok(),
            "EVSE with zero ID should pass validation"
        );

        // Valid EVSE with zero connector ID (minimum allowed value)
        let evse_zero_connector = EVSEType::new(1).with_connector_id(0);
        assert!(
            evse_zero_connector.validate().is_ok(),
            "EVSE with zero connector ID should pass validation"
        );
    }

    #[test]
    fn test_validation_errors() {
        // Invalid EVSE with negative ID
        let invalid_id = EVSEType {
            id: -1,
            connector_id: None,
            custom_data: None,
        };

        let validation_result = invalid_id.validate();
        assert!(
            validation_result.is_err(),
            "EVSE with negative ID should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the id field for range validation
        assert!(
            field_errors.contains_key("id"),
            "Validation errors should contain id field"
        );
        let id_errors = &field_errors["id"];
        assert!(
            !id_errors.is_empty(),
            "id field should have validation errors"
        );
        assert_eq!(
            id_errors[0].code, "range",
            "id field should have a range error"
        );

        // Invalid EVSE with negative connector ID
        let invalid_connector = EVSEType {
            id: 1,
            connector_id: Some(-1),
            custom_data: None,
        };

        let validation_result = invalid_connector.validate();
        assert!(
            validation_result.is_err(),
            "EVSE with negative connector ID should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the connector_id field for range validation
        assert!(
            field_errors.contains_key("connector_id"),
            "Validation errors should contain connector_id field"
        );
        let connector_errors = &field_errors["connector_id"];
        assert!(
            !connector_errors.is_empty(),
            "connector_id field should have validation errors"
        );
        assert_eq!(
            connector_errors[0].code, "range",
            "connector_id field should have a range error"
        );
    }

    #[test]
    fn test_nested_validation() {
        // Test nested validation for CustomDataType
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let evse = EVSEType::new(1).with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = evse.validate();
        assert!(
            validation_result.is_err(),
            "EVSE with invalid custom_data should fail validation"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let id = 42;
        let connector_id = 1;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let evse = EVSEType::new(id)
            .with_connector_id(connector_id)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&evse).unwrap();

        // Deserialize back
        let deserialized: EVSEType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(evse, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_json_structure() {
        let id = 42;
        let connector_id = 1;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let evse = EVSEType::new(id)
            .with_connector_id(connector_id)
            .with_custom_data(custom_data);

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&evse).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        assert!(json_value.get("id").is_some());
        assert!(json_value.get("connectorId").is_some());
        assert!(json_value.get("customData").is_some());

        // Check field values
        assert_eq!(json_value["id"], 42);
        assert_eq!(json_value["connectorId"], 1);
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization_from_json() {
        // Create a JSON string representing an EVSEType
        let json_str = r#"{
            "id": 42,
            "connectorId": 1,
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        // Deserialize from JSON string
        let evse: EVSEType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(evse.id(), 42);
        assert_eq!(evse.connector_id(), Some(1));
        assert_eq!(evse.custom_data().unwrap().vendor_id(), "TestVendor");

        // Check additional properties in custom data
        let custom_data = evse.custom_data().unwrap();
        assert_eq!(
            custom_data.additional_properties()["extraInfo"],
            json!("Something")
        );
    }

    #[test]
    fn test_partial_json() {
        // Test with missing optional fields
        let json_str = r#"{
            "id": 42
        }"#;

        // Deserialize from JSON string
        let evse: EVSEType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(evse.id(), 42);
        assert_eq!(evse.connector_id(), None);
        assert_eq!(evse.custom_data(), None);
    }

    #[test]
    fn test_invalid_json() {
        // Test with missing required fields
        let json_str = r#"{
            "connectorId": 1,
            "customData": {
                "vendorId": "TestVendor"
            }
        }"#;

        // Deserialize from JSON string should fail
        let result = serde_json::from_str::<EVSEType>(json_str);
        assert!(
            result.is_err(),
            "Deserialization should fail with missing required fields"
        );
    }

    #[test]
    fn test_large_values() {
        // Test with large ID values
        let large_id = i32::MAX;
        let large_connector_id = i32::MAX;

        let evse = EVSEType::new(large_id).with_connector_id(large_connector_id);

        assert_eq!(evse.id(), large_id);
        assert_eq!(evse.connector_id(), Some(large_connector_id));

        // Validate should pass
        assert!(
            evse.validate().is_ok(),
            "EVSE with large IDs should pass validation"
        );

        // Serialize and deserialize
        let serialized = serde_json::to_string(&evse).unwrap();
        let deserialized: EVSEType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.id(), large_id);
        assert_eq!(deserialized.connector_id(), Some(large_connector_id));
    }
}
