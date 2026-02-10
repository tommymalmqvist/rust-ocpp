use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{CustomDataType, OCSPRequestDataType, StatusInfoType};
use crate::v2_1::enumerations::GetCertificateStatusEnumType;

/// Request to get the status of a certificate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusRequest {
    /// Required. Information about the certificate for which the status is requested.
    pub ocsp_request_data: OCSPRequestDataType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetCertificateStatusRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateStatusResponse {
    /// Required. This indicates whether the charging station was able to retrieve
    /// the OCSP certificate status.
    pub status: GetCertificateStatusEnumType,

    /// Optional. OCSPResponse class as defined in IETF RFC 6960. DER encoded
    /// (as defined in IETF RFC 6960), and then base64 encoded. MAY only be
    /// omitted when status is not Accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 5500))]
    pub ocsp_result: Option<String>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
