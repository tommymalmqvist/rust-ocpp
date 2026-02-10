use serde::{Deserialize, Serialize};

/// Status of operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GenericStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

impl Default for GenericStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
