use chrono::DateTime;
use chrono::Utc;

use super::charging_schedule_type::ChargingScheduleType;
use crate::v2_0_1::enumerations::charging_profile_kind_enum_type::ChargingProfileKindEnumType;
use crate::v2_0_1::enumerations::charging_profile_purpose_enum_type::ChargingProfilePurposeEnumType;
use crate::v2_0_1::enumerations::recurrency_kind_enum_type::RecurrencyKindEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;
use validator::Validate;
/// A ChargingProfile consists of ChargingSchedule, describing the amount of power or current that can be delivered per time interval
/// ChargingProfileType is used by: RequestStartTransactionRequest , SetChargingProfileRequest , ReportChargingProfilesRequest
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfileType {
    /// Required. Id of ChargingProfile.
    pub id: i32,
    /// Required. Value determining level in hierarchy stack of profiles. Higher values have precedence over lower values. Lowest level is 0.
    pub stack_level: i32,
    /// Required. Defines the purpose of the schedule transferred by this profile
    pub charging_profile_purpose: ChargingProfilePurposeEnumType,
    /// Required. Indicates the kind of schedule.
    pub charging_profile_kind: ChargingProfileKindEnumType,
    /// Optional. Indicates the start point of a recurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnumType>,
    /// Optional. Point in time at which the profile starts to be valid. If absent, the profile is valid as soon as it is received by the Charging Station
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub valid_from: Option<DateTime<Utc>>,
    /// Optional. Point in time at which the profile stops to be valid. If absent, the profile is valid until it is replaced by another profile
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub valid_to: Option<DateTime<Utc>>,
    /// Optional. SHALL only be included if ChargingProfilePurpose is set to TxProfile. The transactionId is used to match the profile to a specific transaction
    #[validate(length(min = 0, max = 36))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Required. Schedule that contains limits for the available power or current over time. In order to support ISO 15118 schedule negotiation, it supports at most three schedules with associated tariff to choose from
    pub charging_schedule: Vec<ChargingScheduleType>,
}
