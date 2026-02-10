use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Part of ISO 15118-20 price schedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RationalNumberType {
    /// The exponent to base 10 (dec)
    pub exponent: i32,

    /// Value which shall be multiplied
    pub value: i32,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RationalNumberType {
    /// Creates a new `RationalNumberType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent to base 10 (dec)
    /// * `value` - Value which shall be multiplied
    ///
    /// # Returns
    ///
    /// A new instance of `RationalNumberType` with optional fields set to `None`
    pub fn new(exponent: i32, value: i32) -> Self {
        Self {
            exponent,
            value,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this rational number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the exponent.
    ///
    /// # Returns
    ///
    /// The exponent to base 10 (dec)
    pub fn exponent(&self) -> i32 {
        self.exponent
    }

    /// Sets the exponent.
    ///
    /// # Arguments
    ///
    /// * `exponent` - The exponent to base 10 (dec)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_exponent(&mut self, exponent: i32) -> &mut Self {
        self.exponent = exponent;
        self
    }

    /// Gets the value.
    ///
    /// # Returns
    ///
    /// The value which shall be multiplied
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value which shall be multiplied
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: i32) -> &mut Self {
        self.value = value;
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
    /// * `custom_data` - Custom data for this rational number, or None to clear
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
    fn test_new_rational_number() {
        let exponent = 3;
        let value = 42;
        let rational_number = RationalNumberType::new(exponent, value);

        assert_eq!(rational_number.exponent(), exponent);
        assert_eq!(rational_number.value(), value);
        assert_eq!(rational_number.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let exponent = 3;
        let value = 42;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let rational_number =
            RationalNumberType::new(exponent, value).with_custom_data(custom_data.clone());

        assert_eq!(rational_number.exponent(), exponent);
        assert_eq!(rational_number.value(), value);
        assert_eq!(rational_number.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let exponent1 = 3;
        let value1 = 42;
        let exponent2 = 2;
        let value2 = 100;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut rational_number = RationalNumberType::new(exponent1, value1);

        rational_number
            .set_exponent(exponent2)
            .set_value(value2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(rational_number.exponent(), exponent2);
        assert_eq!(rational_number.value(), value2);
        assert_eq!(rational_number.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        rational_number.set_custom_data(None);
        assert_eq!(rational_number.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let exponent = -2;
        let value = 5;
        let rational_number = RationalNumberType::new(exponent, value);
        let serialized = serde_json::to_string(&rational_number).unwrap();
        let expected = format!(r#"{{"exponent":{},"value":{}}}"#, exponent, value);
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialization() {
        let json_str = r#"{"exponent": 1, "value": 10}"#;
        let rational_number: RationalNumberType = serde_json::from_str(json_str).unwrap();
        assert_eq!(rational_number.exponent(), 1);
        assert_eq!(rational_number.value(), 10);
        assert_eq!(rational_number.custom_data(), None);
    }

    #[test]
    fn test_edge_cases() {
        // Test with maximum and minimum i32 values
        let rational_number_max = RationalNumberType::new(i32::MAX, i32::MAX);
        assert_eq!(rational_number_max.exponent(), i32::MAX);
        assert_eq!(rational_number_max.value(), i32::MAX);

        let rational_number_min = RationalNumberType::new(i32::MIN, i32::MIN);
        assert_eq!(rational_number_min.exponent(), i32::MIN);
        assert_eq!(rational_number_min.value(), i32::MIN);
    }

    #[test]
    fn test_validation() {
        let rational_number = RationalNumberType::new(0, 0);
        assert!(rational_number.validate().is_ok());
    }
}
