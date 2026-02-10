use serde::{Deserialize, Serialize};

/// Charging Station indicates if it can process the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteCertificateStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "NotFound")]
    NotFound,
}

impl Default for DeleteCertificateStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
