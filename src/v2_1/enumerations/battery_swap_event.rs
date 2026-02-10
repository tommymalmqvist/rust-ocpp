use serde::{Deserialize, Serialize};

/// Battery in/out
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BatterySwapEventEnumType {
    #[serde(rename = "BatteryIn")]
    BatteryIn,
    #[serde(rename = "BatteryOut")]
    BatteryOut,
    #[serde(rename = "BatteryOutTimeout")]
    BatteryOutTimeout,
}
