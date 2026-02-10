use serde::{Deserialize, Serialize};

/// Charging operation mode to use during this time interval. When absent defaults to `ChargingOnly`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationModeEnumType {
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "ChargingOnly")]
    ChargingOnly,
    #[serde(rename = "CentralSetpoint")]
    CentralSetpoint,
    #[serde(rename = "ExternalSetpoint")]
    ExternalSetpoint,
    #[serde(rename = "ExternalLimits")]
    ExternalLimits,
    #[serde(rename = "CentralFrequency")]
    CentralFrequency,
    #[serde(rename = "LocalFrequency")]
    LocalFrequency,
    #[serde(rename = "LocalLoadBalancing")]
    LocalLoadBalancing,
}
