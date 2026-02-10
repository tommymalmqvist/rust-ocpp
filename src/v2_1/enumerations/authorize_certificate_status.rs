use serde::{Deserialize, Serialize};

/// Certificate status information.
/// - if all certificates are valid: return 'Accepted'.
/// - if one of the certificates was revoked, return 'CertificateRevoked'.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthorizeCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "SignatureError")]
    SignatureError,
    #[serde(rename = "CertificateExpired")]
    CertificateExpired,
    #[serde(rename = "CertificateRevoked")]
    CertificateRevoked,
    #[serde(rename = "NoCertificateAvailable")]
    NoCertificateAvailable,
    #[serde(rename = "CertChainError")]
    CertChainError,
    #[serde(rename = "ContractCancelled")]
    ContractCancelled,
}
