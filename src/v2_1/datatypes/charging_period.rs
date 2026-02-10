use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost_dimension::CostDimensionType, custom_data::CustomDataType};

/// A ChargingPeriodType consists of a start time, and a list of possible values that influence this period,
/// for example: amount of energy charged this period, maximum current during this period etc.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingPeriodType {
    /// Start timestamp of charging period. A period ends when the next period starts.
    /// The last period ends when the session ends.
    pub start_period: DateTime<Utc>,

    /// List of dimensions that influence this period.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1), nested)]
    pub dimensions: Option<Vec<CostDimensionType>>,

    /// Unique identifier of the Tariff that was used to calculate cost.
    /// If not provided, then cost was calculated by some other means.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingPeriodType {
    /// Creates a new `ChargingPeriodType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start timestamp of charging period
    /// * `dimensions` - List of dimensions that influence this period
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingPeriodType` with optional fields set to `None`
    pub fn new(start_period: DateTime<Utc>, dimensions: Vec<CostDimensionType>) -> Self {
        Self {
            start_period,
            dimensions: Some(dimensions),
            tariff_id: None,
            custom_data: None,
        }
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier of the Tariff that was used to calculate cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tariff_id(mut self, tariff_id: String) -> Self {
        self.tariff_id = Some(tariff_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start period.
    ///
    /// # Returns
    ///
    /// The start timestamp of the charging period
    pub fn start_period(&self) -> &DateTime<Utc> {
        &self.start_period
    }

    /// Sets the start period.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start timestamp of charging period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_period(&mut self, start_period: DateTime<Utc>) -> &mut Self {
        self.start_period = start_period;
        self
    }

    /// Gets the dimensions.
    ///
    /// # Returns
    ///
    /// A reference to the list of dimensions that influence this period
    pub fn dimensions(&self) -> &Vec<CostDimensionType> {
        self.dimensions
            .as_ref()
            .expect("dimensions should always be set")
    }

    /// Sets the dimensions.
    ///
    /// # Arguments
    ///
    /// * `dimensions` - List of dimensions that influence this period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_dimensions(&mut self, dimensions: Vec<CostDimensionType>) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    /// Gets the tariff ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the tariff ID
    pub fn tariff_id(&self) -> Option<&String> {
        self.tariff_id.as_ref()
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier of the Tariff that was used to calculate cost, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: Option<String>) -> &mut Self {
        self.tariff_id = tariff_id;
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
    /// * `custom_data` - Custom data for this charging period, or None to clear
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
    use crate::v2_1::enumerations::CostDimensionEnumType;
    use rust_decimal::Decimal;
    use validator::Validate;

    #[test]
    fn test_new_charging_period() {
        let start_time = Utc::now();
        let dimension = CostDimensionType {
            type_: CostDimensionEnumType::Energy,
            volume: Decimal::try_from(10.5).unwrap_or_default(),
            custom_data: None,
        };

        let period = ChargingPeriodType::new(start_time, vec![dimension.clone()]);

        assert_eq!(period.start_period(), &start_time);
        assert_eq!(period.dimensions().len(), 1);
        assert_eq!(
            period.dimensions()[0].r#type(),
            &CostDimensionEnumType::Energy
        );
        assert_eq!(period.dimensions()[0].volume(), 10.5);
        assert_eq!(period.tariff_id(), None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let start_time = Utc::now();
        let dimension = CostDimensionType {
            type_: CostDimensionEnumType::Energy,
            volume: Decimal::try_from(10.5).unwrap_or_default(),
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        let period = ChargingPeriodType::new(start_time, vec![dimension.clone()])
            .with_tariff_id("tariff-123".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(period.start_period(), &start_time);
        assert_eq!(period.dimensions().len(), 1);
        assert_eq!(
            period.dimensions()[0].r#type(),
            &CostDimensionEnumType::Energy
        );
        assert_eq!(period.dimensions()[0].volume(), 10.5);
        assert_eq!(period.tariff_id(), Some(&"tariff-123".to_string()));
        assert_eq!(period.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let start_time = Utc::now();
        let dimension1 = CostDimensionType {
            type_: CostDimensionEnumType::Energy,
            volume: Decimal::try_from(10.5).unwrap_or_default(),
            custom_data: None,
        };
        let dimension2 = CostDimensionType {
            type_: CostDimensionEnumType::ChargingTime,
            volume: Decimal::try_from(30.0).unwrap_or_default(),
            custom_data: None,
        };
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut period = ChargingPeriodType::new(start_time, vec![dimension1.clone()]);

        let new_time = Utc::now();
        period
            .set_start_period(new_time)
            .set_dimensions(vec![dimension1.clone(), dimension2.clone()])
            .set_tariff_id(Some("tariff-456".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(period.start_period(), &new_time);
        assert_eq!(period.dimensions().len(), 2);
        assert_eq!(
            period.dimensions()[0].r#type(),
            &CostDimensionEnumType::Energy
        );
        assert_eq!(period.dimensions()[0].volume(), 10.5);
        assert_eq!(
            period.dimensions()[1].r#type(),
            &CostDimensionEnumType::ChargingTime
        );
        assert_eq!(period.dimensions()[1].volume(), 30.0);
        assert_eq!(period.tariff_id(), Some(&"tariff-456".to_string()));
        assert_eq!(period.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        period.set_tariff_id(None).set_custom_data(None);

        assert_eq!(period.tariff_id(), None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        let start_time = Utc::now();
        let dimension = CostDimensionType {
            type_: CostDimensionEnumType::Energy,
            volume: Decimal::try_from(10.5).unwrap_or_default(),
            custom_data: None,
        };

        // 1. Valid charging period - should pass validation
        let valid_period = ChargingPeriodType::new(start_time, vec![dimension.clone()]);
        assert!(
            valid_period.validate().is_ok(),
            "Valid charging period should pass validation"
        );

        // 2. Test dimensions validation (empty dimensions)
        let mut invalid_dimensions_period = valid_period.clone();
        invalid_dimensions_period.dimensions = Some(vec![]);

        let validation_result = invalid_dimensions_period.validate();
        assert!(
            validation_result.is_err(),
            "Charging period with empty dimensions should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("dimensions"),
            "Error should mention dimensions: {}",
            error
        );

        // 3. Test tariff_id validation (too long)
        let long_tariff_id = "A".repeat(61); // 61 characters, exceeds max of 60
        let mut invalid_tariff_id_period = valid_period.clone();
        invalid_tariff_id_period.tariff_id = Some(long_tariff_id);

        let validation_result = invalid_tariff_id_period.validate();
        assert!(
            validation_result.is_err(),
            "Charging period with too long tariff_id should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("tariff_id"),
            "Error should mention tariff_id: {}",
            error
        );

        // 4. Test custom_data nested validation
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255

        let mut invalid_custom_data_period = valid_period.clone();
        invalid_custom_data_period.custom_data = Some(invalid_custom_data);

        let validation_result = invalid_custom_data_period.validate();
        assert!(
            validation_result.is_err(),
            "Charging period with invalid custom_data should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }
}
