use serde::{Deserialize, Serialize};

/// The unit of measure in which limits and setpoints are expressed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingRateUnitEnumType {
    #[serde(rename = "W")]
    W,
    #[serde(rename = "A")]
    A,
}
