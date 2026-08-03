//! GetInstalledCertificateIds
use crate::v2_0_1::datatypes::certificate_hash_data_chain_type::CertificateHashDataChainType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;
use crate::v2_0_1::enumerations::get_installed_certificate_status_enum_type::GetInstalledCertificateStatusEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Used by the CSMS to request an overview of the installed certificates on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    /// Indicates the type of certificates requested.When omitted, all certificate types are requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,
}

/// Response to a GetInstalledCertificateIDsRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    /// Charging Station indicates if it can process therequest
    pub status: GetInstalledCertificateStatusEnumType,
    /// The Charging Station includes the Certificateinformation for each available certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
