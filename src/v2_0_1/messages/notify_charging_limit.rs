//! NotifyChargingLimit

use crate::v2_0_1::datatypes::charging_limit_type::ChargingLimitType;
use crate::v2_0_1::datatypes::charging_schedule_type::ChargingScheduleType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// The message NotifyChargingLimitRequest can be used to communicate a charging limit, set by an external system on the Charging Station (Not installed by the CSO via SetChargingProfileRequest), to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    /// The charging schedule contained in thisnotification applies to an EVSE. evseId must be > 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    /// This contains the source of the charging limitand whether it is grid critical.
    pub charging_limit: ChargingLimitType,
    /// Contains limits for the available power orcurrent over time, as set by the external source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,
}

/// The NotifyChargingLimitResponse message is sent by the CSMS to the Charging Station in response to a NotifyChargingLimitsRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitResponse {
    // No fields are defined
}
