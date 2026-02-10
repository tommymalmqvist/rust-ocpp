use serde::{Deserialize, Serialize};

/// Accepted if the Charging Station has executed the request, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearCacheStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

impl Default for ClearCacheStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
