use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Limit at State of Charge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitAtSoCType {
    /// State of Charge at which power limit becomes active.
    #[validate(range(min = 0, max = 100))]
    pub soc: i32,

    /// Maximum power level when power limit is active.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub limit: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl LimitAtSoCType {
    /// Creates a new `LimitAtSoCType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `soc` - State of Charge at which power limit becomes active (0-100)
    /// * `limit` - Maximum power level when power limit is active
    ///
    /// # Returns
    ///
    /// A new instance of `LimitAtSoCType` with optional fields set to `None`
    pub fn new(soc: i32, limit: Decimal) -> Self {
        Self {
            soc,
            limit,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this LimitAtSoC
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the State of Charge.
    ///
    /// # Returns
    ///
    /// The State of Charge at which power limit becomes active
    pub fn soc(&self) -> i32 {
        self.soc
    }

    /// Sets the State of Charge.
    ///
    /// # Arguments
    ///
    /// * `soc` - State of Charge at which power limit becomes active (0-100)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_soc(&mut self, soc: i32) -> &mut Self {
        self.soc = soc;
        self
    }

    /// Gets the power limit.
    ///
    /// # Returns
    ///
    /// The maximum power level when power limit is active
    pub fn limit(&self) -> &Decimal {
        &self.limit
    }

    /// Sets the power limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum power level when power limit is active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit(&mut self, limit: Decimal) -> &mut Self {
        self.limit = limit;
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
    /// * `custom_data` - Custom data for this LimitAtSoC, or None to clear
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

    #[test]
    fn test_new_limit_at_soc() {
        let soc = 80;
        let limit_value = dec!(7500.0);

        let limit_at_soc = LimitAtSoCType::new(soc, limit_value.clone());

        assert_eq!(limit_at_soc.soc(), soc);
        assert_eq!(limit_at_soc.limit(), &limit_value);
        assert_eq!(limit_at_soc.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let soc = 80;
        let limit_value = dec!(7500.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let limit_at_soc =
            LimitAtSoCType::new(soc, limit_value.clone()).with_custom_data(custom_data.clone());

        assert_eq!(limit_at_soc.soc(), soc);
        assert_eq!(limit_at_soc.limit(), &limit_value);
        assert_eq!(limit_at_soc.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let soc1 = 80;
        let limit_value1 = dec!(7500.0);
        let soc2 = 90;
        let limit_value2 = dec!(5000.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut limit_at_soc = LimitAtSoCType::new(soc1, limit_value1);

        limit_at_soc
            .set_soc(soc2)
            .set_limit(limit_value2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(limit_at_soc.soc(), soc2);
        assert_eq!(limit_at_soc.limit(), &limit_value2);
        assert_eq!(limit_at_soc.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit_at_soc.set_custom_data(None);
        assert_eq!(limit_at_soc.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        // Valid values
        let limit_at_soc = LimitAtSoCType::new(80, dec!(7500.0));
        assert!(limit_at_soc.validate().is_ok());

        // Test with minimum valid SoC value
        let limit_at_soc = LimitAtSoCType::new(0, dec!(7500.0));
        assert!(limit_at_soc.validate().is_ok());

        // Test with maximum valid SoC value
        let limit_at_soc = LimitAtSoCType::new(100, dec!(7500.0));
        assert!(limit_at_soc.validate().is_ok());

        // Test with invalid SoC value (below minimum)
        let limit_at_soc = LimitAtSoCType::new(-1, dec!(7500.0));
        assert!(limit_at_soc.validate().is_err());

        // Test with invalid SoC value (above maximum)
        let limit_at_soc = LimitAtSoCType::new(101, dec!(7500.0));
        assert!(limit_at_soc.validate().is_err());
    }
}
