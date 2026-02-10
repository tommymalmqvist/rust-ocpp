use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    CertificateStatusRequestInfoType, CertificateStatusType, CustomDataType,
};

/// Request to get the status of a certificate chain.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusRequest {
    /// Required. Array of certificate status requests.
    #[validate(length(min = 1, max = 4))]
    pub certificate_status_requests: Vec<CertificateStatusRequestInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetCertificateChainStatusRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateChainStatusResponse {
    /// Required. Array of certificate status information.
    #[validate(length(min = 1, max = 4))]
    pub certificate_status: Vec<CertificateStatusType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
