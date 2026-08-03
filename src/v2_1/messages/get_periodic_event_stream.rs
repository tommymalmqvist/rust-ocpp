#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::v2_1::datatypes::{ConstantStreamDataType, CustomDataType};

/// Request to get information about periodic event streams.
/// This message is empty except for the optional customData field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetPeriodicEventStreamRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetPeriodicEventStreamResponse {
    /// Optional. List of constant stream data elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "std", validate(length(min = 1)))]
    pub constant_stream_data: Option<Vec<ConstantStreamDataType>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
