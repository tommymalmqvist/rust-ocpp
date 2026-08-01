//! A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval
use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;
use crate::v2_0_1::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval.
///
/// ChargingProfileCriterionType is used by: get_charging_profiles [GetChargingProfilesRequest](`crate::v2_0_1::messages::get_charging_profiles::GetChargingProfilesRequest`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileCriterionType {
    /// Optional. Defines the purpose of the schedule transferred by this profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,
    /// Optional. Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
    /// Optional. List of all the chargingProfileIds requested. Any ChargingProfile that matches one of these profiles will be reported. If omitted, the Charging Station SHALL not filter on chargingProfileId. This field SHALL NOT contain more ids than set in ChargingProfileEntries.maxLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<Vec<i32>>,
    /// Optional. For which charging limit sources, charging profiles SHALL be reported. If omitted, the Charging Station SHALL not filter on chargingLimitSource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_limit_source: Option<Vec<ChargingLimitSourceEnumType>>,
}
