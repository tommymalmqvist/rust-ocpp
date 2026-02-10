use serde::{Deserialize, Serialize};

/// Defines whether certificate needs to be installed or updated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateActionEnumType {
    #[serde(rename = "Install")]
    Install,
    #[serde(rename = "Update")]
    Update,
}

impl Default for CertificateActionEnumType {
    fn default() -> Self {
        Self::Install
    }
}
