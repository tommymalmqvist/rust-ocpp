use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to perform the availability change.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeAvailabilityStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Scheduled")]
    Scheduled,
}
