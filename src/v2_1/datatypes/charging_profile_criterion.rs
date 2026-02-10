use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::CustomDataType,
    enumerations::{ChargingLimitSourceEnumType, ChargingProfilePurposeEnumType},
};

/// A ChargingProfileCriterionType is a filter for charging profiles to be selected by a GetChargingProfilesRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    /// Defines the purpose of the schedule transferred by this profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub stack_level: Option<i32>,

    /// List of all the chargingProfileIds requested. Any ChargingProfile that matches one of these profiles will be reported.
    /// If omitted, the Charging Station SHALL not filter on chargingProfileId.
    /// This field SHALL NOT contain more ids than set in ChargingProfileEntries.maxLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub charging_profile_id: Option<Vec<i32>>,

    /// For which charging limit sources, charging profiles SHALL be reported. If omitted, the Charging Station SHALL not filter on chargingLimitSource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_limit_source: Option<ChargingLimitSourceEnumType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingProfileCriterionType {
    /// Creates a new `ChargingProfileCriterionType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingProfileCriterionType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            charging_profile_purpose: None,
            stack_level: None,
            charging_profile_id: None,
            charging_limit_source: None,
        }
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Defines the purpose of the schedule transferred by this profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_profile_purpose(
        mut self,
        charging_profile_purpose: ChargingProfilePurposeEnumType,
    ) -> Self {
        self.charging_profile_purpose = Some(charging_profile_purpose);
        self
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Value determining level in hierarchy stack of profiles
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stack_level(mut self, stack_level: i32) -> Self {
        self.stack_level = Some(stack_level);
        self
    }

    /// Sets the charging profile IDs.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_id` - List of all the chargingProfileIds requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_profile_id(mut self, charging_profile_id: Vec<i32>) -> Self {
        self.charging_profile_id = Some(charging_profile_id);
        self
    }

    /// Sets the charging limit source.
    ///
    /// # Arguments
    ///
    /// * `charging_limit_source` - Charging limit source to filter on
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_limit_source(
        mut self,
        charging_limit_source: ChargingLimitSourceEnumType,
    ) -> Self {
        self.charging_limit_source = Some(charging_limit_source);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging profile criterion
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the charging profile purpose.
    ///
    /// # Returns
    ///
    /// An optional reference to the charging profile purpose
    pub fn charging_profile_purpose(&self) -> Option<&ChargingProfilePurposeEnumType> {
        self.charging_profile_purpose.as_ref()
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Defines the purpose of the schedule transferred by this profile, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_profile_purpose(
        &mut self,
        charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    ) -> &mut Self {
        self.charging_profile_purpose = charging_profile_purpose;
        self
    }

    /// Gets the stack level.
    ///
    /// # Returns
    ///
    /// An optional stack level value
    pub fn stack_level(&self) -> Option<i32> {
        self.stack_level
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Value determining level in hierarchy stack of profiles, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stack_level(&mut self, stack_level: Option<i32>) -> &mut Self {
        self.stack_level = stack_level;
        self
    }

    /// Gets the charging profile IDs.
    ///
    /// # Returns
    ///
    /// An optional reference to the list of charging profile IDs
    pub fn charging_profile_id(&self) -> Option<&Vec<i32>> {
        self.charging_profile_id.as_ref()
    }

    /// Sets the charging profile IDs.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_id` - List of all the chargingProfileIds requested, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_profile_id(&mut self, charging_profile_id: Option<Vec<i32>>) -> &mut Self {
        self.charging_profile_id = charging_profile_id;
        self
    }

    /// Gets the charging limit source.
    ///
    /// # Returns
    ///
    /// An optional reference to the charging limit source
    pub fn charging_limit_source(&self) -> Option<&ChargingLimitSourceEnumType> {
        self.charging_limit_source.as_ref()
    }

    /// Sets the charging limit source.
    ///
    /// # Arguments
    ///
    /// * `charging_limit_source` - Charging limit source to filter on, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_limit_source(
        &mut self,
        charging_limit_source: Option<ChargingLimitSourceEnumType>,
    ) -> &mut Self {
        self.charging_limit_source = charging_limit_source;
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
    /// * `custom_data` - Custom data for this charging profile criterion, or None to clear
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

    #[test]
    fn test_new_charging_profile_criterion() {
        let criterion = ChargingProfileCriterionType::new();

        assert_eq!(criterion.charging_profile_purpose(), None);
        assert_eq!(criterion.stack_level(), None);
        assert_eq!(criterion.charging_profile_id(), None);
        assert_eq!(criterion.charging_limit_source(), None);
        assert_eq!(criterion.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        use crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType;

        let custom_data = CustomDataType::new("VendorX".to_string());
        let limit_source =
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::EMS);

        let criterion = ChargingProfileCriterionType::new()
            .with_charging_profile_purpose(
                ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            )
            .with_stack_level(3)
            .with_charging_profile_id(vec![1, 2, 3])
            .with_charging_limit_source(limit_source.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(
            criterion.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::ChargingStationExternalConstraints)
        );
        assert_eq!(criterion.stack_level(), Some(3));
        assert_eq!(criterion.charging_profile_id(), Some(&vec![1, 2, 3]));
        assert_eq!(criterion.charging_limit_source(), Some(&limit_source));
        assert_eq!(criterion.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        use crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType;

        let custom_data = CustomDataType::new("VendorX".to_string());
        let limit_source =
            ChargingLimitSourceEnumType::Standard(StandardChargingLimitSourceEnumType::SO);

        let mut criterion = ChargingProfileCriterionType::new();

        criterion
            .set_charging_profile_purpose(Some(ChargingProfilePurposeEnumType::TxDefaultProfile))
            .set_stack_level(Some(2))
            .set_charging_profile_id(Some(vec![4, 5, 6]))
            .set_charging_limit_source(Some(limit_source.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            criterion.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxDefaultProfile)
        );
        assert_eq!(criterion.stack_level(), Some(2));
        assert_eq!(criterion.charging_profile_id(), Some(&vec![4, 5, 6]));
        assert_eq!(criterion.charging_limit_source(), Some(&limit_source));
        assert_eq!(criterion.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        criterion
            .set_charging_profile_purpose(None)
            .set_stack_level(None)
            .set_charging_profile_id(None)
            .set_charging_limit_source(None)
            .set_custom_data(None);

        assert_eq!(criterion.charging_profile_purpose(), None);
        assert_eq!(criterion.stack_level(), None);
        assert_eq!(criterion.charging_profile_id(), None);
        assert_eq!(criterion.charging_limit_source(), None);
        assert_eq!(criterion.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        use crate::v2_1::enumerations::charging_limit_source::StandardChargingLimitSourceEnumType;

        // 1. Valid criterion with all fields - should pass validation
        let custom_data = CustomDataType::new("VendorX".to_string());
        let valid_criterion = ChargingProfileCriterionType::new()
            .with_charging_profile_purpose(ChargingProfilePurposeEnumType::TxProfile)
            .with_stack_level(3)
            .with_charging_profile_id(vec![1, 2, 3])
            .with_charging_limit_source(ChargingLimitSourceEnumType::Standard(
                StandardChargingLimitSourceEnumType::EMS,
            ))
            .with_custom_data(custom_data);

        assert!(
            valid_criterion.validate().is_ok(),
            "Valid criterion should pass validation"
        );

        // 2. Test stack_level validation (negative value)
        let mut invalid_stack_level = ChargingProfileCriterionType::new();
        invalid_stack_level.stack_level = Some(-1); // Invalid: must be >= 0

        let validation_result = invalid_stack_level.validate();
        assert!(
            validation_result.is_err(),
            "Criterion with negative stack_level should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("stack_level"),
            "Error should mention stack_level: {}",
            error
        );

        // 3. Test charging_profile_id validation (empty array)
        let mut invalid_profile_id = ChargingProfileCriterionType::new();
        invalid_profile_id.charging_profile_id = Some(vec![]); // Invalid: must have at least 1 element

        let validation_result = invalid_profile_id.validate();
        assert!(
            validation_result.is_err(),
            "Criterion with empty charging_profile_id should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("charging_profile_id"),
            "Error should mention charging_profile_id: {}",
            error
        );

        // 4. Test with invalid custom data (nested validation)
        let invalid_custom_data = CustomDataType::new("a".repeat(256)); // Exceeds max length of 255
        let invalid_criterion =
            ChargingProfileCriterionType::new().with_custom_data(invalid_custom_data);

        assert!(
            invalid_criterion.validate().is_err(),
            "Criterion with invalid custom_data should fail validation"
        );
    }
}
