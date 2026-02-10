use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ChargingLimitType, ChargingScheduleType, CustomDataType};

/// Request to notify the CSMS about charging limits that are set by an external system on the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitRequest {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The EVSE to which the charging limit is set. If absent or when zero, it applies to the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Contains limits for the available power or current over time, as set by the external source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_schedule: Option<Vec<ChargingScheduleType>>,

    /// This contains the source of the charging limit and whether it is grid critical.
    pub charging_limit: ChargingLimitType,
}

/// Response to a NotifyChargingLimitRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyChargingLimitResponse {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
