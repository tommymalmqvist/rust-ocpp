//! GetVariables

use crate::v2_0_1::datatypes::get_variable_data_type::GetVariableDataType;
use crate::v2_0_1::datatypes::get_variable_result_type::GetVariableResultType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// GetVariablesRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesRequest {
    /// List of requested variables.
    pub get_variable_data: Vec<GetVariableDataType>,
}

/// GetVariablesResponse, sent by the CSMS to the Charging Station in response to GetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetVariablesResponse {
    /// List of requested variables and their values.
    pub get_variable_result: Vec<GetVariableResultType>,
}
