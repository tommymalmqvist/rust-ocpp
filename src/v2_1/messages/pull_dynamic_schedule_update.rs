use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingScheduleUpdateType, CustomDataType, StatusInfoType},
    enumerations::ChargingProfileStatusEnumType,
};

/// Request body for the PullDynamicScheduleUpdate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Id of charging profile to update.
    pub charging_profile_id: i32,
}

/// Response body for the PullDynamicScheduleUpdate response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PullDynamicScheduleUpdateResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Updates to a ChargingSchedulePeriodType for dynamic charging profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_update: Option<ChargingScheduleUpdateType>,

    /// Required. Result of request.
    pub status: ChargingProfileStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
