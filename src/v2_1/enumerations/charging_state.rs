use serde::{Deserialize, Serialize};

/// Current charging state, is required when state has changed.
/// Omitted when there is no communication between EVSE and EV, because no cable is plugged in.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingStateEnumType {
    #[serde(rename = "EVConnected")]
    EVConnected,
    #[serde(rename = "Charging")]
    Charging,
    #[serde(rename = "SuspendedEV")]
    SuspendedEV,
    #[serde(rename = "SuspendedEVSE")]
    SuspendedEVSE,
    #[serde(rename = "Idle")]
    Idle,
}
