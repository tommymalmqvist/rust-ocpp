//! CertificateSigned
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::certificate_signed_status_enum_type::CertificateSignedStatusEnumType;
use crate::v2_0_1::enumerations::certificate_signing_use_enum_type::CertificateSigningUseEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// `CertificateSignedRequest`, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    /// The signed PEM encoded X.509 certificate. This can also contain the necessary sub
    /// CA certificates. In that case, the order of the bundle should follow the
    /// certificate chain, starting from the leaf certificate.
    ///
    /// The Configuration Variable `MaxCertificateChainSize` can be used to limit the
    ///  maximum size of this field.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 10000)))]
    pub certificate_chain: String,
    /// Indicates the type of the signed certificate that is returned. When omitted the
    ///  certificate is used for both the 15118 connection (if implemented) and the
    /// Charging Station to CSMS connection. This field is required when a typeOfCertificate
    /// was included in the [SignCertificateRequest](`crate::v2_0_1::messages::sign_certificate::SignCertificateRequest`)
    /// that requested this certificate to be signed AND both the 15118 connection
    /// and the Charging Station connection are implemented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,
}

/// `CertificateSignedResponse`, sent by the Charging Station to the CSMS in response to a [`CertificateSignedRequest`].
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    /// Returns whether certificate signing has been accepted, otherwise rejected.
    pub status: CertificateSignedStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
