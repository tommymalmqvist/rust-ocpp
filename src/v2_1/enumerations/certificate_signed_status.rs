use serde::{Deserialize, Serialize};

/// Returns whether certificate signing has been accepted, otherwise rejected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateSignedStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}

impl Default for CertificateSignedStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
