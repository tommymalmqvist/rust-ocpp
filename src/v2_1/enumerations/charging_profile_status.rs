use serde::{Deserialize, Serialize};

/// Returns whether the Charging Station has been able to process the message successfully.
/// This does not guarantee the schedule will be followed to the letter.
/// There might be other constraints the Charging Station may need to take into account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}
