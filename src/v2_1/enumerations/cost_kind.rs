use serde::{Deserialize, Serialize};

/// The kind of cost referred to in the message element amount
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostKindEnumType {
    #[serde(rename = "CarbonDioxideEmission")]
    CarbonDioxideEmission,
    #[serde(rename = "RelativePricePercentage")]
    RelativePricePercentage,
    #[serde(rename = "RenewableGenerationPercentage")]
    RenewableGenerationPercentage,
}

impl Default for CostKindEnumType {
    fn default() -> Self {
        Self::CarbonDioxideEmission
    }
}
