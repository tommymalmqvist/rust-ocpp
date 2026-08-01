use serde::{Deserialize, Serialize};

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::GenericStatusEnumType,
};

/// Parameters for the periodic event stream.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct PeriodicEventStreamParamsType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Time in seconds after which stream data is sent.
    #[cfg_attr(feature = "std", validate(range(min = 0)))]
    pub interval: i32,

    /// Number of items to be sent together in stream.
    #[cfg_attr(feature = "std", validate(range(min = 0)))]
    pub values: i32,
}

/// Request body for the AdjustPeriodicEventStream request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The identifier of the periodic event stream.
    #[cfg_attr(feature = "std", validate(range(min = 0)))]
    pub id: i32,

    /// Required. Parameters for the periodic event stream.
    pub params: PeriodicEventStreamParamsType,
}

/// Response body for the AdjustPeriodicEventStream response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AdjustPeriodicEventStreamResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
