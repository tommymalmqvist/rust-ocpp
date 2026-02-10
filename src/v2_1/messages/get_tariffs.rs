use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType, TariffAssignmentType};
use crate::v2_1::enumerations::TariffGetStatusEnumType;

/// Request to get tariff information from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsRequest {
    /// Required. EVSE id to get tariff from. When evseId = 0, this gets tariffs from all EVSEs.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetTariffsRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetTariffsResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: TariffGetStatusEnumType,

    /// Optional. List of tariff assignments.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub tariff_assignments: Option<Vec<TariffAssignmentType>>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
