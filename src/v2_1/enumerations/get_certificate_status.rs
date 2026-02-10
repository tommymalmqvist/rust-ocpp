use serde::{Deserialize, Serialize};

/// This indicates whether the charging station was able to retrieve the OCSP certificate status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
}
