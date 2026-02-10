use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingProfileType, CustomDataType, StatusInfoType},
    enumerations::ChargingProfileStatusEnumType,
};

/// Request to set a charging profile at the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. For TxDefaultProfile an evseId=0 applies the profile to each individual evse.
    /// For ChargingStationMaxProfile and ChargingStationExternalConstraints an evseId=0 contains
    /// an overall limit for the whole Charging Station.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. Charging Profile to be set at the Charging Station.
    pub charging_profile: ChargingProfileType,
}

/// Response to a SetChargingProfileRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    /// Optional. Custom data specific to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Returns whether the Charging Station has been able to process the message successfully.
    /// This does not guarantee the schedule will be followed to the letter.
    /// There might be other constraints the Charging Station may need to take into account.
    pub status: ChargingProfileStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
