use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::GenericStatusEnumType,
};

/// Request to set the monitoring level at the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Charging Station SHALL only report events with a severity number lower than or equal to this severity.
    /// The severity range is 0-9, with 0 as the highest and 9 as the lowest severity level.
    #[validate(range(min = 0, max = 9))]
    pub severity: i32,
}

/// Response to a SetMonitoringLevelRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringLevelResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Indicates whether the Charging Station was able to accept the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
