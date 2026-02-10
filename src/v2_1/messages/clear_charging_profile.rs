use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{ChargingProfilePurposeEnumType, ClearChargingProfileStatusEnumType},
};

/// A ClearChargingProfileType is a filter for charging profiles to be cleared by ClearChargingProfileRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileType {
    /// Optional. Specifies the id of the EVSE for which to clear charging profiles.
    /// An evseId of zero (0) specifies the charging profile for the overall Charging Station.
    /// Absence of this parameter means the clearing applies to all charging profiles that match
    /// the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Optional. Specifies to purpose of the charging profiles that will be cleared,
    /// if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeEnumType>,

    /// Optional. Specifies the stackLevel for which charging profiles will be cleared,
    /// if they meet the other criteria in the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub stack_level: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to clear charging profiles from a charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    /// Optional. The Id of the charging profile to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_id: Option<i32>,

    /// Optional. Charging profile criteria to use for clearing profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_criteria: Option<ClearChargingProfileType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearChargingProfileRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileResponse {
    /// Required. Indicates if the Charging Station was able to execute the request.
    pub status: ClearChargingProfileStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
