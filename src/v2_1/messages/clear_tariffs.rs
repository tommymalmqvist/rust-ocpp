use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::TariffClearStatusEnumType;

/// Result of clearing a specific tariff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResultType {
    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Id of tariff for which status is reported.
    /// If no tariffs were found, then this field is absent, and status will be NoTariff.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Required. Status indicating whether the Charging Station was able to clear the tariff.
    pub status: TariffClearStatusEnumType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to clear tariffs from a charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsRequest {
    /// Optional. List of tariff Ids to clear.
    /// When absent clears all tariffs at evse_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub tariff_ids: Option<Vec<String>>,

    /// Optional. When present only clear tariffs matching tariff_ids at EVSE evse_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearTariffsRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResponse {
    /// Required. List of results for each tariff that was cleared or attempted to be cleared.
    #[validate(length(min = 1))]
    pub clear_tariffs_result: Vec<ClearTariffsResultType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
