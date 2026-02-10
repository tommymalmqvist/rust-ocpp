use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, status_info::StatusInfoType};
use crate::v2_1::enumerations::{
    install_certificate_status::InstallCertificateStatusEnumType,
    install_certificate_use::InstallCertificateUseEnumType,
};

/// Used by the CSMS to request installation of a certificate on a Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateRequest {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Indicates the certificate type that is sent.
    pub certificate_type: InstallCertificateUseEnumType,

    /// Required. A PEM encoded X.509 certificate.
    #[validate(length(max = 10000))]
    pub certificate: String,
}

/// The response to a InstallCertificateRequest, sent by the Charging Station to the CSMS.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InstallCertificateResponse {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Charging Station indicates if installation was successful.
    pub status: InstallCertificateStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
