use serde::{Deserialize, Serialize};

/// This indicates the success or failure of the data transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataTransferStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "UnknownMessageId")]
    UnknownMessageId,
    #[serde(rename = "UnknownVendorId")]
    UnknownVendorId,
}

impl Default for DataTransferStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
