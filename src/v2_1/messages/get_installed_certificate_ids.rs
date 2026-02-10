use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CertificateHashDataChainType, CustomDataType, StatusInfoType},
    enumerations::{GetCertificateIdUseEnumType, GetInstalledCertificateStatusEnumType},
};

/// Request to get the installed certificate IDs from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsRequest {
    /// Optional. Indicates the type of certificates requested.
    /// When omitted, all certificate types are requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub certificate_type: Option<Vec<GetCertificateIdUseEnumType>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetInstalledCertificateIdsRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledCertificateIdsResponse {
    /// Required. Charging Station indicates if it can process the request.
    pub status: GetInstalledCertificateStatusEnumType,

    /// Optional. Array of certificate hash data chains, each representing a certificate chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub certificate_hash_data_chain: Option<Vec<CertificateHashDataChainType>>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
