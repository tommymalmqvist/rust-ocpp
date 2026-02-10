use serde::{Deserialize, Serialize};

/// Charging Station indicates if installation was successful.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstallCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Failed")]
    Failed,
}

impl Default for InstallCertificateStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
