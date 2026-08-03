use super::component_type::ComponentType;
use super::variable_attribute_type::VariableAttributeType;
use super::variable_characteristics_type::VariableCharacteristicsType;
use super::variable_type::VariableType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Class to report components, variables and variable attributes and characteristics.
/// ReportDataType is used by: NotifyReportRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    pub component: ComponentType,
    pub variable: VariableType,
    pub variable_attribute: Vec<VariableAttributeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_characteristics: Option<VariableCharacteristicsType>,
}
