use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{datatypes::CustomDataType, enumerations::CostDimensionEnumType};

/// Volume consumed of cost dimension.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostDimensionType {
    /// Type of cost dimension: energy, power, time, etc.
    #[serde(rename = "type")]
    pub type_: CostDimensionEnumType,

    /// Volume of the dimension consumed, measured according to the dimension type.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub volume: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CostDimensionType {
    /// Creates a new `CostDimensionType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `type` - Type of cost dimension: energy, power, time, etc
    /// * `volume` - Volume of the dimension consumed, measured according to the dimension type
    ///
    /// # Returns
    ///
    /// A new instance of `CostDimensionType` with optional fields set to `None`
    pub fn new(type_: CostDimensionEnumType, volume: f64) -> Self {
        Self {
            type_,
            volume: Decimal::try_from(volume).unwrap_or_default(),
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this cost dimension
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the type of cost dimension.
    ///
    /// # Returns
    ///
    /// The type of cost dimension
    pub fn r#type(&self) -> &CostDimensionEnumType {
        &self.type_
    }

    /// Sets the type of cost dimension.
    ///
    /// # Arguments
    ///
    /// * `type` - Type of cost dimension: energy, power, time, etc
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: CostDimensionEnumType) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the volume of the dimension consumed.
    ///
    /// # Returns
    ///
    /// The volume of the dimension consumed
    pub fn volume(&self) -> f64 {
        self.volume.try_into().unwrap_or_default()
    }

    /// Sets the volume of the dimension consumed.
    ///
    /// # Arguments
    ///
    /// * `volume` - Volume of the dimension consumed, measured according to the dimension type
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_volume(&mut self, volume: f64) -> &mut Self {
        self.volume = Decimal::try_from(volume).unwrap_or_default();
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
    /// * `custom_data` - Custom data for this cost dimension, or None to clear
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
    use validator::Validate;

    #[test]
    fn test_new_cost_dimension() {
        let dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5);

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::Energy);
        assert_eq!(dimension.volume(), 10.5);
        assert_eq!(dimension.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let dimension = CostDimensionType::new(CostDimensionEnumType::ChargingTime, 30.0)
            .with_custom_data(custom_data.clone());

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::ChargingTime);
        assert_eq!(dimension.volume(), 30.0);
        assert_eq!(dimension.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let mut dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5);

        dimension
            .set_type(CostDimensionEnumType::MaxPower)
            .set_volume(50.0)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(dimension.r#type(), &CostDimensionEnumType::MaxPower);
        assert_eq!(dimension.volume(), 50.0);
        assert_eq!(dimension.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        dimension.set_custom_data(None);
        assert_eq!(dimension.custom_data(), None);
    }

    #[test]
    fn test_validation_basic() {
        // Valid case
        let dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5);
        assert!(
            dimension.validate().is_ok(),
            "Valid cost dimension should pass validation"
        );

        // Test with different enum values
        let dimension_max_power = CostDimensionType::new(CostDimensionEnumType::MaxPower, 10.5);
        assert!(
            dimension_max_power.validate().is_ok(),
            "MaxPower cost dimension should pass validation"
        );

        let dimension_min_power = CostDimensionType::new(CostDimensionEnumType::MinPower, 10.5);
        assert!(
            dimension_min_power.validate().is_ok(),
            "MinPower cost dimension should pass validation"
        );

        let dimension_charging_time =
            CostDimensionType::new(CostDimensionEnumType::ChargingTime, 30.0);
        assert!(
            dimension_charging_time.validate().is_ok(),
            "ChargingTime cost dimension should pass validation"
        );
    }

    #[test]
    fn test_validation_edge_cases() {
        // Zero volume should be valid
        let zero_volume = CostDimensionType::new(CostDimensionEnumType::Energy, 0.0);
        assert!(
            zero_volume.validate().is_ok(),
            "Zero volume should be valid"
        );

        // Negative volume should be valid (might represent reverse energy flow)
        let negative_volume = CostDimensionType::new(CostDimensionEnumType::Energy, -10.5);
        assert!(
            negative_volume.validate().is_ok(),
            "Negative volume should be valid"
        );

        // Large volume should be valid
        let large_volume = CostDimensionType::new(CostDimensionEnumType::Energy, 1_000_000.0);
        assert!(
            large_volume.validate().is_ok(),
            "Large volume should be valid"
        );
    }

    #[test]
    fn test_custom_data_validation() {
        // Create custom data with invalid vendor_id (too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let dimension = CostDimensionType::new(CostDimensionEnumType::Energy, 10.5)
            .with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = dimension.validate();
        assert!(
            validation_result.is_err(),
            "Invalid custom_data should cause validation failure"
        );
    }
}
