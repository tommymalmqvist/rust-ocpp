use crate::v2_1::{
    datatypes::custom_data::CustomDataType, enumerations::ChargingLimitSourceEnumType,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents a charging limit for a charging session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingLimitType {
    /// Represents the source of the charging limit.
    pub charging_limit_source: ChargingLimitSourceEnumType,

    /// True when the reported limit concerns local generation that is providing extra capacity, instead of a limitation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_generation: Option<bool>,

    /// Indicates whether the charging limit is critical for the grid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_grid_critical: Option<bool>,

    /// Custom data specific to this charging limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingLimitType {
    /// Creates a new `ChargingLimitType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `charging_limit_source` - Source of the charging limit
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingLimitType` with optional fields set to `None`
    pub fn new(charging_limit_source: ChargingLimitSourceEnumType) -> Self {
        Self {
            charging_limit_source,
            is_local_generation: None,
            is_grid_critical: None,
            custom_data: None,
        }
    }

    /// Sets whether the limit concerns local generation providing extra capacity.
    ///
    /// # Arguments
    ///
    /// * `is_local_generation` - True when the reported limit concerns local generation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_local_generation(mut self, is_local_generation: bool) -> Self {
        self.is_local_generation = Some(is_local_generation);
        self
    }

    /// Sets whether the charging limit is critical for the grid.
    ///
    /// # Arguments
    ///
    /// * `is_grid_critical` - Indicates whether the charging limit is critical for the grid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_grid_critical(mut self, is_grid_critical: bool) -> Self {
        self.is_grid_critical = Some(is_grid_critical);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging limit
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the charging limit source.
    ///
    /// # Returns
    ///
    /// The source of the charging limit
    pub fn charging_limit_source(&self) -> &ChargingLimitSourceEnumType {
        &self.charging_limit_source
    }

    /// Sets the charging limit source.
    ///
    /// # Arguments
    ///
    /// * `charging_limit_source` - Source of the charging limit
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_limit_source(
        &mut self,
        charging_limit_source: ChargingLimitSourceEnumType,
    ) -> &mut Self {
        self.charging_limit_source = charging_limit_source;
        self
    }

    /// Gets whether the limit concerns local generation.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating if the limit concerns local generation
    pub fn is_local_generation(&self) -> Option<bool> {
        self.is_local_generation
    }

    /// Sets whether the limit concerns local generation.
    ///
    /// # Arguments
    ///
    /// * `is_local_generation` - True when the reported limit concerns local generation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_local_generation(&mut self, is_local_generation: Option<bool>) -> &mut Self {
        self.is_local_generation = is_local_generation;
        self
    }

    /// Gets whether the charging limit is critical for the grid.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating if the charging limit is critical for the grid
    pub fn is_grid_critical(&self) -> Option<bool> {
        self.is_grid_critical
    }

    /// Sets whether the charging limit is critical for the grid.
    ///
    /// # Arguments
    ///
    /// * `is_grid_critical` - Indicates whether the charging limit is critical for the grid, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_grid_critical(&mut self, is_grid_critical: Option<bool>) -> &mut Self {
        self.is_grid_critical = is_grid_critical;
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
    /// * `custom_data` - Custom data for this charging limit, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Validates this instance according to the OCPP 2.1 specification.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the instance is valid, otherwise an error
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Validate::validate(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_new_charging_limit() {
        let limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::EMS
            )
        );

        assert_eq!(
            limit.charging_limit_source(),
            &ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::EMS
            )
        );
        assert_eq!(limit.is_local_generation(), None);
        assert_eq!(limit.is_grid_critical(), None);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::CSO
            )
        )
        .with_local_generation(true)
        .with_grid_critical(false)
        .with_custom_data(custom_data.clone());

        assert_eq!(
            limit.charging_limit_source(),
            &ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::CSO
            )
        );
        assert_eq!(limit.is_local_generation(), Some(true));
        assert_eq!(limit.is_grid_critical(), Some(false));
        assert_eq!(limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::EMS
            )
        );

        limit
            .set_charging_limit_source(
                ChargingLimitSourceEnumType::Standard(
                    crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::SO
                )
            )
            .set_local_generation(Some(true))
            .set_grid_critical(Some(true))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            limit.charging_limit_source(),
            &ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::SO
            )
        );
        assert_eq!(limit.is_local_generation(), Some(true));
        assert_eq!(limit.is_grid_critical(), Some(true));
        assert_eq!(limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit
            .set_local_generation(None)
            .set_grid_critical(None)
            .set_custom_data(None);

        assert_eq!(limit.is_local_generation(), None);
        assert_eq!(limit.is_grid_critical(), None);
        assert_eq!(limit.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Create a valid charging limit
        let valid_limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::EMS
            )
        );

        // Validation should pass
        assert!(valid_limit.validate().is_ok());

        // Test with valid custom data
        let custom_data = CustomDataType::new("VendorX".to_string());
        let valid_limit_with_custom_data = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::CSO
            )
        )
        .with_local_generation(true)
        .with_grid_critical(false)
        .with_custom_data(custom_data);

        // Validation should pass
        assert!(valid_limit_with_custom_data.validate().is_ok());

        // Test with invalid custom data (nested validation)
        let invalid_custom_data = CustomDataType::new("a".repeat(256)); // Exceeds max length of 255
        let invalid_limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::SO
            )
        )
        .with_custom_data(invalid_custom_data);

        // Validation should fail
        assert!(invalid_limit.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let limit = ChargingLimitType::new(
            ChargingLimitSourceEnumType::Standard(
                crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType::CSO
            )
        )
        .with_local_generation(true)
        .with_grid_critical(false)
        .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&limit).unwrap();

        // Deserialize back
        let deserialized: ChargingLimitType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(limit, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }
}
