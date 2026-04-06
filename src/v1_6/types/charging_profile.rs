use chrono::{DateTime, Utc};

use super::{
    ChargingProfileKindType, ChargingProfilePurposeType, ChargingSchedule, RecurrencyKindType,
};
use crate::v1_6::helpers::datetime_rfc3339;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    pub charging_profile_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    pub stack_level: u32,
    pub charging_profile_purpose: ChargingProfilePurposeType,
    pub charging_profile_kind: ChargingProfileKindType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "datetime_rfc3339::option")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "datetime_rfc3339::option")]
    pub valid_to: Option<DateTime<Utc>>,
    pub charging_schedule: ChargingSchedule,
}
