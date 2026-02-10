use super::super::helpers::validator::validate_identifier_string;
use super::{custom_data::CustomDataType, gradient::GradientType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Gradient get type for retrieving gradient settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GradientGetType {
    /// Id of the setting.
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub id: String,

    /// Default ramp rate in seconds (0 if not applicable)
    #[validate(nested)]
    pub gradient: GradientType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl GradientGetType {
    /// Creates a new `GradientGetType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `gradient` - The gradient settings
    /// * `id` - Id of the setting
    ///
    /// # Returns
    ///
    /// A new `GradientGetType` instance with optional fields set to `None`
    pub fn new(gradient: GradientType, id: String) -> Self {
        Self {
            custom_data: None,
            gradient,
            id,
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
    /// The modified `GradientGetType` instance
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
    /// The modified `GradientGetType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the gradient settings.
    ///
    /// # Returns
    ///
    /// A reference to the gradient settings
    pub fn gradient(&self) -> &GradientType {
        &self.gradient
    }

    /// Sets the gradient settings.
    ///
    /// # Arguments
    ///
    /// * `gradient` - The gradient settings
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
    pub fn set_gradient(&mut self, gradient: GradientType) -> &mut Self {
        self.gradient = gradient;
        self
    }

    /// Gets the id of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the id of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the id of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the setting
    ///
    /// # Returns
    ///
    /// The modified `GradientGetType` instance
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_gradient_get_new() {
        let gradient = GradientType::new_from_f64(1, 5.0, 2.5);
        let id = "setting1".to_string();

        let gradient_get = GradientGetType::new(gradient.clone(), id.clone());

        assert_eq!(gradient_get.gradient(), &gradient);
        assert_eq!(gradient_get.id(), id);
        assert_eq!(gradient_get.custom_data(), None);
    }

    #[test]
    fn test_gradient_get_with_methods() {
        let gradient = GradientType::new_from_f64(1, 5.0, 2.5);
        let id = "setting1".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let gradient_get = GradientGetType::new(gradient.clone(), id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(gradient_get.gradient(), &gradient);
        assert_eq!(gradient_get.id(), id);
        assert_eq!(gradient_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_gradient_get_setters() {
        let gradient1 = GradientType::new_from_f64(1, 5.0, 2.5);
        let gradient2 = GradientType::new_from_f64(2, 10.0, 5.0);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut gradient_get = GradientGetType::new(gradient1.clone(), id1.clone());

        gradient_get
            .set_gradient(gradient2.clone())
            .set_id(id2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(gradient_get.gradient(), &gradient2);
        assert_eq!(gradient_get.id(), id2);
        assert_eq!(gradient_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        gradient_get.set_custom_data(None);
        assert_eq!(gradient_get.custom_data(), None);
    }

    #[test]
    fn test_gradient_get_methods() {
        let gradient = GradientType::new(1, dec!(5.0), dec!(2.5));
        let id = "setting1".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        // Create using constructor
        let mut gradient_get = GradientGetType::new(gradient.clone(), id.clone());

        // Use methods
        gradient_get = gradient_get.with_custom_data(custom_data.clone());

        assert_eq!(gradient_get.gradient(), &gradient);
        assert_eq!(gradient_get.id(), id);
        assert_eq!(gradient_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_gradient_settings_access() {
        let gradient = GradientType::new_from_f64(1, 5.0, 2.5);
        let id = "setting1".to_string();

        let gradient_get = GradientGetType::new(gradient.clone(), id.clone());

        // Access gradient settings directly
        assert_eq!(gradient_get.gradient().priority(), 1);
        assert_eq!(
            gradient_get.gradient().gradient(),
            Decimal::from_f64(5.0).unwrap()
        );
        assert_eq!(
            gradient_get.gradient().soft_gradient(),
            Decimal::from_f64(2.5).unwrap()
        );
    }
}
