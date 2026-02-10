use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Request to get the version number of the local authorization list in the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetLocalListVersionRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetLocalListVersionResponse {
    /// Required. This contains the current version number of the local authorization list
    /// in the Charging Station.
    pub version_number: i32,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
