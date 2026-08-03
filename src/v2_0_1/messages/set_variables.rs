use crate::v2_0_1::datatypes::set_variable_data_type::SetVariableDataType;
use crate::v2_0_1::datatypes::set_variable_result_type::SetVariableResultType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// This contains the field definition of the SetVariablesRequest PDU sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    pub set_variable_data: Vec<SetVariableDataType>,
}

/// This contains the field definition of the SetVariablesResponse PDU sent by the Charging Station to the CSMS in response to a SetVariablesRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    pub set_variable_result: Vec<SetVariableResultType>,
}
