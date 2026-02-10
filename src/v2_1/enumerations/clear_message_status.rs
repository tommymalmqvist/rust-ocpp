use serde::{Deserialize, Serialize};

/// Returns whether the Charging Station has been able to remove the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearMessageStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Rejected")]
    Rejected,
}

impl Default for ClearMessageStatusEnumType {
    fn default() -> Self {
        Self::Unknown
    }
}
