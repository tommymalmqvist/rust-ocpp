use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Contains information about a specific firmware version.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FirmwareType {
    /// URL from which the firmware can be downloaded.
    #[validate(length(max = 2000))]
    pub location: String,

    /// Date and time at which the firmware shall be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_date_time: Option<DateTime<Utc>>,

    /// Date and time at which the firmware shall be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_date_time: Option<DateTime<Utc>>,

    /// Firmware version.
    #[validate(length(max = 800))]
    pub signature: String,

    /// MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 5500))]
    pub signing_certificate: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl FirmwareType {
    /// Creates a new `FirmwareType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `location` - URL from which the firmware can be downloaded
    /// * `signature` - Firmware version
    ///
    /// # Returns
    ///
    /// A new instance of `FirmwareType` with optional fields set to `None`
    pub fn new(location: String, signature: String) -> Self {
        Self {
            location,
            signature,
            retrieve_date_time: None,
            install_date_time: None,
            signing_certificate: None,
            custom_data: None,
        }
    }

    /// Sets the retrieve date and time.
    ///
    /// # Arguments
    ///
    /// * `retrieve_date_time` - Date and time at which the firmware shall be retrieved
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_retrieve_date_time(mut self, retrieve_date_time: DateTime<Utc>) -> Self {
        self.retrieve_date_time = Some(retrieve_date_time);
        self
    }

    /// Sets the install date and time.
    ///
    /// # Arguments
    ///
    /// * `install_date_time` - Date and time at which the firmware shall be installed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_install_date_time(mut self, install_date_time: DateTime<Utc>) -> Self {
        self.install_date_time = Some(install_date_time);
        self
    }

    /// Sets the signing certificate.
    ///
    /// # Arguments
    ///
    /// * `signing_certificate` - MD5 checksum over the entire firmware file
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_signing_certificate(mut self, signing_certificate: String) -> Self {
        self.signing_certificate = Some(signing_certificate);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this firmware
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the location.
    ///
    /// # Returns
    ///
    /// The URL from which the firmware can be downloaded
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - URL from which the firmware can be downloaded
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_location(&mut self, location: String) -> &mut Self {
        self.location = location;
        self
    }

    /// Gets the retrieve date and time.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which the firmware shall be retrieved
    pub fn retrieve_date_time(&self) -> Option<&DateTime<Utc>> {
        self.retrieve_date_time.as_ref()
    }

    /// Sets the retrieve date and time.
    ///
    /// # Arguments
    ///
    /// * `retrieve_date_time` - Date and time at which the firmware shall be retrieved, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_retrieve_date_time(
        &mut self,
        retrieve_date_time: Option<DateTime<Utc>>,
    ) -> &mut Self {
        self.retrieve_date_time = retrieve_date_time;
        self
    }

    /// Gets the install date and time.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which the firmware shall be installed
    pub fn install_date_time(&self) -> Option<&DateTime<Utc>> {
        self.install_date_time.as_ref()
    }

    /// Sets the install date and time.
    ///
    /// # Arguments
    ///
    /// * `install_date_time` - Date and time at which the firmware shall be installed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_install_date_time(&mut self, install_date_time: Option<DateTime<Utc>>) -> &mut Self {
        self.install_date_time = install_date_time;
        self
    }

    /// Gets the signature.
    ///
    /// # Returns
    ///
    /// The firmware version
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Sets the signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - Firmware version
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signature(&mut self, signature: String) -> &mut Self {
        self.signature = signature;
        self
    }

    /// Gets the signing certificate.
    ///
    /// # Returns
    ///
    /// An optional reference to the MD5 checksum over the entire firmware file
    pub fn signing_certificate(&self) -> Option<&str> {
        self.signing_certificate.as_deref()
    }

    /// Sets the signing certificate.
    ///
    /// # Arguments
    ///
    /// * `signing_certificate` - MD5 checksum over the entire firmware file, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signing_certificate(&mut self, signing_certificate: Option<String>) -> &mut Self {
        self.signing_certificate = signing_certificate;
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
    /// * `custom_data` - Custom data for this firmware, or None to clear
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
    fn test_new_firmware() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();

        let firmware = FirmwareType::new(location.clone(), signature.clone());

        assert_eq!(firmware.location(), location);
        assert_eq!(firmware.signature(), signature);
        assert_eq!(firmware.retrieve_date_time(), None);
        assert_eq!(firmware.install_date_time(), None);
        assert_eq!(firmware.signing_certificate(), None);
        assert_eq!(firmware.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let retrieve_date_time = Utc::now();
        let install_date_time = Utc::now();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let firmware = FirmwareType::new(location.clone(), signature.clone())
            .with_retrieve_date_time(retrieve_date_time.clone())
            .with_install_date_time(install_date_time.clone())
            .with_signing_certificate(signing_certificate.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(firmware.location(), location);
        assert_eq!(firmware.signature(), signature);
        assert_eq!(firmware.retrieve_date_time(), Some(&retrieve_date_time));
        assert_eq!(firmware.install_date_time(), Some(&install_date_time));
        assert_eq!(
            firmware.signing_certificate(),
            Some(signing_certificate.as_str())
        );
        assert_eq!(firmware.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let location1 = "https://example.com/firmware/v1.2.3".to_string();
        let signature1 = "1.2.3".to_string();
        let location2 = "https://example.com/firmware/v1.2.4".to_string();
        let signature2 = "1.2.4".to_string();
        let retrieve_date_time = Utc::now();
        let install_date_time = Utc::now();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut firmware = FirmwareType::new(location1.clone(), signature1.clone());

        firmware
            .set_location(location2.clone())
            .set_signature(signature2.clone())
            .set_retrieve_date_time(Some(retrieve_date_time.clone()))
            .set_install_date_time(Some(install_date_time.clone()))
            .set_signing_certificate(Some(signing_certificate.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(firmware.location(), location2);
        assert_eq!(firmware.signature(), signature2);
        assert_eq!(firmware.retrieve_date_time(), Some(&retrieve_date_time));
        assert_eq!(firmware.install_date_time(), Some(&install_date_time));
        assert_eq!(
            firmware.signing_certificate(),
            Some(signing_certificate.as_str())
        );
        assert_eq!(firmware.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        firmware
            .set_retrieve_date_time(None)
            .set_install_date_time(None)
            .set_signing_certificate(None)
            .set_custom_data(None);

        assert_eq!(firmware.retrieve_date_time(), None);
        assert_eq!(firmware.install_date_time(), None);
        assert_eq!(firmware.signing_certificate(), None);
        assert_eq!(firmware.custom_data(), None);
    }

    #[test]
    fn test_validation_basic() {
        // Valid firmware with minimum requirements
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let firmware = FirmwareType::new(location, signature);

        assert!(
            firmware.validate().is_ok(),
            "Valid firmware should pass validation"
        );

        // Valid firmware with all fields
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let retrieve_date_time = Utc::now();
        let install_date_time = Utc::now();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let firmware_with_all = FirmwareType::new(location, signature)
            .with_retrieve_date_time(retrieve_date_time)
            .with_install_date_time(install_date_time)
            .with_signing_certificate(signing_certificate)
            .with_custom_data(custom_data);

        assert!(
            firmware_with_all.validate().is_ok(),
            "Firmware with all fields should pass validation"
        );
    }

    #[test]
    fn test_validation_errors() {
        // Test with location that's too long (>2000 chars)
        let long_location = "https://example.com/".to_string() + &"a".repeat(2000);
        let signature = "1.2.3".to_string();

        let invalid_firmware = FirmwareType::new(long_location, signature);

        let validation_result = invalid_firmware.validate();
        assert!(
            validation_result.is_err(),
            "Firmware with too long location should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the location field for length validation
        assert!(
            field_errors.contains_key("location"),
            "Validation errors should contain location field"
        );
        let location_errors = &field_errors["location"];
        assert!(
            !location_errors.is_empty(),
            "location field should have validation errors"
        );
        assert_eq!(
            location_errors[0].code, "length",
            "location field should have a length error"
        );

        // Test with signature that's too long (>800 chars)
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let long_signature = "a".repeat(801);

        let invalid_firmware = FirmwareType::new(location, long_signature);

        let validation_result = invalid_firmware.validate();
        assert!(
            validation_result.is_err(),
            "Firmware with too long signature should fail validation"
        );

        // Test with signing_certificate that's too long (>5500 chars)
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let long_cert = "a".repeat(5501);

        let invalid_firmware =
            FirmwareType::new(location, signature).with_signing_certificate(long_cert);

        let validation_result = invalid_firmware.validate();
        assert!(
            validation_result.is_err(),
            "Firmware with too long signing_certificate should fail validation"
        );
    }

    #[test]
    fn test_nested_validation() {
        // Test nested validation for CustomDataType
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();

        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let firmware = FirmwareType::new(location, signature).with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = firmware.validate();
        assert!(
            validation_result.is_err(),
            "Firmware with invalid custom_data should fail validation"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let retrieve_date_time = Utc::now();
        let install_date_time = Utc::now();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let firmware = FirmwareType::new(location, signature)
            .with_retrieve_date_time(retrieve_date_time)
            .with_install_date_time(install_date_time)
            .with_signing_certificate(signing_certificate)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&firmware).unwrap();

        // Deserialize back
        let deserialized: FirmwareType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(firmware, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_json_structure() {
        let location = "https://example.com/firmware/v1.2.3".to_string();
        let signature = "1.2.3".to_string();
        let retrieve_date_time = Utc::now();
        let install_date_time = Utc::now();
        let signing_certificate = "0123456789abcdef0123456789abcdef".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let firmware = FirmwareType::new(location, signature)
            .with_retrieve_date_time(retrieve_date_time)
            .with_install_date_time(install_date_time)
            .with_signing_certificate(signing_certificate)
            .with_custom_data(custom_data);

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&firmware).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        assert!(json_value.get("location").is_some());
        assert!(json_value.get("signature").is_some());
        assert!(json_value.get("retrieveDateTime").is_some());
        assert!(json_value.get("installDateTime").is_some());
        assert!(json_value.get("signingCertificate").is_some());
        assert!(json_value.get("customData").is_some());

        // Check field values
        assert_eq!(
            json_value["location"],
            "https://example.com/firmware/v1.2.3"
        );
        assert_eq!(json_value["signature"], "1.2.3");
        assert_eq!(
            json_value["signingCertificate"],
            "0123456789abcdef0123456789abcdef"
        );
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization_from_json() {
        // Create a JSON string representing a FirmwareType
        let json_str = r#"{
            "location": "https://example.com/firmware/v1.2.3",
            "signature": "1.2.3",
            "retrieveDateTime": "2023-01-01T12:00:00Z",
            "installDateTime": "2023-01-02T12:00:00Z",
            "signingCertificate": "0123456789abcdef0123456789abcdef",
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        // Deserialize from JSON string
        let firmware: FirmwareType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(firmware.location(), "https://example.com/firmware/v1.2.3");
        assert_eq!(firmware.signature(), "1.2.3");
        assert!(firmware.retrieve_date_time().is_some());
        assert!(firmware.install_date_time().is_some());
        assert_eq!(
            firmware.signing_certificate(),
            Some("0123456789abcdef0123456789abcdef")
        );
        assert_eq!(firmware.custom_data().unwrap().vendor_id(), "TestVendor");

        // Check additional properties in custom data
        let custom_data = firmware.custom_data().unwrap();
        assert_eq!(
            custom_data.additional_properties()["extraInfo"],
            json!("Something")
        );
    }

    #[test]
    fn test_partial_json() {
        // Test with only required fields
        let json_str = r#"{
            "location": "https://example.com/firmware/v1.2.3",
            "signature": "1.2.3"
        }"#;

        // Deserialize from JSON string
        let firmware: FirmwareType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(firmware.location(), "https://example.com/firmware/v1.2.3");
        assert_eq!(firmware.signature(), "1.2.3");
        assert_eq!(firmware.retrieve_date_time(), None);
        assert_eq!(firmware.install_date_time(), None);
        assert_eq!(firmware.signing_certificate(), None);
        assert_eq!(firmware.custom_data(), None);
    }

    #[test]
    fn test_invalid_json() {
        // Test with missing required fields
        let json_str = r#"{
            "retrieveDateTime": "2023-01-01T12:00:00Z",
            "customData": {
                "vendorId": "TestVendor"
            }
        }"#;

        // Deserialize from JSON string should fail
        let result = serde_json::from_str::<FirmwareType>(json_str);
        assert!(
            result.is_err(),
            "Deserialization should fail with missing required fields"
        );
    }
}
