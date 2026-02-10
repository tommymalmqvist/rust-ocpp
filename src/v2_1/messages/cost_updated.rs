use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Request to notify the Charging Station about updated cost for the current transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// Required. Current total cost, based on the information known by the CSMS,
    /// of the transaction including taxes. In the currency configured with the
    /// configuration Variable: Currency.
    pub total_cost: f64,

    /// Required. Transaction Id of the transaction the current cost are asked for.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a CostUpdatedRequest.
/// This response contains no fields other than the optional customData field,
/// because the request cannot be denied by the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
