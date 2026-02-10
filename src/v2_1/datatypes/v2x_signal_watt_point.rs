use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// *(2.1)* A point of a signal-watt curve.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XSignalWattPointType {
    /// Signal value from an AFRRSignalRequest.
    pub signal: i32,

    /// Power in W to charge (positive) or discharge (negative) at specified frequency.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub power: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl V2XSignalWattPointType {
    /// Creates a new `V2XSignalWattPointType` with the required fields.
    pub fn new(signal: i32, power: Decimal) -> Self {
        Self {
            signal,
            power,
            custom_data: None,
        }
    }

    /// Creates a new `V2XSignalWattPointType` from floating-point power value.
    pub fn new_with_f64_power(signal: i32, power: f64) -> Self {
        Self {
            signal,
            power: Decimal::from_f64(power).unwrap_or_else(|| Decimal::new(0, 0)),
            custom_data: None,
        }
    }

    /// Gets the signal value.
    pub fn signal(&self) -> i32 {
        self.signal
    }

    /// Sets the signal value.
    pub fn set_signal(&mut self, signal: i32) -> &mut Self {
        self.signal = signal;
        self
    }

    /// Gets the power value.
    pub fn power(&self) -> Decimal {
        self.power
    }

    /// Sets the power value.
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
        let signal = 75;
        let power = Decimal::from_f64(-3000.0).unwrap();

        let point = V2XSignalWattPointType::new(signal, power);

        assert_eq!(point.signal(), signal);
        assert_eq!(point.power(), power);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_new_with_f64_power() {
        let signal = 50;
        let power_f64 = -3000.0;

        let point = V2XSignalWattPointType::new_with_f64_power(signal, power_f64);

        assert_eq!(point.signal(), signal);
        assert_eq!(point.power(), Decimal::from_f64(power_f64).unwrap());
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let signal = 75;
        let power = Decimal::from_f64(-3000.0).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let point =
            V2XSignalWattPointType::new(signal, power).with_custom_data(custom_data.clone());

        assert_eq!(point.signal(), signal);
        assert_eq!(point.power(), power);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let initial_signal = 75;
        let initial_power = Decimal::from_f64(-3000.0).unwrap();

        let new_signal = 80;
        let new_power = Decimal::from_f64(-2500.0).unwrap();

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut point = V2XSignalWattPointType::new(initial_signal, initial_power);

        point
            .set_signal(new_signal)
            .set_power(new_power)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(point.signal(), new_signal);
        assert_eq!(point.power(), new_power);
        assert_eq!(point.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        point.set_custom_data(None);

        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let signal = 75;
        let power = Decimal::from_f64(-3000.0).unwrap();

        let point = V2XSignalWattPointType::new(signal, power);

        let json = serde_json::to_string(&point).unwrap();
        let deserialized: V2XSignalWattPointType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, point);
    }
}
