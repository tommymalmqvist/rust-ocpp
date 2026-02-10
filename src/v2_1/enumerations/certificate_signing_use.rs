use serde::{Deserialize, Serialize};

/// Indicates the type of the signed certificate that is returned. When omitted the certificate is used for both the 15118 connection (if implemented) and the Charging Station to CSMS connection. This field is required when a typeOfCertificate was included in the SignCertificateRequest that requested this certificate to be signed AND both the 15118 connection and the Charging Station connection are implemented.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificateSigningUseEnumType {
    #[serde(rename = "ChargingStationCertificate")]
    ChargingStationCertificate,
    #[serde(rename = "V2GCertificate")]
    V2GCertificate,
    #[serde(rename = "V2G20Certificate")]
    V2G20Certificate,
}
