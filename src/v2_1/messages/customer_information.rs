use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CertificateHashDataType, CustomDataType, IdTokenType, StatusInfoType},
    enumerations::CustomerInformationStatusEnumType,
};

/// Request to get or clear customer information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationRequest {
    /// Required. The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Required. Flag indicating whether the Charging Station should return
    /// NotifyCustomerInformationRequest messages containing information about
    /// the customer referred to.
    pub report: bool,

    /// Required. Flag indicating whether the Charging Station should clear
    /// all information about the customer referred to.
    pub clear: bool,

    /// Optional. A (e.g. vendor specific) identifier of the customer this request
    /// refers to. This field contains a custom identifier other than IdToken and
    /// Certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub customer_identifier: Option<String>,

    /// Optional. The customer certificate to get or clear information for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_certificate: Option<CertificateHashDataType>,

    /// Optional. The customer token to get or clear information for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a CustomerInformationRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerInformationResponse {
    /// Required. Indicates whether the request was accepted.
    pub status: CustomerInformationStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
