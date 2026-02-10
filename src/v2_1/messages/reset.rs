use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};

/// The type of reset that the Charging Station or EVSE should perform.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResetEnumType {
    Immediate,
    OnIdle,
    ImmediateAndResume,
}

/// The status indicating whether the Charging Station is able to perform the reset.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ResetStatusEnumType {
    Accepted,
    Rejected,
    Scheduled,
}

/// Request body for the Reset request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the type of reset that the Charging Station or EVSE should perform.
    #[serde(rename = "type")]
    pub reset_type: ResetEnumType,

    /// Optional. This contains the ID of a specific EVSE that needs to be reset, instead of the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,
}

/// Response body for the Reset response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This indicates whether the Charging Station is able to perform the reset.
    pub status: ResetStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
