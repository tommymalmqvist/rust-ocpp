#[cfg(not(feature = "std"))]
use alloc::string::String;
use rust_decimal::Decimal;

use crate::v2_0_1::enumerations::data_enum_type::DataEnumType;

/// Fixed read-only parameters of a variable.
/// VariableCharacteristicsType is used by: NotifyReportRequest.ReportDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    pub data_type: DataEnumType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision_option")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision::option")
    )]
    pub min_limit: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision_option")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision::option")
    )]
    pub max_limit: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_list: Option<String>,
    pub supports_monitoring: bool,
}
