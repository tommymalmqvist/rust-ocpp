use serde::{Deserialize, Serialize};

/// Type of cost dimension: energy, power, time, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostDimensionEnumType {
    #[serde(rename = "Energy")]
    Energy,
    #[serde(rename = "MaxCurrent")]
    MaxCurrent,
    #[serde(rename = "MinCurrent")]
    MinCurrent,
    #[serde(rename = "MaxPower")]
    MaxPower,
    #[serde(rename = "MinPower")]
    MinPower,
    #[serde(rename = "IdleTIme")]
    IdleTime,
    #[serde(rename = "ChargingTime")]
    ChargingTime,
}

impl Default for CostDimensionEnumType {
    fn default() -> Self {
        Self::Energy
    }
}
