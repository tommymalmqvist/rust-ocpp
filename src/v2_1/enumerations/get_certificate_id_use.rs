use serde::{Deserialize, Serialize};

/// Indicates the type of the requested certificate(s).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCertificateIdUseEnumType {
    #[serde(rename = "V2GRootCertificate")]
    V2GRootCertificate,
    #[serde(rename = "MORootCertificate")]
    MORootCertificate,
    #[serde(rename = "CSMSRootCertificate")]
    CSMSRootCertificate,
    #[serde(rename = "V2GCertificateChain")]
    V2GCertificateChain,
    #[serde(rename = "ManufacturerRootCertificate")]
    ManufacturerRootCertificate,
    #[serde(rename = "OEMRootCertificate")]
    OEMRootCertificate,
}

impl Default for GetCertificateIdUseEnumType {
    fn default() -> Self {
        Self::CSMSRootCertificate
    }
}
