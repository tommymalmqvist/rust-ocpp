use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::der_control::{DERControlEnumType, DERControlStatusEnumType},
};

/// Request to clear DER control settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDERControlRequest {
    /// Required. True: clearing default DER controls. False: clearing scheduled controls.
    pub is_default: bool,

    /// Optional. Name of control settings to clear. Not used when control_id is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<DERControlEnumType>,

    /// Optional. Id of control setting to clear.
    /// When omitted all settings for control_type are cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub control_id: Option<String>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearDERControlRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearDERControlResponse {
    /// Required. Result of the clear operation.
    pub status: DERControlStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
