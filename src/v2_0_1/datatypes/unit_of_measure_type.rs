/// Represents a UnitOfMeasure with a multiplier
/// UnitOfMeasureType is used by: Common:SampledValueType
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasureType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}
