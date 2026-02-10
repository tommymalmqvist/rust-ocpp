use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to perform the reset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Scheduled")]
    Scheduled,
}
