use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CertificateHashDataType, CustomDataType, StatusInfoType};

/// Indicates the type of certificate that is to be signed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateSigningUseEnumType {
    ChargingStationCertificate,
    V2GCertificate,
    V2G20Certificate,
}

/// Specifies whether the CSMS can process the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GenericStatusEnumType {
    Accepted,
    Rejected,
}

/// Request to sign a certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The Charging Station SHALL send the public key in form of a Certificate
    /// Signing Request (CSR) as described in RFC 2986 and then PEM encoded.
    #[validate(length(max = 5500))]
    pub csr: String,

    /// Optional. Indicates the type of certificate that is to be signed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    /// Optional. Hash of the root certificate to be installed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_root_certificate: Option<CertificateHashDataType>,

    /// Optional. RequestId to match this message with the CertificateSignedRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,
}

/// Response to a SignCertificateRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignCertificateResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Returns whether the CSMS can process the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
