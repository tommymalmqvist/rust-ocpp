//! InstallCertificate

use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::install_certificate_status_enum_type::InstallCertificateStatusEnumType;
use crate::v2_0_1::enumerations::install_certificate_use_enum_type::InstallCertificateUseEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Used by the CSMS to request installation of a certificate on a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    /// Indicates the certificate type that is sent.
    pub certificate_type: InstallCertificateUseEnumType,
    /// A PEM encoded X.509 certificate.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 5500)))]
    pub certificate: String,
}

/// The response to a InstallCertificateRequest, sent by the Charging Station to the CSMS
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    /// Charging Station indicates if installation wassuccessful.
    pub status: InstallCertificateStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
