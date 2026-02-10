use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::ChargingProfilePurposeEnumType;

/// A ClearChargingProfileType is a filter for charging profiles to be cleared by ClearChargingProfileRequest.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    /// Specifies the id of the EVSE for which to clear charging profiles. An evseId of zero (0) specifies the charging profile for the overall Charging Station. Absence of this parameter means the clearing applies to all charging profiles that match the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Specifies to purpose of the charging profiles that will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Specifies the stackLevel for which charging profiles will be cleared, if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub stack_level: Option<i32>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ClearChargingProfileType {
    /// Creates a new `ClearChargingProfileType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ClearChargingProfileType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            evse_id: None,
            charging_profile_purpose: None,
            stack_level: None,
        }
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - ID of the EVSE for which to clear charging profiles
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse_id(mut self, evse_id: i32) -> Self {
        self.evse_id = Some(evse_id);
        self
    }

    /// Sets the charging profile purpose.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_purpose` - Purpose of the charging profiles to be cleared
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
    /// * `stack_level` - Stack level for which charging profiles will be cleared
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stack_level(mut self, stack_level: i32) -> Self {
        self.stack_level = Some(stack_level);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear charging profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EVSE ID.
    ///
    /// # Returns
    ///
    /// An optional EVSE ID
    pub fn evse_id(&self) -> Option<i32> {
        self.evse_id
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - ID of the EVSE for which to clear charging profiles, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_id(&mut self, evse_id: Option<i32>) -> &mut Self {
        self.evse_id = evse_id;
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
    /// * `charging_profile_purpose` - Purpose of the charging profiles to be cleared, or None to clear
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
    /// An optional stack level
    pub fn stack_level(&self) -> Option<i32> {
        self.stack_level
    }

    /// Sets the stack level.
    ///
    /// # Arguments
    ///
    /// * `stack_level` - Stack level for which charging profiles will be cleared, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stack_level(&mut self, stack_level: Option<i32>) -> &mut Self {
        self.stack_level = stack_level;
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
    /// * `custom_data` - Custom data for this clear charging profile, or None to clear
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
    use serde_json::{from_str, to_string};
    use validator::Validate;

    #[test]
    fn test_new_clear_charging_profile() {
        let profile = ClearChargingProfileType::new();

        assert_eq!(profile.evse_id(), None);
        assert_eq!(profile.charging_profile_purpose(), None);
        assert_eq!(profile.stack_level(), None);
        assert_eq!(profile.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let profile = ClearChargingProfileType::new()
            .with_evse_id(1)
            .with_charging_profile_purpose(
                ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            )
            .with_stack_level(3)
            .with_custom_data(custom_data.clone());

        assert_eq!(profile.evse_id(), Some(1));
        assert_eq!(
            profile.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::ChargingStationExternalConstraints)
        );
        assert_eq!(profile.stack_level(), Some(3));
        assert_eq!(profile.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut profile = ClearChargingProfileType::new();

        profile
            .set_evse_id(Some(2))
            .set_charging_profile_purpose(Some(ChargingProfilePurposeEnumType::TxDefaultProfile))
            .set_stack_level(Some(4))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(profile.evse_id(), Some(2));
        assert_eq!(
            profile.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxDefaultProfile)
        );
        assert_eq!(profile.stack_level(), Some(4));
        assert_eq!(profile.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        profile
            .set_evse_id(None)
            .set_charging_profile_purpose(None)
            .set_stack_level(None)
            .set_custom_data(None);

        assert_eq!(profile.evse_id(), None);
        assert_eq!(profile.charging_profile_purpose(), None);
        assert_eq!(profile.stack_level(), None);
        assert_eq!(profile.custom_data(), None);
    }

    #[test]
    fn test_serialization_deserialization() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let profile = ClearChargingProfileType::new()
            .with_evse_id(1)
            .with_charging_profile_purpose(
                ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            )
            .with_stack_level(3)
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = to_string(&profile).unwrap();

        // Verify JSON contains expected fields
        assert!(serialized.contains(r#""evseId":1"#));
        assert!(
            serialized.contains(r#""chargingProfilePurpose":"ChargingStationExternalConstraints""#)
        );
        assert!(serialized.contains(r#""stackLevel":3"#));
        assert!(serialized.contains(r#""vendorId":"VendorX""#));

        // Deserialize back
        let deserialized: ClearChargingProfileType = from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.evse_id(), profile.evse_id());
        assert_eq!(
            deserialized.charging_profile_purpose(),
            profile.charging_profile_purpose()
        );
        assert_eq!(deserialized.stack_level(), profile.stack_level());
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            custom_data.vendor_id()
        );
    }

    #[test]
    fn test_validation() {
        // Valid profile
        let valid_profile = ClearChargingProfileType::new()
            .with_evse_id(1)
            .with_stack_level(3);
        assert!(
            valid_profile.validate().is_ok(),
            "Valid profile should pass validation"
        );

        // Test evse_id validation (negative value)
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.evse_id = Some(-1); // Invalid: must be >= 0
        assert!(
            invalid_profile.validate().is_err(),
            "Profile with negative evse_id should fail validation"
        );

        // Test stack_level validation (negative value)
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.stack_level = Some(-1); // Invalid: must be >= 0
        assert!(
            invalid_profile.validate().is_err(),
            "Profile with negative stack_level should fail validation"
        );

        // Test with all fields None (should be valid)
        let empty_profile = ClearChargingProfileType::new();
        assert!(
            empty_profile.validate().is_ok(),
            "Profile with all None fields should pass validation"
        );
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero values (should be valid)
        let zero_profile = ClearChargingProfileType::new()
            .with_evse_id(0)
            .with_stack_level(0);
        assert!(
            zero_profile.validate().is_ok(),
            "Profile with zero values should pass validation"
        );

        // Test with maximum integer values
        let max_profile = ClearChargingProfileType::new()
            .with_evse_id(i32::MAX)
            .with_stack_level(i32::MAX);
        assert!(
            max_profile.validate().is_ok(),
            "Profile with maximum integer values should pass validation"
        );
    }

    #[test]
    fn test_all_charging_profile_purpose_enum_values() {
        // Test with each enum value
        let purposes = vec![
            ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            ChargingProfilePurposeEnumType::ChargingStationMaxProfile,
            ChargingProfilePurposeEnumType::TxDefaultProfile,
            ChargingProfilePurposeEnumType::TxProfile,
        ];

        for purpose in purposes {
            let profile =
                ClearChargingProfileType::new().with_charging_profile_purpose(purpose.clone());

            assert_eq!(
                profile.charging_profile_purpose(),
                Some(&purpose),
                "Profile should have the correct charging profile purpose"
            );

            // Serialize and deserialize
            let serialized = to_string(&profile).unwrap();
            let deserialized: ClearChargingProfileType = from_str(&serialized).unwrap();

            assert_eq!(
                deserialized.charging_profile_purpose(),
                Some(&purpose),
                "Deserialized profile should have the correct charging profile purpose"
            );
        }
    }

    #[test]
    fn test_complex_scenario() {
        // Create a profile with custom data
        let custom_data = CustomDataType::new("VendorX".to_string());

        let profile = ClearChargingProfileType::new()
            .with_evse_id(1)
            .with_charging_profile_purpose(ChargingProfilePurposeEnumType::TxProfile)
            .with_stack_level(3)
            .with_custom_data(custom_data.clone());

        // Validate the complex object
        assert!(
            profile.validate().is_ok(),
            "Complex profile should pass validation"
        );

        // Serialize and deserialize
        let serialized = to_string(&profile).unwrap();
        let deserialized: ClearChargingProfileType = from_str(&serialized).unwrap();

        // Verify all fields are preserved
        assert_eq!(deserialized.evse_id(), Some(1));
        assert_eq!(
            deserialized.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxProfile)
        );
        assert_eq!(deserialized.stack_level(), Some(3));
        assert_eq!(deserialized.custom_data().unwrap().vendor_id(), "VendorX");
    }

    #[test]
    fn test_partial_fields() {
        // Test with only evse_id
        let profile_with_evse = ClearChargingProfileType::new().with_evse_id(1);

        assert_eq!(profile_with_evse.evse_id(), Some(1));
        assert_eq!(profile_with_evse.charging_profile_purpose(), None);
        assert_eq!(profile_with_evse.stack_level(), None);

        // Test with only charging_profile_purpose
        let profile_with_purpose = ClearChargingProfileType::new()
            .with_charging_profile_purpose(ChargingProfilePurposeEnumType::TxDefaultProfile);

        assert_eq!(profile_with_purpose.evse_id(), None);
        assert_eq!(
            profile_with_purpose.charging_profile_purpose(),
            Some(&ChargingProfilePurposeEnumType::TxDefaultProfile)
        );
        assert_eq!(profile_with_purpose.stack_level(), None);

        // Test with only stack_level
        let profile_with_stack = ClearChargingProfileType::new().with_stack_level(3);

        assert_eq!(profile_with_stack.evse_id(), None);
        assert_eq!(profile_with_stack.charging_profile_purpose(), None);
        assert_eq!(profile_with_stack.stack_level(), Some(3));

        // Validate all partial profiles
        assert!(
            profile_with_evse.validate().is_ok(),
            "Profile with only evse_id should pass validation"
        );
        assert!(
            profile_with_purpose.validate().is_ok(),
            "Profile with only charging_profile_purpose should pass validation"
        );
        assert!(
            profile_with_stack.validate().is_ok(),
            "Profile with only stack_level should pass validation"
        );
    }
}
