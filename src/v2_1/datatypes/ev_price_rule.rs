use super::custom_data::CustomDataType;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Price rule for a power range.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPriceRuleType {
    /// Energy fee in the currency specified in EVAbsolutePriceSchedule.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub energy_fee: Decimal,

    /// Start of the power range in Watts (W).
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub power_range_start: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EVPriceRuleType {
    /// Creates a new `EVPriceRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee in the currency specified in EVAbsolutePriceSchedule
    /// * `power_range_start` - Start of the power range in Watts (W)
    ///
    /// # Returns
    ///
    /// A new instance of `EVPriceRuleType` with optional fields set to `None`
    pub fn new(energy_fee: f64, power_range_start: f64) -> Self {
        Self {
            energy_fee: Decimal::try_from(energy_fee).unwrap_or_default(),
            power_range_start: Decimal::try_from(power_range_start).unwrap_or_default(),
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the energy fee.
    ///
    /// # Returns
    ///
    /// The energy fee in the currency specified in EVAbsolutePriceSchedule
    pub fn energy_fee(&self) -> &Decimal {
        &self.energy_fee
    }

    /// Gets the energy fee as f64.
    ///
    /// # Returns
    ///
    /// The energy fee as an f64, or 0.0 if conversion fails
    pub fn energy_fee_as_f64(&self) -> f64 {
        self.energy_fee.to_f64().unwrap_or(0.0)
    }

    /// Sets the energy fee.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee in the currency specified in EVAbsolutePriceSchedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_fee(&mut self, energy_fee: f64) -> &mut Self {
        self.energy_fee = Decimal::try_from(energy_fee).unwrap_or_default();
        self
    }

    /// Sets the energy fee from a Decimal.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee in the currency specified in EVAbsolutePriceSchedule as a Decimal
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_fee_decimal(&mut self, energy_fee: Decimal) -> &mut Self {
        self.energy_fee = energy_fee;
        self
    }

    /// Gets the power range start.
    ///
    /// # Returns
    ///
    /// The start of the power range in Watts (W)
    pub fn power_range_start(&self) -> &Decimal {
        &self.power_range_start
    }

    /// Gets the power range start as f64.
    ///
    /// # Returns
    ///
    /// The power range start as an f64, or 0.0 if conversion fails
    pub fn power_range_start_as_f64(&self) -> f64 {
        self.power_range_start.to_f64().unwrap_or(0.0)
    }

    /// Sets the power range start.
    ///
    /// # Arguments
    ///
    /// * `power_range_start` - Start of the power range in Watts (W)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_range_start(&mut self, power_range_start: f64) -> &mut Self {
        self.power_range_start = Decimal::try_from(power_range_start).unwrap_or_default();
        self
    }

    /// Sets the power range start from a Decimal.
    ///
    /// # Arguments
    ///
    /// * `power_range_start` - Start of the power range in Watts (W) as a Decimal
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_range_start_decimal(&mut self, power_range_start: Decimal) -> &mut Self {
        self.power_range_start = power_range_start;
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
    /// * `custom_data` - Custom data for this price rule, or None to clear
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
    use rust_decimal_macros::dec;
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_new_ev_price_rule() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;
        let expected_energy_fee = Decimal::try_from(energy_fee).unwrap();
        let expected_power_range_start = Decimal::try_from(power_range_start).unwrap();

        let price_rule = EVPriceRuleType::new(energy_fee, power_range_start);

        assert_eq!(price_rule.energy_fee(), &expected_energy_fee);
        assert_eq!(price_rule.power_range_start(), &expected_power_range_start);
        assert_eq!(price_rule.energy_fee_as_f64(), energy_fee);
        assert_eq!(price_rule.power_range_start_as_f64(), power_range_start);
        assert_eq!(price_rule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;
        let expected_energy_fee = Decimal::try_from(energy_fee).unwrap();
        let expected_power_range_start = Decimal::try_from(power_range_start).unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let price_rule = EVPriceRuleType::new(energy_fee, power_range_start)
            .with_custom_data(custom_data.clone());

        assert_eq!(price_rule.energy_fee(), &expected_energy_fee);
        assert_eq!(price_rule.power_range_start(), &expected_power_range_start);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_fee1 = 0.25;
        let power_range_start1 = 5000.0;
        let energy_fee2 = 0.30;
        let power_range_start2 = 7500.0;
        let expected_energy_fee2 = Decimal::try_from(energy_fee2).unwrap();
        let expected_power_range_start2 = Decimal::try_from(power_range_start2).unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut price_rule = EVPriceRuleType::new(energy_fee1, power_range_start1);

        price_rule
            .set_energy_fee(energy_fee2)
            .set_power_range_start(power_range_start2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(price_rule.energy_fee(), &expected_energy_fee2);
        assert_eq!(price_rule.power_range_start(), &expected_power_range_start2);
        assert_eq!(price_rule.energy_fee_as_f64(), energy_fee2);
        assert_eq!(price_rule.power_range_start_as_f64(), power_range_start2);
        assert_eq!(price_rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        price_rule.set_custom_data(None);
        assert_eq!(price_rule.custom_data(), None);
    }

    #[test]
    fn test_decimal_setters() {
        let energy_fee_decimal = dec!(0.25);
        let power_range_start_decimal = dec!(5000.0);

        let mut price_rule = EVPriceRuleType::new(0.0, 0.0);
        price_rule
            .set_energy_fee_decimal(energy_fee_decimal)
            .set_power_range_start_decimal(power_range_start_decimal);

        assert_eq!(price_rule.energy_fee(), &energy_fee_decimal);
        assert_eq!(price_rule.power_range_start(), &power_range_start_decimal);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero values
        let zero_energy_fee = EVPriceRuleType::new(0.0, 0.0);
        assert_eq!(zero_energy_fee.energy_fee_as_f64(), 0.0);
        assert_eq!(zero_energy_fee.power_range_start_as_f64(), 0.0);

        // Test with negative values (for discharging scenarios)
        let negative_power = EVPriceRuleType::new(0.25, -5000.0);
        assert_eq!(negative_power.energy_fee_as_f64(), 0.25);
        assert_eq!(negative_power.power_range_start_as_f64(), -5000.0);

        // Test with negative energy fee (could represent payment to EV owner for discharging)
        let negative_fee = EVPriceRuleType::new(-0.15, -5000.0);
        assert_eq!(negative_fee.energy_fee_as_f64(), -0.15);
        assert_eq!(negative_fee.power_range_start_as_f64(), -5000.0);

        // Test with very large values
        let large_values = EVPriceRuleType::new(999999.99, 1000000.0);
        assert_eq!(large_values.energy_fee_as_f64(), 999999.99);
        assert_eq!(large_values.power_range_start_as_f64(), 1000000.0);

        // Test with very small values
        let small_values = EVPriceRuleType::new(0.0001, 0.1);
        assert_eq!(small_values.energy_fee_as_f64(), 0.0001);
        assert_eq!(small_values.power_range_start_as_f64(), 0.1);
    }

    #[test]
    fn test_validation() {
        // Basic validation - should pass
        let price_rule = EVPriceRuleType::new(0.25, 5000.0);
        assert!(
            price_rule.validate().is_ok(),
            "Valid price rule should pass validation"
        );

        // With custom data - should pass
        let custom_data = CustomDataType::new("VendorX".to_string());
        let price_rule_with_custom =
            EVPriceRuleType::new(0.25, 5000.0).with_custom_data(custom_data);
        assert!(
            price_rule_with_custom.validate().is_ok(),
            "Price rule with valid custom data should pass validation"
        );

        // With invalid custom data (vendor_id too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);
        let price_rule_invalid_custom =
            EVPriceRuleType::new(0.25, 5000.0).with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = price_rule_invalid_custom.validate();
        assert!(
            validation_result.is_err(),
            "Price rule with invalid custom data should fail validation"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let price_rule =
            EVPriceRuleType::new(energy_fee, power_range_start).with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&price_rule).unwrap();

        // Deserialize back
        let deserialized: EVPriceRuleType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(price_rule, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_json_structure() {
        let energy_fee = 0.25;
        let power_range_start = 5000.0;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let price_rule =
            EVPriceRuleType::new(energy_fee, power_range_start).with_custom_data(custom_data);

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&price_rule).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        assert!(json_value.get("energyFee").is_some());
        assert!(json_value.get("powerRangeStart").is_some());
        assert!(json_value.get("customData").is_some());

        // Check field values
        assert_eq!(json_value["energyFee"], 0.25);
        assert_eq!(json_value["powerRangeStart"], 5000.0);
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization_from_json() {
        // Create a JSON string representing an EVPriceRuleType
        let json_str = r#"{
            "energyFee": 0.25,
            "powerRangeStart": 5000.0,
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        // Deserialize from JSON string
        let price_rule: EVPriceRuleType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(price_rule.energy_fee_as_f64(), 0.25);
        assert_eq!(price_rule.power_range_start_as_f64(), 5000.0);
        assert_eq!(price_rule.custom_data().unwrap().vendor_id(), "TestVendor");

        // Check additional properties in custom data
        let custom_data = price_rule.custom_data().unwrap();
        assert_eq!(
            custom_data.additional_properties()["extraInfo"],
            json!("Something")
        );
    }

    #[test]
    fn test_partial_json() {
        // Test with missing optional fields
        let json_str = r#"{
            "energyFee": 0.25,
            "powerRangeStart": 5000.0
        }"#;

        // Deserialize from JSON string
        let price_rule: EVPriceRuleType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(price_rule.energy_fee_as_f64(), 0.25);
        assert_eq!(price_rule.power_range_start_as_f64(), 5000.0);
        assert_eq!(price_rule.custom_data(), None);
    }

    #[test]
    fn test_invalid_json() {
        // Test with missing required fields
        let json_str = r#"{
            "customData": {
                "vendorId": "TestVendor"
            }
        }"#;

        // Deserialize from JSON string should fail
        let result = serde_json::from_str::<EVPriceRuleType>(json_str);
        assert!(
            result.is_err(),
            "Deserialization should fail with missing required fields"
        );
    }
}
