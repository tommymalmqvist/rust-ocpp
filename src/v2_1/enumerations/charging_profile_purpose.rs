use serde::{Deserialize, Serialize};

/// Defines the purpose of the schedule transferred by this profile
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingProfilePurposeEnumType {
    #[serde(rename = "ChargingStationExternalConstraints")]
    ChargingStationExternalConstraints,
    #[serde(rename = "ChargingStationMaxProfile")]
    ChargingStationMaxProfile,
    #[serde(rename = "TxDefaultProfile")]
    TxDefaultProfile,
    #[serde(rename = "TxProfile")]
    TxProfile,
    #[serde(rename = "PriorityCharging")]
    PriorityCharging,
    #[serde(rename = "LocalGeneration")]
    LocalGeneration,
}
