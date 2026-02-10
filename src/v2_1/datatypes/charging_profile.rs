use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{charging_schedule::ChargingScheduleType, CustomDataType},
    enumerations::{
        ChargingProfileKindEnumType, ChargingProfilePurposeEnumType, RecurrencyKindEnumType,
    },
};

/// A ChargingProfile consists of 1 to 3 ChargingSchedules with a list of ChargingSchedulePeriods,
/// describing the amount of power or current that can be delivered per time interval.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    /// Id of ChargingProfile. Unique within charging station. Id can have a negative value.
    /// This is useful to distinguish charging profiles from an external actor (external constraints)
    /// from charging profiles received from CSMS.
    pub id: i32,

    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values.
    /// Lowest level is 0.
    #[validate(range(min = 0))]
    pub stack_level: i32,

    /// Defines the purpose of the schedule transferred by this profile
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,

    /// Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKindEnumType,

    /// Indicates the start point of a recurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,

    /// Point in time at which the profile starts to be valid.
    /// If absent, the profile is valid as soon as it is received by the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Point in time at which the profile stops to be valid.
    /// If absent, the profile is valid until it is replaced by another profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,

    /// SHALL only be included if ChargingProfilePurpose is set to TxProfile.
    /// The transactionId is used to match the profile to a specific transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Period in seconds that this charging profile remains valid after the Charging Station has gone offline. After this period the charging profile becomes invalid for as long as it is offline and the Charging Station reverts back to a valid profile with a lower stack level. \r\nIf _invalidAfterOfflineDuration_ is true, then this charging profile will become permanently invalid.\r\nA value of 0 means that the charging profile is immediately invalid while offline. When the field is absent, then  no timeout applies and the charging profile remains valid when offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_offline_duration: Option<i32>,

    /// When set to true this charging profile will not be valid anymore after being offline for more than _maxOfflineDuration_. When absent defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_after_offline_duration: Option<bool>,

    /// Interval in seconds after receipt of last update, when to request a profile update by sending a PullDynamicScheduleUpdateRequest message.\r\n    A value of 0 or no value means that no update interval applies. +\r\n    Only relevant in a dynamic charging profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyn_update_interval: Option<i32>,

    /// Time at which limits or setpoints in this charging profile were last updated by a PullDynamicScheduleUpdateRequest or UpdateDynamicScheduleRequest or by an external actor. Only relevant in a dynamic charging profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyn_update_time: Option<DateTime<Utc>>,

    /// ISO 15118-20 signature for all price schedules in _chargingSchedules_. +\r\nNote: for 256-bit elliptic curves (like secp256k1) the ECDSA signature is 512 bits (64 bytes) and for 521-bit curves (like secp521r1) the signature is 1042 bits. This equals 131 bytes, which can be encoded as base64 in 176 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 176))]
    pub price_schedule_signature: Option<String>,

    /// Schedule that contains limits for the available
    /// power or current over time. In order to support ISO 15118
    /// schedule negotiation, it supports at most three schedules
    /// with associated tariff to choose from. Having multiple
    /// chargingSchedules is only allowed for charging profiles of
    /// purpose TxProfile in the context of an ISO 15118
    /// charging session. For ISO 15118 Dynamic Control Mode
    /// only one chargingSchedule shall be provided.
    #[validate(length(min = 1, max = 3), nested)]
    pub charging_schedule: Vec<ChargingScheduleType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingProfileType {
    /// Creates a new `ChargingProfileType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of ChargingProfile
    /// * `stack_level` - Value determining level in hierarchy stack of profiles
    /// * `charging_profile_purpose` - Defines the purpose of the schedule transferred by this profile
    /// * `charging_profile_kind` - Indicates the kind of schedule
    /// * `charging_schedule` - Contains limits for the available power or current over time
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingProfileType` with optional fields set to `None`
    pub fn new(
        id: i32,
        stack_level: i32,
        charging_profile_purpose: ChargingProfilePurposeEnumType,
        charging_profile_kind: ChargingProfileKindEnumType,
        charging_schedule: Vec<ChargingScheduleType>,
    ) -> Self {
        Self {
            id,
            stack_level,
            charging_profile_purpose,
            charging_profile_kind,
            charging_schedule,
            custom_data: None,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            transaction_id: None,
            max_offline_duration: None,
            invalid_after_offline_duration: None,
            dyn_update_interval: None,
            dyn_update_time: None,
            price_schedule_signature: None,
        }
    }

    /// Sets the recurrency kind.
    ///
    /// # Arguments
    ///
    /// * `recurrency_kind` - Indicates the start point of a recurrence
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_recurrency_kind(mut self, recurrency_kind: RecurrencyKindEnumType) -> Self {
        self.recurrency_kind = Some(recurrency_kind);
        self
    }

    /// Sets the valid from time.
    ///
    /// # Arguments
    ///
    /// * `valid_from` - Point in time at which the profile starts to be valid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_from(mut self, valid_from: DateTime<Utc>) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    /// Sets the valid to time.
    ///
    /// # Arguments
    ///
    /// * `valid_to` - Point in time at which the profile stops to be valid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_to(mut self, valid_to: DateTime<Utc>) -> Self {
        self.valid_to = Some(valid_to);
        self
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transactionId used to match the profile to a specific transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_transaction_id(mut self, transaction_id: String) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID of the charging profile.
    ///
    /// # Returns
    ///
    /// The ID of the charging profile
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID of the charging profile.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the charging profile
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the stack level.
    ///
    /// # Returns
    ///
    /// The stack level value
    pub fn stack_level(&self) -> i32 {
        self.stack_level
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
    pub fn set_stack_level(&mut self, stack_level: i32) -> &mut Self {
        self.stack_level = stack_level;
        self
    }

    /// Gets the charging profile purpose.
    ///
    /// # Returns
    ///
    /// The purpose of the schedule transferred by this profile
    pub fn charging_profile_purpose(&self) -> &ChargingProfilePurposeEnumType {
        &self.charging_profile_purpose
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
    pub fn set_charging_profile_purpose(
        &mut self,
        charging_profile_purpose: ChargingProfilePurposeEnumType,
    ) -> &mut Self {
        self.charging_profile_purpose = charging_profile_purpose;
        self
    }

    /// Gets the charging profile kind.
    ///
    /// # Returns
    ///
    /// The kind of schedule
    pub fn charging_profile_kind(&self) -> &ChargingProfileKindEnumType {
        &self.charging_profile_kind
    }

    /// Sets the charging profile kind.
    ///
    /// # Arguments
    ///
    /// * `charging_profile_kind` - Indicates the kind of schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_profile_kind(
        &mut self,
        charging_profile_kind: ChargingProfileKindEnumType,
    ) -> &mut Self {
        self.charging_profile_kind = charging_profile_kind;
        self
    }

    /// Gets the recurrency kind.
    ///
    /// # Returns
    ///
    /// An optional reference to the recurrency kind
    pub fn recurrency_kind(&self) -> Option<&RecurrencyKindEnumType> {
        self.recurrency_kind.as_ref()
    }

    /// Sets the recurrency kind.
    ///
    /// # Arguments
    ///
    /// * `recurrency_kind` - Indicates the start point of a recurrence, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_recurrency_kind(
        &mut self,
        recurrency_kind: Option<RecurrencyKindEnumType>,
    ) -> &mut Self {
        self.recurrency_kind = recurrency_kind;
        self
    }

    /// Gets the valid from time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time at which the profile starts to be valid
    pub fn valid_from(&self) -> Option<&DateTime<Utc>> {
        self.valid_from.as_ref()
    }

    /// Sets the valid from time.
    ///
    /// # Arguments
    ///
    /// * `valid_from` - Point in time at which the profile starts to be valid, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_from(&mut self, valid_from: Option<DateTime<Utc>>) -> &mut Self {
        self.valid_from = valid_from;
        self
    }

    /// Gets the valid to time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time at which the profile stops to be valid
    pub fn valid_to(&self) -> Option<&DateTime<Utc>> {
        self.valid_to.as_ref()
    }

    /// Sets the valid to time.
    ///
    /// # Arguments
    ///
    /// * `valid_to` - Point in time at which the profile stops to be valid, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_to(&mut self, valid_to: Option<DateTime<Utc>>) -> &mut Self {
        self.valid_to = valid_to;
        self
    }

    /// Gets the charging schedule.
    ///
    /// # Returns
    ///
    /// A reference to the charging schedule
    pub fn charging_schedule(&self) -> &Vec<ChargingScheduleType> {
        &self.charging_schedule
    }

    /// Sets the charging schedule.
    ///
    /// # Arguments
    ///
    /// * `charging_schedule` - Contains limits for the available power or current over time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_schedule(
        &mut self,
        charging_schedule: Vec<ChargingScheduleType>,
    ) -> &mut Self {
        self.charging_schedule = charging_schedule;
        self
    }

    /// Gets the transaction ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the transaction ID
    pub fn transaction_id(&self) -> Option<&String> {
        self.transaction_id.as_ref()
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - The transactionId used to match the profile to a specific transaction, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction_id(&mut self, transaction_id: Option<String>) -> &mut Self {
        self.transaction_id = transaction_id;
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
    /// * `custom_data` - Custom data for this charging profile, or None to clear
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
    use crate::v2_1::datatypes::charging_schedule_period::ChargingSchedulePeriodType;
    use crate::v2_1::enumerations::ChargingRateUnitEnumType;

    fn create_test_charging_schedule() -> ChargingScheduleType {
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);

        ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period])
    }

    #[test]
    fn test_new_charging_profile() {
        let schedule = create_test_charging_schedule();
        let profile = ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::ChargingStationExternalConstraints,
            ChargingProfileKindEnumType::Absolute,
            vec![schedule.clone()],
        );

        assert_eq!(profile.id(), 1);
        assert_eq!(profile.stack_level(), 0);
        assert_eq!(
            profile.charging_profile_purpose(),
            &ChargingProfilePurposeEnumType::ChargingStationExternalConstraints
        );
        assert_eq!(
            profile.charging_profile_kind(),
            &ChargingProfileKindEnumType::Absolute
        );
        assert_eq!(profile.charging_schedule().len(), 1);
        assert_eq!(profile.charging_schedule()[0].id, schedule.id);
        assert_eq!(profile.recurrency_kind(), None);
        assert_eq!(profile.valid_from(), None);
        assert_eq!(profile.valid_to(), None);
        assert_eq!(profile.transaction_id(), None);
        assert_eq!(profile.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let schedule = create_test_charging_schedule();
        let custom_data = CustomDataType::new("VendorX".to_string());
        let valid_from = Utc::now();
        let valid_to = valid_from + chrono::Duration::days(1);

        let profile = ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::TxProfile,
            ChargingProfileKindEnumType::Recurring,
            vec![schedule.clone()],
        )
        .with_recurrency_kind(RecurrencyKindEnumType::Daily)
        .with_valid_from(valid_from)
        .with_valid_to(valid_to)
        .with_transaction_id("tx-123".to_string())
        .with_custom_data(custom_data.clone());

        assert_eq!(profile.id(), 1);
        assert_eq!(profile.stack_level(), 0);
        assert_eq!(
            profile.charging_profile_purpose(),
            &ChargingProfilePurposeEnumType::TxProfile
        );
        assert_eq!(
            profile.charging_profile_kind(),
            &ChargingProfileKindEnumType::Recurring
        );
        assert_eq!(profile.charging_schedule().len(), 1);
        assert_eq!(profile.charging_schedule()[0].id, schedule.id);
        assert_eq!(
            profile.recurrency_kind(),
            Some(&RecurrencyKindEnumType::Daily)
        );
        assert_eq!(profile.valid_from(), Some(&valid_from));
        assert_eq!(profile.valid_to(), Some(&valid_to));
        assert_eq!(profile.transaction_id(), Some(&"tx-123".to_string()));
        assert_eq!(profile.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let schedule1 = create_test_charging_schedule();
        let schedule2 = ChargingScheduleType::new(
            2,
            ChargingRateUnitEnumType::W,
            vec![ChargingSchedulePeriodType::new_from_f64(0, 11000.0)],
        );

        let custom_data = CustomDataType::new("VendorX".to_string());
        let valid_from = Utc::now();
        let valid_to = valid_from + chrono::Duration::days(1);

        let mut profile = ChargingProfileType::new(
            1,
            0,
            ChargingProfilePurposeEnumType::ChargingStationMaxProfile,
            ChargingProfileKindEnumType::Absolute,
            vec![schedule1.clone()],
        );

        profile
            .set_id(2)
            .set_stack_level(1)
            .set_charging_profile_purpose(ChargingProfilePurposeEnumType::TxDefaultProfile)
            .set_charging_profile_kind(ChargingProfileKindEnumType::Recurring)
            .set_recurrency_kind(Some(RecurrencyKindEnumType::Daily))
            .set_valid_from(Some(valid_from))
            .set_valid_to(Some(valid_to))
            .set_charging_schedule(vec![schedule1.clone(), schedule2.clone()])
            .set_transaction_id(Some("tx-456".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(profile.id(), 2);
        assert_eq!(profile.stack_level(), 1);
        assert_eq!(
            profile.charging_profile_purpose(),
            &ChargingProfilePurposeEnumType::TxDefaultProfile
        );
        assert_eq!(
            profile.charging_profile_kind(),
            &ChargingProfileKindEnumType::Recurring
        );
        assert_eq!(profile.charging_schedule().len(), 2);
        assert_eq!(profile.charging_schedule()[0].id, schedule1.id);
        assert_eq!(profile.charging_schedule()[1].id, schedule2.id);
        assert_eq!(
            profile.recurrency_kind(),
            Some(&RecurrencyKindEnumType::Daily)
        );
        assert_eq!(profile.valid_from(), Some(&valid_from));
        assert_eq!(profile.valid_to(), Some(&valid_to));
        assert_eq!(profile.transaction_id(), Some(&"tx-456".to_string()));
        assert_eq!(profile.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        profile
            .set_recurrency_kind(None)
            .set_valid_from(None)
            .set_valid_to(None)
            .set_transaction_id(None)
            .set_custom_data(None);

        assert_eq!(profile.recurrency_kind(), None);
        assert_eq!(profile.valid_from(), None);
        assert_eq!(profile.valid_to(), None);
        assert_eq!(profile.transaction_id(), None);
        assert_eq!(profile.custom_data(), None);
    }
}
