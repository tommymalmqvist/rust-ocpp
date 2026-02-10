use serde::{Deserialize, Serialize};

/// Result of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DERControlStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "NotFound")]
    NotFound,
}

impl Default for DERControlStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
