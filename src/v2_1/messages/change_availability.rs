use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{ChangeAvailabilityStatusEnumType, OperationalStatusEnumType};

/// Electric Vehicle Supply Equipment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVSEType {
    /// EVSE Identifier. This contains a number (> 0) designating an EVSE of the Charging Station.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Optional. An id to designate a specific connector (on an EVSE) by connector index number.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub connector_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to change the availability of a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityRequest {
    /// Optional. Electric Vehicle Supply Equipment to change availability for.
    /// If no EVSE is specified, the message refers to the entire Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,

    /// Required. This contains the type of availability change that the Charging Station should perform.
    pub operational_status: OperationalStatusEnumType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ChangeAvailabilityRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAvailabilityResponse {
    /// Required. This indicates whether the Charging Station is able to perform the availability change.
    pub status: ChangeAvailabilityStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
