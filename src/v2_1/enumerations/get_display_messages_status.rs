use serde::{Deserialize, Serialize};

/// Indicates if the Charging Station has Display Messages that match the request criteria in the GetDisplayMessagesRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDisplayMessagesStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Unknown")]
    Unknown,
}

impl Default for GetDisplayMessagesStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
