use super::custom_data::CustomDataType;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Gradient settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GradientType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Default ramp rate in seconds (0 if not applicable)
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub gradient: Decimal,

    /// Soft-start ramp rate in seconds (0 if not applicable)
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub soft_gradient: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GradientType {
    /// Creates a new `GradientType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `gradient` - Default ramp rate in seconds
    /// * `soft_gradient` - Soft-start ramp rate in seconds
    ///
    /// # Returns
    ///
    /// A new `GradientType` instance with optional fields set to `None`
    pub fn new(priority: i32, gradient: Decimal, soft_gradient: Decimal) -> Self {
        Self {
            custom_data: None,
            priority,
            gradient,
            soft_gradient,
        }
    }

    /// Creates a new `GradientType` with the required fields using f64 values.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `gradient` - Default ramp rate in seconds
    /// * `soft_gradient` - Soft-start ramp rate in seconds
    ///
    /// # Returns
    ///
    /// A new `GradientType` instance with optional fields set to `None`
    pub fn new_from_f64(priority: i32, gradient: f64, soft_gradient: f64) -> Self {
        Self {
            custom_data: None,
            priority,
            gradient: Decimal::from_f64(gradient).unwrap_or_default(),
            soft_gradient: Decimal::from_f64(soft_gradient).unwrap_or_default(),
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the priority.
    ///
    /// # Returns
    ///
    /// The priority of setting (0=highest)
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the gradient.
    ///
    /// # Returns
    ///
    /// The default ramp rate in seconds
    pub fn gradient(&self) -> Decimal {
        self.gradient
    }

    /// Sets the gradient.
    ///
    /// # Arguments
    ///
    /// * `gradient` - Default ramp rate in seconds
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_gradient(&mut self, gradient: Decimal) -> &mut Self {
        self.gradient = gradient;
        self
    }

    /// Sets the gradient using an f64 value.
    ///
    /// # Arguments
    ///
    /// * `gradient` - Default ramp rate in seconds
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_gradient_f64(&mut self, gradient: f64) -> &mut Self {
        self.gradient = Decimal::from_f64(gradient).unwrap_or_default();
        self
    }

    /// Gets the soft gradient.
    ///
    /// # Returns
    ///
    /// The soft-start ramp rate in seconds
    pub fn soft_gradient(&self) -> Decimal {
        self.soft_gradient
    }

    /// Sets the soft gradient.
    ///
    /// # Arguments
    ///
    /// * `soft_gradient` - Soft-start ramp rate in seconds
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_soft_gradient(&mut self, soft_gradient: Decimal) -> &mut Self {
        self.soft_gradient = soft_gradient;
        self
    }

    /// Sets the soft gradient using an f64 value.
    ///
    /// # Arguments
    ///
    /// * `soft_gradient` - Soft-start ramp rate in seconds
    ///
    /// # Returns
    ///
    /// The modified `GradientType` instance
    pub fn set_soft_gradient_f64(&mut self, soft_gradient: f64) -> &mut Self {
        self.soft_gradient = Decimal::from_f64(soft_gradient).unwrap_or_default();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_gradient_new() {
        let priority = 1;
        let gradient = dec!(5.0);
        let soft_gradient = dec!(2.5);

        let gradient_type = GradientType::new(priority, gradient, soft_gradient);

        assert_eq!(gradient_type.priority(), priority);
        assert_eq!(gradient_type.gradient(), gradient);
        assert_eq!(gradient_type.soft_gradient(), soft_gradient);
        assert_eq!(gradient_type.custom_data(), None);
    }

    #[test]
    fn test_gradient_new_from_f64() {
        let priority = 1;
        let gradient_f64 = 5.0;
        let soft_gradient_f64 = 2.5;

        let gradient_type = GradientType::new_from_f64(priority, gradient_f64, soft_gradient_f64);

        assert_eq!(gradient_type.priority(), priority);
        assert_eq!(
            gradient_type.gradient(),
            Decimal::from_f64(gradient_f64).unwrap()
        );
        assert_eq!(
            gradient_type.soft_gradient(),
            Decimal::from_f64(soft_gradient_f64).unwrap()
        );
        assert_eq!(gradient_type.custom_data(), None);
    }

    #[test]
    fn test_gradient_with_methods() {
        let priority = 1;
        let gradient = dec!(5.0);
        let soft_gradient = dec!(2.5);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let gradient_type = GradientType::new(priority, gradient, soft_gradient)
            .with_custom_data(custom_data.clone());

        assert_eq!(gradient_type.priority(), priority);
        assert_eq!(gradient_type.gradient(), gradient);
        assert_eq!(gradient_type.soft_gradient(), soft_gradient);
        assert_eq!(gradient_type.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_gradient_setters() {
        let priority1 = 1;
        let priority2 = 2;
        let gradient1 = dec!(5.0);
        let gradient2 = dec!(10.0);
        let soft_gradient1 = dec!(2.5);
        let soft_gradient2 = dec!(5.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut gradient_type = GradientType::new(priority1, gradient1, soft_gradient1);

        gradient_type
            .set_priority(priority2)
            .set_gradient(gradient2)
            .set_soft_gradient(soft_gradient2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(gradient_type.priority(), priority2);
        assert_eq!(gradient_type.gradient(), gradient2);
        assert_eq!(gradient_type.soft_gradient(), soft_gradient2);
        assert_eq!(gradient_type.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        gradient_type.set_custom_data(None);
        assert_eq!(gradient_type.custom_data(), None);
    }

    #[test]
    fn test_gradient_f64_setters() {
        let priority = 1;
        let gradient = dec!(5.0);
        let soft_gradient = dec!(2.5);
        let gradient_f64 = 10.0;
        let soft_gradient_f64 = 5.0;

        let mut gradient_type = GradientType::new(priority, gradient, soft_gradient);

        gradient_type
            .set_gradient_f64(gradient_f64)
            .set_soft_gradient_f64(soft_gradient_f64);

        assert_eq!(
            gradient_type.gradient(),
            Decimal::from_f64(gradient_f64).unwrap()
        );
        assert_eq!(
            gradient_type.soft_gradient(),
            Decimal::from_f64(soft_gradient_f64).unwrap()
        );
    }

    #[test]
    fn test_gradient_methods() {
        let priority = 1;
        let gradient = dec!(5.0);
        let soft_gradient = dec!(2.5);
        let custom_data = CustomDataType::new("VendorX".to_string());

        // Create using constructor
        let mut gradient_type = GradientType::new(priority, gradient, soft_gradient);

        // Use methods
        gradient_type = gradient_type.with_custom_data(custom_data.clone());

        assert_eq!(gradient_type.priority(), priority);
        assert_eq!(gradient_type.gradient(), gradient);
        assert_eq!(gradient_type.soft_gradient(), soft_gradient);
        assert_eq!(gradient_type.custom_data(), Some(&custom_data));

        // Test setter methods
        gradient_type.set_priority(2);
        assert_eq!(gradient_type.priority(), 2);
    }
}
