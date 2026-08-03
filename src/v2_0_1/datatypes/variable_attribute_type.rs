use crate::v2_0_1::enumerations::attribute_enum_type::AttributeEnumType;
use crate::v2_0_1::enumerations::mutability_enum_type::MutabilityEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Attribute data of a variable.
/// VariableAttributeType is used by: NotifyReportRequest.ReportDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AttributeEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutability: Option<MutabilityEnumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,
}
