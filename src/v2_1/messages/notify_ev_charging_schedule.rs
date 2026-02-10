use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ChargingScheduleType, CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::GenericStatusEnumType;

/// Request to notify the CSMS about an EV charging schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleRequest {
    /// Required. Periods contained in the charging profile are relative to this point in time.
    pub time_base: DateTime<Utc>,

    /// Required. The charging schedule contained in this notification applies to an EVSE. EvseId must be > 0.
    #[validate(range(min = 1))]
    pub evse_id: i32,

    /// Required. Charging schedule structure defines a list of charging periods.
    pub charging_schedule: ChargingScheduleType,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyEVChargingScheduleRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingScheduleResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
