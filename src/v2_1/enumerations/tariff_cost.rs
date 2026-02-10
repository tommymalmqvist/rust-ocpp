use serde::{Deserialize, Serialize};

/// Type of cost: normal or the minimum or maximum cost.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TariffCostEnumType {
    #[serde(rename = "NormalCost")]
    NormalCost,
    #[serde(rename = "MinCost")]
    MinCost,
    #[serde(rename = "MaxCost")]
    MaxCost,
}
