use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Represents a UnitOfMeasure with a multiplier
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    /// Unit of the value. Default = "Wh" if the (default) measurand is an "Energy" type.
    /// This field SHALL use a value from the list Standardized Units of Measurements in Part 2 Appendices.
    /// If an applicable unit is available in that list, otherwise a "custom" unit might be used.
    #[serde(default = "default_unit")]
    #[validate(length(max = 20))]
    pub unit: String,

    /// Multiplier, this value represents the exponent to base 10. I.e. multiplier 3 means 10 raised to the 3rd power. Default is 0.
    #[serde(default)]
    pub multiplier: i32,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

fn default_unit() -> String {
    "Wh".to_string()
}

impl UnitOfMeasureType {
    /// Creates a new `UnitOfMeasureType` with default values.
    pub fn new() -> Self {
        Self {
            unit: default_unit(),
            multiplier: 0,
            custom_data: None,
        }
    }

    /// Creates a new `UnitOfMeasureType` with the specified unit.
    pub fn new_with_unit(unit: String) -> Self {
        Self {
            unit,
            multiplier: 0,
            custom_data: None,
        }
    }

    /// Gets the unit.
    pub fn unit(&self) -> &str {
        &self.unit
    }

    /// Sets the unit.
    pub fn set_unit(&mut self, unit: String) -> &mut Self {
        self.unit = unit;
        self
    }

    /// Gets the multiplier.
    pub fn multiplier(&self) -> i32 {
        self.multiplier
    }

    /// Sets the multiplier.
    pub fn set_multiplier(&mut self, multiplier: i32) -> &mut Self {
        self.multiplier = multiplier;
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

    /// Sets the unit using the builder pattern.
    pub fn with_unit(mut self, unit: String) -> Self {
        self.unit = unit;
        self
    }

    /// Sets the multiplier using the builder pattern.
    pub fn with_multiplier(mut self, multiplier: i32) -> Self {
        self.multiplier = multiplier;
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
    fn test_unit_of_measure_new() {
        let unit_of_measure = UnitOfMeasureType::new();

        assert_eq!(unit_of_measure.unit(), "Wh");
        assert_eq!(unit_of_measure.multiplier(), 0);
        assert_eq!(unit_of_measure.custom_data(), None);
    }

    #[test]
    fn test_unit_of_measure_new_with_unit() {
        let unit = "kWh".to_string();
        let unit_of_measure = UnitOfMeasureType::new_with_unit(unit.clone());

        assert_eq!(unit_of_measure.unit(), unit);
        assert_eq!(unit_of_measure.multiplier(), 0);
        assert_eq!(unit_of_measure.custom_data(), None);
    }

    #[test]
    fn test_unit_of_measure_with_methods() {
        let unit = "kWh".to_string();
        let multiplier = 3;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let unit_of_measure = UnitOfMeasureType::new()
            .with_unit(unit.clone())
            .with_multiplier(multiplier)
            .with_custom_data(custom_data.clone());

        assert_eq!(unit_of_measure.unit(), unit);
        assert_eq!(unit_of_measure.multiplier(), multiplier);
        assert_eq!(unit_of_measure.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_unit_of_measure_setters() {
        let unit = "A".to_string();
        let multiplier1 = 0;
        let multiplier2 = 3;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut unit_of_measure = UnitOfMeasureType::new();
        assert_eq!(unit_of_measure.multiplier(), multiplier1);

        unit_of_measure
            .set_unit(unit.clone())
            .set_multiplier(multiplier2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(unit_of_measure.unit(), unit);
        assert_eq!(unit_of_measure.multiplier(), multiplier2);
        assert_eq!(unit_of_measure.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        unit_of_measure.set_custom_data(None);

        assert_eq!(unit_of_measure.custom_data(), None);
    }

    #[test]
    fn test_default_serialization() {
        let unit_of_measure = UnitOfMeasureType::new();
        let json = serde_json::to_string(&unit_of_measure).unwrap();

        // Default values should be included in serialization
        assert!(json.contains("\"unit\":\"Wh\""));
        assert!(json.contains("\"multiplier\":0"));

        // Custom data is None, so it should not be included
        assert!(!json.contains("customData"));
    }
}
