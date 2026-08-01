use super::consumption_cost_type::ConsumptionCostType;
use super::relative_time_interval_type::RelativeTimeIntervalType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// SalesTariffEntryType is used by: Common:SalesTariffType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_price_level: Option<u32>,
    pub relative_time_interval: RelativeTimeIntervalType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,
}
