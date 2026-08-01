use crate::v2_0_1::datatypes::charging_profile_type::ChargingProfileType;
use crate::v2_0_1::enumerations::charging_limit_source_enum_type::ChargingLimitSourceEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
/// Reports charging profiles installed in the Charging Station, as requested via a GetChargingProfilesRequest message. The charging profile report can be split over multiple ReportChargingProfilesRequest messages, this can be because charging profiles for different charging sources need to be reported, or because there is just to much data for one message.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesRequest {
    pub request_id: i32,
    pub charging_limit_source: ChargingLimitSourceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub evse_id: i32,
    pub charging_profile: Vec<ChargingProfileType>,
}

/// The ReportChargingProfilesResponse message is sent by the CSMS to the Charging Station in response to a ReportChargingProfilesRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportChargingProfilesResponse {}
