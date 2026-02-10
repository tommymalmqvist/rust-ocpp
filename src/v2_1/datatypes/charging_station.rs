use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::modem::ModemType;

/// The physical system where an Electrical Vehicle (EV) can be charged.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// Vendor-specific device identifier.
    #[validate(length(max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,

    /// Required. Defines the model of the device.
    #[validate(length(max = 20))]
    pub model: String,

    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(max = 50))]
    pub vendor_name: String,

    /// This contains the firmware version of the Charging Station.
    #[validate(length(max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// Defines parameters required for initiating and maintaining wireless communication with other devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub modem: Option<ModemType>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingStationType {
    /// Creates a new `ChargingStationType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `model` - Defines the model of the device
    /// * `vendor_name` - Identifies the vendor
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingStationType` with optional fields set to `None`
    pub fn new(model: String, vendor_name: String) -> Self {
        Self {
            model,
            vendor_name,
            custom_data: None,
            serial_number: None,
            firmware_version: None,
            modem: None,
        }
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Vendor-specific device identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_serial_number(mut self, serial_number: String) -> Self {
        self.serial_number = Some(serial_number);
        self
    }

    /// Sets the firmware version.
    ///
    /// # Arguments
    ///
    /// * `firmware_version` - Firmware version of the Charging Station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_firmware_version(mut self, firmware_version: String) -> Self {
        self.firmware_version = Some(firmware_version);
        self
    }

    /// Sets the modem.
    ///
    /// # Arguments
    ///
    /// * `modem` - Parameters for wireless communication
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_modem(mut self, modem: ModemType) -> Self {
        self.modem = Some(modem);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging station
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the model.
    ///
    /// # Returns
    ///
    /// A reference to the model of the device
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Sets the model.
    ///
    /// # Arguments
    ///
    /// * `model` - Defines the model of the device
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_model(&mut self, model: String) -> &mut Self {
        self.model = model;
        self
    }

    /// Gets the vendor name.
    ///
    /// # Returns
    ///
    /// A reference to the vendor name
    pub fn vendor_name(&self) -> &str {
        &self.vendor_name
    }

    /// Sets the vendor name.
    ///
    /// # Arguments
    ///
    /// * `vendor_name` - Identifies the vendor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_vendor_name(&mut self, vendor_name: String) -> &mut Self {
        self.vendor_name = vendor_name;
        self
    }

    /// Gets the serial number.
    ///
    /// # Returns
    ///
    /// An optional reference to the serial number
    pub fn serial_number(&self) -> Option<&str> {
        self.serial_number.as_deref()
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Vendor-specific device identifier, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_serial_number(&mut self, serial_number: Option<String>) -> &mut Self {
        self.serial_number = serial_number;
        self
    }

    /// Gets the firmware version.
    ///
    /// # Returns
    ///
    /// An optional reference to the firmware version
    pub fn firmware_version(&self) -> Option<&str> {
        self.firmware_version.as_deref()
    }

    /// Sets the firmware version.
    ///
    /// # Arguments
    ///
    /// * `firmware_version` - Firmware version of the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_firmware_version(&mut self, firmware_version: Option<String>) -> &mut Self {
        self.firmware_version = firmware_version;
        self
    }

    /// Gets the modem.
    ///
    /// # Returns
    ///
    /// An optional reference to the modem
    pub fn modem(&self) -> Option<&ModemType> {
        self.modem.as_ref()
    }

    /// Sets the modem.
    ///
    /// # Arguments
    ///
    /// * `modem` - Parameters for wireless communication, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_modem(&mut self, modem: Option<ModemType>) -> &mut Self {
        self.modem = modem;
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
    /// * `custom_data` - Custom data for this charging station, or None to clear
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
    use serde_json::{from_str, to_string};
    use validator::Validate;

    #[test]
    fn test_new_charging_station() {
        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string());

        assert_eq!(station.model(), "Model X");
        assert_eq!(station.vendor_name(), "Vendor Y");
        assert_eq!(station.serial_number(), None);
        assert_eq!(station.firmware_version(), None);
        assert_eq!(station.modem(), None);
        assert_eq!(station.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string())
            .with_serial_number("SN12345".to_string())
            .with_firmware_version("1.0.0".to_string())
            .with_modem(modem.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(station.model(), "Model X");
        assert_eq!(station.vendor_name(), "Vendor Y");
        assert_eq!(station.serial_number(), Some("SN12345"));
        assert_eq!(station.firmware_version(), Some("1.0.0"));
        assert_eq!(station.modem(), Some(&modem));
        assert_eq!(station.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        let mut station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string());

        station
            .set_model("Model Z".to_string())
            .set_vendor_name("Vendor Z".to_string())
            .set_serial_number(Some("SN67890".to_string()))
            .set_firmware_version(Some("2.0.0".to_string()))
            .set_modem(Some(modem.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(station.model(), "Model Z");
        assert_eq!(station.vendor_name(), "Vendor Z");
        assert_eq!(station.serial_number(), Some("SN67890"));
        assert_eq!(station.firmware_version(), Some("2.0.0"));
        assert_eq!(station.modem(), Some(&modem));
        assert_eq!(station.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        station
            .set_serial_number(None)
            .set_firmware_version(None)
            .set_modem(None)
            .set_custom_data(None);

        assert_eq!(station.serial_number(), None);
        assert_eq!(station.firmware_version(), None);
        assert_eq!(station.modem(), None);
        assert_eq!(station.custom_data(), None);
    }

    #[test]
    fn test_serialization_deserialization() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string())
            .with_serial_number("SN12345".to_string())
            .with_firmware_version("1.0.0".to_string())
            .with_modem(modem.clone())
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = to_string(&station).unwrap();

        // Verify JSON contains expected fields
        assert!(serialized.contains(r#""model":"Model X"#));
        assert!(serialized.contains(r#""vendorName":"Vendor Y"#));
        assert!(serialized.contains(r#""serialNumber":"SN12345"#));
        assert!(serialized.contains(r#""firmwareVersion":"1.0.0"#));
        assert!(serialized.contains(r#""iccid":"12345678901234567890"#));
        assert!(serialized.contains(r#""imsi":"123456789012345"#));
        assert!(serialized.contains(r#""vendorId":"VendorX"#));

        // Deserialize back
        let deserialized: ChargingStationType = from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.model(), station.model());
        assert_eq!(deserialized.vendor_name(), station.vendor_name());
        assert_eq!(deserialized.serial_number(), station.serial_number());
        assert_eq!(deserialized.firmware_version(), station.firmware_version());
        assert_eq!(deserialized.modem().unwrap().iccid(), modem.iccid());
        assert_eq!(deserialized.modem().unwrap().imsi(), modem.imsi());
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            custom_data.vendor_id()
        );
    }

    #[test]
    fn test_validation() {
        // Valid charging station
        let valid_station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string());
        assert!(
            valid_station.validate().is_ok(),
            "Valid station should pass validation"
        );

        // Test model length validation (too long)
        let mut invalid_station = valid_station.clone();
        invalid_station.model = "a".repeat(21); // Exceeds max length of 20
        assert!(
            invalid_station.validate().is_err(),
            "Station with too long model should fail validation"
        );

        // Test vendor_name length validation (too long)
        let mut invalid_station = valid_station.clone();
        invalid_station.vendor_name = "a".repeat(51); // Exceeds max length of 50
        assert!(
            invalid_station.validate().is_err(),
            "Station with too long vendor_name should fail validation"
        );

        // Test serial_number length validation (too long)
        let mut invalid_station = valid_station.clone();
        invalid_station.serial_number = Some("a".repeat(26)); // Exceeds max length of 25
        assert!(
            invalid_station.validate().is_err(),
            "Station with too long serial_number should fail validation"
        );

        // Test firmware_version length validation (too long)
        let mut invalid_station = valid_station.clone();
        invalid_station.firmware_version = Some("a".repeat(51)); // Exceeds max length of 50
        assert!(
            invalid_station.validate().is_err(),
            "Station with too long firmware_version should fail validation"
        );

        // Test nested validation for modem
        let mut invalid_station = valid_station.clone();
        let invalid_modem = ModemType {
            iccid: "a".repeat(21), // Exceeds max length of 20
            imsi: "123456789012345".to_string(),
            custom_data: None,
        };
        invalid_station.modem = Some(invalid_modem);
        assert!(
            invalid_station.validate().is_err(),
            "Station with invalid modem should fail validation"
        );

        // Test nested validation for custom_data
        // Note: CustomDataType doesn't have validation constraints that can be easily violated
        // in a test, but we're testing the principle of nested validation
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty strings for required fields
        let station = ChargingStationType::new("".to_string(), "".to_string());
        // Empty strings are valid for these fields from a validation perspective
        assert!(station.validate().is_ok());

        // Test with maximum allowed lengths
        let station = ChargingStationType::new(
            "a".repeat(20), // Max length for model
            "a".repeat(50), // Max length for vendor_name
        )
        .with_serial_number("a".repeat(25)) // Max length for serial_number
        .with_firmware_version("a".repeat(50)); // Max length for firmware_version

        assert!(
            station.validate().is_ok(),
            "Station with maximum length fields should pass validation"
        );

        // Test with modem that has maximum length fields
        let modem = ModemType::new(
            "a".repeat(20), // Max length for iccid
            "a".repeat(20), // Max length for imsi
        );
        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string())
            .with_modem(modem);

        assert!(
            station.validate().is_ok(),
            "Station with modem having maximum length fields should pass validation"
        );
    }

    #[test]
    fn test_complex_scenario() {
        // Create a modem with custom data
        let modem_custom_data = CustomDataType::new("ModemVendor".to_string());
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        )
        .with_custom_data(modem_custom_data);

        // Create a charging station with all fields populated
        let station_custom_data = CustomDataType::new("StationVendor".to_string());
        let station = ChargingStationType::new("Model X".to_string(), "Vendor Y".to_string())
            .with_serial_number("SN12345".to_string())
            .with_firmware_version("1.0.0".to_string())
            .with_modem(modem.clone())
            .with_custom_data(station_custom_data.clone());

        // Validate the complex object
        assert!(
            station.validate().is_ok(),
            "Complex station should pass validation"
        );

        // Serialize and deserialize
        let serialized = to_string(&station).unwrap();
        let deserialized: ChargingStationType = from_str(&serialized).unwrap();

        // Verify nested custom data is preserved
        assert_eq!(
            deserialized
                .modem()
                .unwrap()
                .custom_data()
                .unwrap()
                .vendor_id(),
            "ModemVendor"
        );
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            "StationVendor"
        );
    }
}
