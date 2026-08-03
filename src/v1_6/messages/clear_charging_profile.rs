use crate::v1_6::types::{ChargingProfilePurposeType, ClearChargingProfileStatus};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClearChargingProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_profile_purpose: Option<ChargingProfilePurposeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_level: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
pub struct ClearChargingProfileResponse {
    /// This contains the field definition of the ClearChargingProfile.conf PDU sent by the Charge Point to the Central
    /// System in response to a ClearChargingProfile.req PDU. See also Clear Charging Profile
    pub status: ClearChargingProfileStatus,
}
