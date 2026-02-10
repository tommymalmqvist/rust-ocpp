use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ChargingProfileCriterionType, CustomDataType, StatusInfoType},
    enumerations::GetChargingProfileStatusEnumType,
};

/// Request to get the charging profiles installed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesRequest {
    /// Required. Reference identification that is to be used by the Charging Station
    /// in the ReportChargingProfilesRequest when provided.
    pub request_id: i32,

    /// Required. Specifies the charging profile criteria to be used for selecting
    /// charging profiles to report.
    pub charging_profile: ChargingProfileCriterionType,

    /// Optional. For which EVSE installed charging profiles SHALL be reported.
    /// If 0, only charging profiles installed on the Charging Station itself (the grid connection)
    /// SHALL be reported. If omitted, all installed charging profiles SHALL be reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetChargingProfilesRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetChargingProfilesResponse {
    /// Required. This indicates whether the Charging Station is able to process this request
    /// and will send ReportChargingProfilesRequest messages.
    pub status: GetChargingProfileStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
