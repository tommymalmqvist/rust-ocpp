use rust_decimal::Decimal;

use super::component_type::ComponentType;
use super::variable_type::VariableType;
use crate::v2_0_1::enumerations::monitor_enum_type::MonitorEnumType;

/// Class to hold parameters of SetVariableMonitoring request.
/// SetMonitoringDataType is used by: SetVariableMonitoringRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMonitoringDataType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<bool>,
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
    pub component: ComponentType,
    pub variable: VariableType,
}
