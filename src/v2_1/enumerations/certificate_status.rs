use serde::{Deserialize, Serialize};

/// Status of certificate: good, revoked or unknown.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateStatusEnumType {
    #[serde(rename = "Good")]
    Good,
    #[serde(rename = "Revoked")]
    Revoked,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Failed")]
    Failed,
}

impl Default for CertificateStatusEnumType {
    fn default() -> Self {
        Self::Unknown
    }
}
