use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station was able to execute the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for ClearChargingProfileStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
