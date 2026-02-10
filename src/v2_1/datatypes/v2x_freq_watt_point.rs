use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// *(2.1)* A point of a frequency-watt curve.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XFreqWattPointType {
    /// Net frequency in Hz.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub frequency: Decimal,

    /// Power in W to charge (positive) or discharge (negative) at specified frequency.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub power: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl V2XFreqWattPointType {
    /// Creates a new `V2XFreqWattPointType` with the required fields.
    pub fn new(frequency: Decimal, power: Decimal) -> Self {
        Self {
            frequency,
            power,
            custom_data: None,
        }
    }

    /// Creates a new `V2XFreqWattPointType` from floating-point values.
    pub fn new_from_f64(frequency: f64, power: f64) -> Self {
        Self {
            frequency: Decimal::from_f64(frequency).unwrap_or_else(|| Decimal::new(0, 0)),
            power: Decimal::from_f64(power).unwrap_or_else(|| Decimal::new(0, 0)),
            custom_data: None,
        }
    }

    /// Gets the frequency in Hz.
    pub fn frequency(&self) -> Decimal {
        self.frequency
    }

    /// Sets the frequency in Hz.
    pub fn set_frequency(&mut self, frequency: Decimal) -> &mut Self {
        self.frequency = frequency;
        self
    }

    /// Gets the power in W.
    pub fn power(&self) -> Decimal {
        self.power
    }

    /// Sets the power in W.
    pub fn set_power(&mut self, power: Decimal) -> &mut Self {
        self.power = power;
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
    fn test_new() {
        let frequency = Decimal::from_f64(50.0).unwrap();
        let power = Decimal::from_f64(-3000.0).unwrap();

        let point = V2XFreqWattPointType::new(frequency, power);

        assert_eq!(point.frequency(), frequency);
        assert_eq!(point.power(), power);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_new_from_f64() {
        let frequency_f64 = 50.0;
        let power_f64 = -3000.0;

        let point = V2XFreqWattPointType::new_from_f64(frequency_f64, power_f64);

        assert_eq!(point.frequency(), Decimal::from_f64(frequency_f64).unwrap());
        assert_eq!(point.power(), Decimal::from_f64(power_f64).unwrap());
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let frequency = Decimal::from_f64(50.0).unwrap();
        let power = Decimal::from_f64(-3000.0).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let point =
            V2XFreqWattPointType::new(frequency, power).with_custom_data(custom_data.clone());

        assert_eq!(point.frequency(), frequency);
        assert_eq!(point.power(), power);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let initial_frequency = Decimal::from_f64(50.0).unwrap();
        let initial_power = Decimal::from_f64(-3000.0).unwrap();

        let new_frequency = Decimal::from_f64(49.8).unwrap();
        let new_power = Decimal::from_f64(-2500.0).unwrap();

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut point = V2XFreqWattPointType::new(initial_frequency, initial_power);

        point
            .set_frequency(new_frequency)
            .set_power(new_power)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(point.frequency(), new_frequency);
        assert_eq!(point.power(), new_power);
        assert_eq!(point.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        point.set_custom_data(None);

        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let frequency = Decimal::from_f64(50.0).unwrap();
        let power = Decimal::from_f64(-3000.0).unwrap();

        let point = V2XFreqWattPointType::new(frequency, power);

        let json = serde_json::to_string(&point).unwrap();
        let deserialized: V2XFreqWattPointType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, point);
    }
}
