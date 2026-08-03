use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// CertificateHashDataType is used by: Common:CertificateHashDataChainType , DeleteCertificateRequest , CustomerInformationRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,
    /// Required. Hashed value of the Issuer DN (Distinguished Name).
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 128)))]
    pub issuer_name_hash: String,
    /// Required. Hashed value of the issuers public key
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 128)))]
    pub issuer_key_hash: String,
    /// Required. The serial number of the certificate.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 40)))]
    pub serial_number: String,
}
