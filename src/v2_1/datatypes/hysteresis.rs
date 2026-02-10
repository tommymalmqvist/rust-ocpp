use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Hysteresis parameters for DER control functions.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct HysteresisType {
    /// High value for return to normal operation after a grid event, in absolute value. This value adopts the same unit as defined by yUnit
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default,
        rename = "hysteresisHigh"
    )]
    pub hysteresis_high: Option<Decimal>,

    /// Low value for return to normal operation after a grid event, in absolute value. This value adopts the same unit as defined by yUnit
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default,
        rename = "hysteresisLow"
    )]
    pub hysteresis_low: Option<Decimal>,

    /// Delay in seconds, once grid parameter within HysteresisLow and HysteresisHigh, for the EV to return to normal operation after a grid event.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default,
        rename = "hysteresisDelay"
    )]
    pub hysteresis_delay: Option<Decimal>,

    /// Set default rate of change (ramp rate %/s) for the EV to return to normal operation after a grid event
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default,
        rename = "hysteresisGradient"
    )]
    pub hysteresis_gradient: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl HysteresisType {
    /// Creates a new `HysteresisType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `HysteresisType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            hysteresis_high: None,
            hysteresis_low: None,
            hysteresis_delay: None,
            hysteresis_gradient: None,
            custom_data: None,
        }
    }

    /// Sets the hysteresis high value.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_high` - High value for return to normal operation after a grid event
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hysteresis_high(mut self, hysteresis_high: Decimal) -> Self {
        self.hysteresis_high = Some(hysteresis_high);
        self
    }

    /// Sets the hysteresis low value.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_low` - Low value for return to normal operation after a grid event
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hysteresis_low(mut self, hysteresis_low: Decimal) -> Self {
        self.hysteresis_low = Some(hysteresis_low);
        self
    }

    /// Sets the hysteresis delay.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_delay` - Delay in seconds for the EV to return to normal operation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hysteresis_delay(mut self, hysteresis_delay: Decimal) -> Self {
        self.hysteresis_delay = Some(hysteresis_delay);
        self
    }

    /// Sets the hysteresis gradient.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_gradient` - Rate of change for the EV to return to normal operation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hysteresis_gradient(mut self, hysteresis_gradient: Decimal) -> Self {
        self.hysteresis_gradient = Some(hysteresis_gradient);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these hysteresis parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the hysteresis high value.
    ///
    /// # Returns
    ///
    /// An optional reference to the hysteresis high value
    pub fn hysteresis_high(&self) -> Option<&Decimal> {
        self.hysteresis_high.as_ref()
    }

    /// Sets the hysteresis high value.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_high` - High value for return to normal operation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hysteresis_high(&mut self, hysteresis_high: Option<Decimal>) -> &mut Self {
        self.hysteresis_high = hysteresis_high;
        self
    }

    /// Gets the hysteresis low value.
    ///
    /// # Returns
    ///
    /// An optional reference to the hysteresis low value
    pub fn hysteresis_low(&self) -> Option<&Decimal> {
        self.hysteresis_low.as_ref()
    }

    /// Sets the hysteresis low value.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_low` - Low value for return to normal operation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hysteresis_low(&mut self, hysteresis_low: Option<Decimal>) -> &mut Self {
        self.hysteresis_low = hysteresis_low;
        self
    }

    /// Gets the hysteresis delay.
    ///
    /// # Returns
    ///
    /// An optional reference to the hysteresis delay
    pub fn hysteresis_delay(&self) -> Option<&Decimal> {
        self.hysteresis_delay.as_ref()
    }

    /// Sets the hysteresis delay.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_delay` - Delay in seconds for the EV to return to normal operation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hysteresis_delay(&mut self, hysteresis_delay: Option<Decimal>) -> &mut Self {
        self.hysteresis_delay = hysteresis_delay;
        self
    }

    /// Gets the hysteresis gradient.
    ///
    /// # Returns
    ///
    /// An optional reference to the hysteresis gradient
    pub fn hysteresis_gradient(&self) -> Option<&Decimal> {
        self.hysteresis_gradient.as_ref()
    }

    /// Sets the hysteresis gradient.
    ///
    /// # Arguments
    ///
    /// * `hysteresis_gradient` - Rate of change for the EV to return to normal operation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hysteresis_gradient(&mut self, hysteresis_gradient: Option<Decimal>) -> &mut Self {
        self.hysteresis_gradient = hysteresis_gradient;
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
    /// * `custom_data` - Custom data for these hysteresis parameters, or None to clear
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
    fn test_new_hysteresis() {
        let hysteresis = HysteresisType::new();

        assert_eq!(hysteresis.hysteresis_high(), None);
        assert_eq!(hysteresis.hysteresis_low(), None);
        assert_eq!(hysteresis.hysteresis_delay(), None);
        assert_eq!(hysteresis.hysteresis_gradient(), None);
        assert_eq!(hysteresis.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let high = dec!(5.0);
        let low = dec!(2.0);
        let delay = dec!(10.0);
        let gradient = dec!(0.5);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let hysteresis = HysteresisType::new()
            .with_hysteresis_high(high)
            .with_hysteresis_low(low)
            .with_hysteresis_delay(delay)
            .with_hysteresis_gradient(gradient)
            .with_custom_data(custom_data.clone());

        assert_eq!(hysteresis.hysteresis_high(), Some(&high));
        assert_eq!(hysteresis.hysteresis_low(), Some(&low));
        assert_eq!(hysteresis.hysteresis_delay(), Some(&delay));
        assert_eq!(hysteresis.hysteresis_gradient(), Some(&gradient));
        assert_eq!(hysteresis.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let high1 = dec!(5.0);
        let high2 = dec!(6.0);
        let low = dec!(2.0);
        let delay = dec!(10.0);
        let gradient = dec!(0.5);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut hysteresis = HysteresisType::new();

        hysteresis
            .set_hysteresis_high(Some(high1))
            .set_hysteresis_low(Some(low))
            .set_hysteresis_delay(Some(delay))
            .set_hysteresis_gradient(Some(gradient))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(hysteresis.hysteresis_high(), Some(&high1));
        assert_eq!(hysteresis.hysteresis_low(), Some(&low));
        assert_eq!(hysteresis.hysteresis_delay(), Some(&delay));
        assert_eq!(hysteresis.hysteresis_gradient(), Some(&gradient));
        assert_eq!(hysteresis.custom_data(), Some(&custom_data));

        // Test updating a field
        hysteresis.set_hysteresis_high(Some(high2));
        assert_eq!(hysteresis.hysteresis_high(), Some(&high2));

        // Test clearing optional fields
        hysteresis.set_hysteresis_high(None);
        hysteresis.set_hysteresis_low(None);
        hysteresis.set_hysteresis_delay(None);
        hysteresis.set_hysteresis_gradient(None);
        hysteresis.set_custom_data(None);

        assert_eq!(hysteresis.hysteresis_high(), None);
        assert_eq!(hysteresis.hysteresis_low(), None);
        assert_eq!(hysteresis.hysteresis_delay(), None);
        assert_eq!(hysteresis.hysteresis_gradient(), None);
        assert_eq!(hysteresis.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        let hysteresis = HysteresisType::new()
            .with_hysteresis_high(dec!(5.0))
            .with_hysteresis_low(dec!(2.0))
            .with_hysteresis_delay(dec!(10.0))
            .with_hysteresis_gradient(dec!(0.5));

        // Validation should pass as all fields are valid
        assert!(hysteresis.validate().is_ok());
    }
}
