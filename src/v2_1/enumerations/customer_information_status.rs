use serde::{Deserialize, Serialize};

/// Indicates whether the request was accepted.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CustomerInformationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Invalid")]
    Invalid,
}

impl Default for CustomerInformationStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
