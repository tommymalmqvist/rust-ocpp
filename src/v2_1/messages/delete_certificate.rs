use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CertificateHashDataType, CustomDataType, StatusInfoType},
    enumerations::DeleteCertificateStatusEnumType,
};

/// Request to delete a certificate from the charging station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateRequest {
    /// Required. Certificate data to be deleted from the charging station.
    pub certificate_hash_data: CertificateHashDataType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a DeleteCertificateRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCertificateResponse {
    /// Required. Charging Station indicates if it can process the request.
    pub status: DeleteCertificateStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
