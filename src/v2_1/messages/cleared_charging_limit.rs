use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::enumerations::ChargingLimitSourceEnumType;

/// Request to notify that a charging limit has been cleared.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitRequest {
    /// Required. Source of the charging limit.
    pub charging_limit_source: ChargingLimitSourceEnumType,

    /// Optional. EVSE Identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub evse_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ClearedChargingLimitRequest.
/// This response contains no fields other than the optional customData field,
/// because the request cannot be denied by the CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearedChargingLimitResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
