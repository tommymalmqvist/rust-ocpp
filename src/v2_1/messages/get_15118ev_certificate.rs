use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{CertificateActionEnumType, Iso15118EVCertificateStatusEnumType},
};

/// Request to get the ISO 15118 certificate for an EV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    /// Required. Schema version currently used for the 15118 session between EV and
    /// Charging Station. Needed for parsing of the EXI stream by the CSMS.
    #[validate(length(max = 50))]
    pub iso_15118_schema_version: String,

    /// Required. Defines whether certificate needs to be installed or updated.
    pub action: CertificateActionEnumType,

    /// Required. Raw CertificateInstallationReq request from EV, Base64 encoded.
    /// Extended to support ISO 15118-20 certificates. The minimum supported length is 11000.
    /// If a longer exiRequest is supported, then the supported length must be communicated
    /// in variable OCPPCommCtrlr.FieldLength["Get15118EVCertificateRequest.exiRequest"].
    #[validate(length(max = 11000))]
    pub exi_request: String,

    /// Optional. Absent during ISO 15118-2 session. Required during ISO 15118-20 session.
    /// Maximum number of contracts that EV wants to install.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub maximum_contract_certificate_chains: Option<i32>,

    /// Optional. Absent during ISO 15118-2 session. Optional during ISO 15118-20 session.
    /// List of EMAIDs for which contract certificates must be requested first, in case
    /// there are more certificates than allowed by maximumContractCertificateChains.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 8))]
    pub prioritized_emaids: Option<Vec<String>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a Get15118EVCertificateRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    /// Required. Indicates whether the message was processed properly.
    pub status: Iso15118EVCertificateStatusEnumType,

    /// Required. Raw CertificateInstallationRes response for the EV, Base64 encoded.
    /// Extended to support ISO 15118-20 certificates. The minimum supported length is 17000.
    /// If a longer exiResponse is supported, then the supported length must be communicated
    /// in variable OCPPCommCtrlr.FieldLength["Get15118EVCertificateResponse.exiResponse"].
    #[validate(length(max = 17000))]
    pub exi_response: String,

    /// Optional. Number of contracts that can be retrieved with additional requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub remaining_contracts: Option<i32>,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
