use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::{CertificateSignedStatusEnumType, CertificateSigningUseEnumType};

/// Request to inform the Charging Station about the result of a certificate signing operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedRequest {
    /// The signed PEM encoded X.509 certificate. This SHALL also contain the necessary sub CA certificates, when applicable.
    /// The order of the bundle follows the certificate chain, starting from the leaf certificate.
    #[validate(length(max = 10000))]
    pub certificate_chain: String,

    /// Optional. Indicates the type of the signed certificate that is returned.
    /// When omitted the certificate is used for both the 15118 connection (if implemented) and the Charging Station to CSMS connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<CertificateSigningUseEnumType>,

    /// Optional. RequestId to correlate this message with the SignCertificateRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a CertificateSignedRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateSignedResponse {
    /// Required. Returns whether certificate signing has been accepted, otherwise rejected.
    pub status: CertificateSignedStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
