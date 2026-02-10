use serde::{Deserialize, Serialize};

/// Standardized values for the signingMethod in a SignedMeterValueType.
/// The algorithm, curve, key length and hash algorithm information is for documentation only
/// and not part of the standardized value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SigningMethodEnumType {
    /// Standard OCPP signing methods
    Standard(StandardSigningMethodEnumType),
    /// Custom signing method value
    Custom(String),
}

/// Standard OCPP signing method values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandardSigningMethodEnumType {
    /// ECDSA with secp192k1 curve and SHA256 hash
    #[serde(rename = "ECDSA-secp192k1-SHA256")]
    ECDSAsecp192k1SHA256,
    /// ECDSA with secp256k1 curve and SHA256 hash
    #[serde(rename = "ECDSA-secp256k1-SHA256")]
    ECDSAsecp256k1SHA256,
    /// ECDSA with secp192r1 curve and SHA256 hash
    #[serde(rename = "ECDSA-secp192r1-SHA256")]
    ECDSAsecp192r1SHA256,
    /// ECDSA with secp256r1 curve and SHA256 hash
    #[serde(rename = "ECDSA-secp256r1-SHA256")]
    ECDSAsecp256r1SHA256,
    /// ECDSA with brainpool256r1 curve and SHA256 hash
    #[serde(rename = "ECDSA-brainpool256r1-SHA256")]
    ECDSAbrainpool256r1SHA256,
    /// ECDSA with secp384r1 curve and SHA256 hash
    #[serde(rename = "ECDSA-secp384r1-SHA256")]
    ECDSAsecp384r1SHA256,
    /// ECDSA with brainpool384r1 curve and SHA256 hash
    #[serde(rename = "ECDSA-brainpool384r1-SHA256")]
    ECDSAbrainpool384r1SHA256,
}

impl SigningMethodEnumType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standard(s) => match s {
                StandardSigningMethodEnumType::ECDSAsecp192k1SHA256 => "ECDSA-secp192k1-SHA256",
                StandardSigningMethodEnumType::ECDSAsecp256k1SHA256 => "ECDSA-secp256k1-SHA256",
                StandardSigningMethodEnumType::ECDSAsecp192r1SHA256 => "ECDSA-secp192r1-SHA256",
                StandardSigningMethodEnumType::ECDSAsecp256r1SHA256 => "ECDSA-secp256r1-SHA256",
                StandardSigningMethodEnumType::ECDSAbrainpool256r1SHA256 => {
                    "ECDSA-brainpool256r1-SHA256"
                }
                StandardSigningMethodEnumType::ECDSAsecp384r1SHA256 => "ECDSA-secp384r1-SHA256",
                StandardSigningMethodEnumType::ECDSAbrainpool384r1SHA256 => {
                    "ECDSA-brainpool384r1-SHA256"
                }
            },
            Self::Custom(s) => s,
        }
    }
}

impl From<String> for SigningMethodEnumType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ECDSA-secp192k1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAsecp192k1SHA256)
            }
            "ECDSA-secp256k1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAsecp256k1SHA256)
            }
            "ECDSA-secp192r1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAsecp192r1SHA256)
            }
            "ECDSA-secp256r1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAsecp256r1SHA256)
            }
            "ECDSA-brainpool256r1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAbrainpool256r1SHA256)
            }
            "ECDSA-secp384r1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAsecp384r1SHA256)
            }
            "ECDSA-brainpool384r1-SHA256" => {
                Self::Standard(StandardSigningMethodEnumType::ECDSAbrainpool384r1SHA256)
            }
            _ => Self::Custom(s),
        }
    }
}

impl ToString for SigningMethodEnumType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
