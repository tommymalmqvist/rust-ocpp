use serde::{Deserialize, Serialize};

/// Source of status: OCSP, CRL
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateStatusSourceEnumType {
    #[serde(rename = "CRL")]
    CRL,
    #[serde(rename = "OCSP")]
    OCSP,
}

impl Default for CertificateStatusSourceEnumType {
    fn default() -> Self {
        Self::OCSP
    }
}
