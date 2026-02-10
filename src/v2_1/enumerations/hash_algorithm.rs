use serde::{Deserialize, Serialize};

/// Used algorithms for the hashes provided.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "SHA384")]
    SHA384,
    #[serde(rename = "SHA512")]
    SHA512,
}
