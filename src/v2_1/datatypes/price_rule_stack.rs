use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price_rule::PriceRuleType};

/// Stack of price rules, defining the price of charging.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleStackType {
    /// Required. Duration in seconds after which the price rule becomes active.
    #[validate(range(min = 0))]
    pub duration: i32,

    /// Required. List of price rules that are part of the stack.
    #[validate(length(min = 1, max = 8))]
    pub price_rules: Vec<PriceRuleType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PriceRuleStackType {
    /// Creates a new `PriceRuleStackType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds after which the price rule becomes active
    /// * `price_rules` - List of price rules that are part of the stack
    ///
    /// # Returns
    ///
    /// A new instance of `PriceRuleStackType` with optional fields set to `None`
    pub fn new(duration: i32, price_rules: Vec<PriceRuleType>) -> Self {
        Self {
            custom_data: None,
            duration,
            price_rules,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price rule stack
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this price rule stack, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// The duration in seconds after which the price rule becomes active
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds after which the price rule becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the price rules.
    ///
    /// # Returns
    ///
    /// Reference to the list of price rules that are part of the stack
    pub fn price_rules(&self) -> &[PriceRuleType] {
        &self.price_rules
    }

    /// Sets the price rules.
    ///
    /// # Arguments
    ///
    /// * `price_rules` - List of price rules that are part of the stack
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_rules(&mut self, price_rules: Vec<PriceRuleType>) -> &mut Self {
        self.price_rules = price_rules;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::rational_number::RationalNumberType;

    #[test]
    fn test_new_price_rule_stack() {
        let duration = 3600;
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rules = vec![PriceRuleType::new(energy_fee, power_range_start)];

        let stack = PriceRuleStackType::new(duration, price_rules.clone());

        assert_eq!(stack.duration(), duration);
        assert_eq!(stack.price_rules().len(), 1);
        assert_eq!(stack.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rules = vec![PriceRuleType::new(energy_fee, power_range_start)];

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let stack = PriceRuleStackType::new(duration, price_rules.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(stack.duration(), duration);
        assert_eq!(stack.price_rules().len(), 1);
        assert_eq!(stack.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let duration2 = 7200;

        let energy_fee1 = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start1 = RationalNumberType::new(0, 0);
        let price_rules1 = vec![PriceRuleType::new(energy_fee1, power_range_start1)];

        let energy_fee2 = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start2 = RationalNumberType::new(0, 0);
        let energy_fee3 = RationalNumberType::new(2, 20); // Represents 0.20 with exponent 2
        let power_range_start3 = RationalNumberType::new(0, 0);
        let price_rules2 = vec![
            PriceRuleType::new(energy_fee2, power_range_start2),
            PriceRuleType::new(energy_fee3, power_range_start3),
        ];

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut stack = PriceRuleStackType::new(duration1, price_rules1);

        stack
            .set_duration(duration2)
            .set_price_rules(price_rules2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(stack.duration(), duration2);
        assert_eq!(stack.price_rules().len(), 2);
        assert_eq!(stack.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        stack.set_custom_data(None);
        assert_eq!(stack.custom_data(), None);
    }
}
