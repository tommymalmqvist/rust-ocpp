use crate::v2_0_1::enumerations::hash_algorithm_enum_type::HashAlgorithmEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// OCSPRequestDataType is used by: AuthorizeRequest , GetCertificateStatusRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    pub hash_algorithm: HashAlgorithmEnumType,
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 128)))]
    pub issuer_name_hash: String,
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 128)))]
    pub issuer_key_hash: String,
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 40)))]
    pub serial_number: String,
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 512)))]
    #[serde(rename = "responderURL")]
    pub responder_url: String,
}
