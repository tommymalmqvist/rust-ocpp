use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, NetworkConnectionProfileType, StatusInfoType},
    enumerations::SetNetworkProfileStatusEnumType,
};

/// Request to configure network connection profiles on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Slot in which the configuration should be stored.
    #[validate(range(min = 0))]
    pub configuration_slot: i32,

    /// Required. Network connection profile details.
    pub connection_data: NetworkConnectionProfileType,
}

/// Response to a SetNetworkProfileRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetNetworkProfileResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Result of operation.
    pub status: SetNetworkProfileStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
