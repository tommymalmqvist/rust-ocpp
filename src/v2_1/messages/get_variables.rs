#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use validator::Validate;

use crate::v2_1::datatypes::{
    custom_data::CustomDataType, get_variable_data::GetVariableDataType,
    get_variable_result::GetVariableResultType,
};

/// GetVariablesRequest, sent by the CSMS to the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of requested variables.
    #[cfg_attr(feature = "std", validate(length(min = 1), nested))]
    pub get_variable_data: Vec<GetVariableDataType>,
}

/// GetVariablesResponse, sent by the Charging Station to the CSMS in response to GetVariablesRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of requested variables and their values.
    #[cfg_attr(feature = "std", validate(length(min = 1), nested))]
    pub get_variable_result: Vec<GetVariableResultType>,
}
