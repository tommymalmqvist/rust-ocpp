use serde::{Deserialize, Serialize};

/// Current status of the ID Token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Blocked")]
    Blocked,
    #[serde(rename = "ConcurrentTx")]
    ConcurrentTx,
    #[serde(rename = "Expired")]
    Expired,
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "NoCredit")]
    NoCredit,
    #[serde(rename = "NotAllowedTypeEVSE")]
    NotAllowedTypeEVSE,
    #[serde(rename = "NotAtThisLocation")]
    NotAtThisLocation,
    #[serde(rename = "NotAtThisTime")]
    NotAtThisTime,
    #[serde(rename = "Unknown")]
    Unknown,
}
