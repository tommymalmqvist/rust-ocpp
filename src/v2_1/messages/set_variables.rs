use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ComponentType, CustomDataType, StatusInfoType, VariableType};

/// Type of attribute: Actual, Target, MinSet, MaxSet. Default is Actual when omitted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AttributeEnumType {
    Actual,
    Target,
    MinSet,
    MaxSet,
}

impl Default for AttributeEnumType {
    fn default() -> Self {
        Self::Actual
    }
}

/// Result status of setting the variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SetVariableStatusEnumType {
    Accepted,
    Rejected,
    UnknownComponent,
    UnknownVariable,
    NotSupportedAttributeType,
    RebootRequired,
}

/// Class to hold parameters for setting a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableDataType {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Type of attribute: Actual, Target, MinSet, MaxSet.
    /// Default is Actual when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,

    /// Required. Value to be assigned to attribute of variable.
    #[validate(length(max = 2500))]
    pub attribute_value: String,

    /// Required. Component for which the variable is set.
    pub component: ComponentType,

    /// Required. Variable which the value is set for.
    pub variable: VariableType,
}

/// Class to hold the result of setting a variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResultType {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. Type of attribute: Actual, Target, MinSet, MaxSet.
    /// Default is Actual when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<AttributeEnumType>,

    /// Required. Result status of setting the variable.
    pub attribute_status: SetVariableStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_status_info: Option<StatusInfoType>,

    /// Required. Component for which the variable is set.
    pub component: ComponentType,

    /// Required. Variable which the value is set for.
    pub variable: VariableType,
}

/// Request to set variables in a charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of settings to set in components.
    #[validate(length(min = 1), nested)]
    pub set_variable_data: Vec<SetVariableDataType>,
}

/// Response to SetVariables request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SetVariablesResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. List of result statuses per settings.
    #[validate(length(min = 1), nested)]
    pub set_variable_result: Vec<SetVariableResultType>,
}
