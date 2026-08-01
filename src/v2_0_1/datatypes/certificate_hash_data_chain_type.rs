use super::certificate_hash_data_type::CertificateHashDataType;
use crate::v2_0_1::enumerations::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// CertificateHashDataChainType is used by: GetInstalledCertificateIdsResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    /// Required. Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType,
    /// Required. Information to identify a certificate
    pub certificate_hash_data: CertificateHashDataType,
    /// Optional. Information to identify the child certificate(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
}
