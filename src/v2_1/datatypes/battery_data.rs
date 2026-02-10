use super::custom_data::CustomDataType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// Validates if a Decimal value is within the specified range
///
/// # Arguments
///
/// * `value` - The Decimal value to validate
///
/// # Returns
///
/// Returns Ok(()) if the value is between 0 and 100 (inclusive), otherwise returns Err
pub fn validate_decimal_range(value: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::ZERO;
    let max = Decimal::new(100, 0); // 100.0

    if *value < min || *value > max {
        return Err(ValidationError::new("decimal_range"));
    }

    Ok(())
}

/// Contains EV battery parameters.
///
/// This type represents battery data for an electric vehicle, including state of charge (SoC)
/// at the start and end of charging, battery capacity, and rechargeable energy capacity.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatteryDataType {
    ///Required. Slot number where battery is inserted or removed
    #[validate(range(min = 0))]
    pub evse_id: i32,

    ///Required. Serial number of battery
    #[validate(length(max = 50))]
    pub serial_number: String,

    ///Required. State of charge
    #[validate(custom(function = "validate_decimal_range"))]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub so_c: Decimal,

    ///Required. State of health
    #[validate(custom(function = "validate_decimal_range"))]
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub so_h: Decimal,

    ///Optional. Production date of battery
    pub production_date: DateTime<Utc>,

    ///Optional. Vendor-specific info from battery in undefined format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub vendor_info: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl BatteryDataType {
    /// Creates a new `BatteryDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - Slot number where battery is inserted or removed
    /// * `serial_number` - Serial number of battery
    /// * `so_c` - State of charge (0-100%)
    /// * `so_h` - State of health (0-100%)
    /// * `production_date` - Production date of battery
    ///
    /// # Returns
    ///
    /// A new instance of `BatteryDataType` with optional fields set to `None`
    pub fn new(
        evse_id: i32,
        serial_number: String,
        so_c: Decimal,
        so_h: Decimal,
        production_date: DateTime<Utc>,
    ) -> Self {
        Self {
            evse_id,
            serial_number,
            so_c,
            so_h,
            production_date,
            vendor_info: None,
            custom_data: None,
        }
    }

    /// Sets the vendor info.
    ///
    /// # Arguments
    ///
    /// * `vendor_info` - Vendor-specific info from battery
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_vendor_info(mut self, vendor_info: String) -> Self {
        self.vendor_info = Some(vendor_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this battery data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EVSE ID.
    ///
    /// # Returns
    ///
    /// The slot number where battery is inserted or removed
    pub fn evse_id(&self) -> i32 {
        self.evse_id
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - Slot number where battery is inserted or removed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Gets the serial number.
    ///
    /// # Returns
    ///
    /// The serial number of the battery
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Serial number of battery
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_serial_number(&mut self, serial_number: String) -> &mut Self {
        self.serial_number = serial_number;
        self
    }

    /// Gets the state of charge.
    ///
    /// # Returns
    ///
    /// The state of charge as a percentage (0-100%)
    pub fn so_c(&self) -> Decimal {
        self.so_c
    }

    /// Sets the state of charge.
    ///
    /// # Arguments
    ///
    /// * `so_c` - State of charge (0-100%)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_so_c(&mut self, so_c: Decimal) -> &mut Self {
        self.so_c = so_c;
        self
    }

    /// Gets the state of health.
    ///
    /// # Returns
    ///
    /// The state of health as a percentage (0-100%)
    pub fn so_h(&self) -> Decimal {
        self.so_h
    }

    /// Sets the state of health.
    ///
    /// # Arguments
    ///
    /// * `so_h` - State of health (0-100%)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_so_h(&mut self, so_h: Decimal) -> &mut Self {
        self.so_h = so_h;
        self
    }

    /// Gets the production date.
    ///
    /// # Returns
    ///
    /// The production date of the battery
    pub fn production_date(&self) -> &DateTime<Utc> {
        &self.production_date
    }

    /// Sets the production date.
    ///
    /// # Arguments
    ///
    /// * `production_date` - Production date of battery
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_production_date(&mut self, production_date: DateTime<Utc>) -> &mut Self {
        self.production_date = production_date;
        self
    }

    /// Gets the vendor info.
    ///
    /// # Returns
    ///
    /// An optional reference to the vendor-specific info
    pub fn vendor_info(&self) -> Option<&String> {
        self.vendor_info.as_ref()
    }

    /// Sets the vendor info.
    ///
    /// # Arguments
    ///
    /// * `vendor_info` - Vendor-specific info from battery, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_vendor_info(&mut self, vendor_info: Option<String>) -> &mut Self {
        self.vendor_info = vendor_info;
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
    /// * `custom_data` - Custom data for this battery data, or None to clear
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
    fn test_new_battery_data() {
        let now = Utc::now();
        let so_c = Decimal::new(500, 1); // 50.0%
        let so_h = Decimal::new(750, 1); // 75.0%

        let battery_data = BatteryDataType::new(
            1,                       // evse_id
            "BAT123456".to_string(), // serial_number
            so_c,                    // so_c
            so_h,                    // so_h
            now,                     // production_date
        );

        assert_eq!(battery_data.evse_id(), 1);
        assert_eq!(battery_data.serial_number(), "BAT123456");
        assert_eq!(battery_data.so_c(), so_c);
        assert_eq!(battery_data.so_h(), so_h);
        assert_eq!(battery_data.production_date(), &now);
        assert_eq!(battery_data.vendor_info(), None);
        assert_eq!(battery_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let now = Utc::now();
        let so_c = Decimal::new(500, 1); // 50.0%
        let so_h = Decimal::new(750, 1); // 75.0%
        let custom_data = CustomDataType::new("VendorX".to_string());

        let battery_data = BatteryDataType::new(
            1,                       // evse_id
            "BAT123456".to_string(), // serial_number
            so_c,                    // so_c
            so_h,                    // so_h
            now,                     // production_date
        )
        .with_vendor_info("Vendor specific info".to_string())
        .with_custom_data(custom_data.clone());

        assert_eq!(battery_data.evse_id(), 1);
        assert_eq!(battery_data.serial_number(), "BAT123456");
        assert_eq!(battery_data.so_c(), so_c);
        assert_eq!(battery_data.so_h(), so_h);
        assert_eq!(battery_data.production_date(), &now);
        assert_eq!(
            battery_data.vendor_info(),
            Some(&"Vendor specific info".to_string())
        );
        assert_eq!(battery_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let now = Utc::now();
        let tomorrow = now + chrono::Duration::days(1);
        let so_c1 = Decimal::new(500, 1); // 50.0%
        let so_c2 = Decimal::new(800, 1); // 80.0%
        let so_h1 = Decimal::new(750, 1); // 75.0%
        let so_h2 = Decimal::new(850, 1); // 85.0%
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut battery_data = BatteryDataType::new(
            1,                       // evse_id
            "BAT123456".to_string(), // serial_number
            so_c1,                   // so_c
            so_h1,                   // so_h
            now,                     // production_date
        );

        battery_data
            .set_evse_id(2)
            .set_serial_number("BAT654321".to_string())
            .set_so_c(so_c2)
            .set_so_h(so_h2)
            .set_production_date(tomorrow)
            .set_vendor_info(Some("Updated vendor info".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(battery_data.evse_id(), 2);
        assert_eq!(battery_data.serial_number(), "BAT654321");
        assert_eq!(battery_data.so_c(), so_c2);
        assert_eq!(battery_data.so_h(), so_h2);
        assert_eq!(battery_data.production_date(), &tomorrow);
        assert_eq!(
            battery_data.vendor_info(),
            Some(&"Updated vendor info".to_string())
        );
        assert_eq!(battery_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        battery_data.set_vendor_info(None).set_custom_data(None);

        assert_eq!(battery_data.vendor_info(), None);
        assert_eq!(battery_data.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        use validator::Validate;

        let now = Utc::now();
        let so_c = Decimal::new(500, 1); // 50.0%
        let so_h = Decimal::new(750, 1); // 75.0%

        // 1. Valid battery data - should pass validation
        let valid_battery_data = BatteryDataType::new(
            1,                       // evse_id (valid: >= 0)
            "BAT123456".to_string(), // serial_number (valid: <= 50 chars)
            so_c,                    // so_c (valid: 0-100%)
            so_h,                    // so_h (valid: 0-100%)
            now,                     // production_date
        );

        assert!(
            valid_battery_data.validate().is_ok(),
            "Valid battery data should pass validation"
        );

        // 2. Test evse_id validation (negative value)
        let mut invalid_evse_id_battery = valid_battery_data.clone();
        invalid_evse_id_battery.evse_id = -1;

        let validation_result = invalid_evse_id_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with negative evse_id should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("evse_id"),
            "Error should mention evse_id: {}",
            error
        );

        // 3. Test serial_number validation (too long)
        let mut invalid_serial_battery = valid_battery_data.clone();
        invalid_serial_battery.serial_number = "A".repeat(51); // 51 characters, exceeds max of 50

        let validation_result = invalid_serial_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with too long serial_number should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("serial_number"),
            "Error should mention serial_number: {}",
            error
        );

        // 4. Test so_c validation (too high)
        let mut invalid_soc_battery = valid_battery_data.clone();
        invalid_soc_battery.so_c = Decimal::new(1010, 1); // 101.0%, exceeds max of 100.0%

        let validation_result = invalid_soc_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with too high so_c should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("so_c"),
            "Error should mention so_c: {}",
            error
        );

        // 5. Test so_c validation (negative)
        let mut invalid_soc_battery = valid_battery_data.clone();
        invalid_soc_battery.so_c = Decimal::new(-10, 1); // -1.0%, below min of 0.0%

        let validation_result = invalid_soc_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with negative so_c should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("so_c"),
            "Error should mention so_c: {}",
            error
        );

        // 6. Test so_h validation (too high)
        let mut invalid_soh_battery = valid_battery_data.clone();
        invalid_soh_battery.so_h = Decimal::new(1010, 1); // 101.0%, exceeds max of 100.0%

        let validation_result = invalid_soh_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with too high so_h should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("so_h"),
            "Error should mention so_h: {}",
            error
        );

        // 7. Test so_h validation (negative)
        let mut invalid_soh_battery = valid_battery_data.clone();
        invalid_soh_battery.so_h = Decimal::new(-10, 1); // -1.0%, below min of 0.0%

        let validation_result = invalid_soh_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with negative so_h should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("so_h"),
            "Error should mention so_h: {}",
            error
        );

        // 8. Test vendor_info validation (too long)
        let mut invalid_vendor_info_battery = valid_battery_data.clone();
        invalid_vendor_info_battery.vendor_info = Some("A".repeat(501)); // 501 characters, exceeds max of 500

        let validation_result = invalid_vendor_info_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with too long vendor_info should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("vendor_info"),
            "Error should mention vendor_info: {}",
            error
        );

        // 9. Test custom_data nested validation
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255

        let mut invalid_custom_data_battery = valid_battery_data.clone();
        invalid_custom_data_battery.custom_data = Some(invalid_custom_data);

        let validation_result = invalid_custom_data_battery.validate();
        assert!(
            validation_result.is_err(),
            "Battery data with invalid custom_data should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }
}
