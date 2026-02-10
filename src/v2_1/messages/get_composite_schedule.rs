use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CompositeScheduleType, CustomDataType, StatusInfoType},
    enumerations::{ChargingRateUnitEnumType, GenericStatusEnumType},
};

/// Request to get a composite charging schedule from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleRequest {
    /// Required. Length of the requested schedule in seconds.
    pub duration: i32,

    /// Required. The ID of the EVSE for which the schedule is requested.
    /// When evseid=0, the Charging Station will calculate the expected consumption
    /// for the grid connection.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Optional. Can be used to force a power or current profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_rate_unit: Option<ChargingRateUnitEnumType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetCompositeScheduleRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCompositeScheduleResponse {
    /// Required. The Charging Station will indicate if it was able to process the request.
    pub status: GenericStatusEnumType,

    /// Optional. The composite schedule that applies to the selected EVSE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CompositeScheduleType>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
