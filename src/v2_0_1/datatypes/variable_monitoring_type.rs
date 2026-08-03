use rust_decimal::Decimal;

use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;

/// A monitoring setting for a variable.
/// VariableMonitoringType is used by: NotifyMonitoringReportRequest.MonitoringDataType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct VariableMonitoringType {
    pub id: i32,
    pub transaction: bool,
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision")
    )]
    pub value: Decimal,
    #[serde(rename = "type")]
    pub kind: MonitorEnumType,
    pub severity: u8,
}
