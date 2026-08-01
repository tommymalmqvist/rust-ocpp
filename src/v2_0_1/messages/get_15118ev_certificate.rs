//! Get15118EVCertificate
#[cfg(not(feature = "std"))]
use alloc::string::String;

use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::certificate_action_enum_type::CertificateActionEnumType;
use crate::v2_0_1::enumerations::iso15118ev_certificate_status_enum_type::Iso15118EVCertificateStatusEnumType;

/// Get15118EVCertificateRequest, sent by the Charging Station to the CSMS.
///
/// If an ISO 15118 vehicle selects the service Certificate installation.
///
/// NOTE:
/// This message is based on CertificateInstallationReq Res from ISO 15118 2.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateRequest {
    /// Schema version currently used for the 15118 session between EV and Charging Station. Needed for parsing of the EXI stream by the CSMS.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 50)))]
    #[serde(rename = "iso15118SchemaVersion")]
    pub iso_15118_schema_version: String,
    /// Defines whether certificate needs to be installed or updated.
    pub action: CertificateActionEnumType,
    /// Raw CertificateInstallationReq request from EV, Base64 encoded.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 5600)))]
    pub exi_request: String,
}

/// Get15118EVCertificateResponse, Response message from CSMS to Charging Station.
///
/// Containing the status and optionally new certificate.
///
/// NOTE: This message is based on CertificateInstallationReq Res from ISO 15118-2.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Get15118EVCertificateResponse {
    /// Indicates whether the message was processed properly.
    pub status: Iso15118EVCertificateStatusEnumType,
    /// Raw CertificateInstallationRes response for the EV, Base64 encoded.
    pub exi_response: String,
    /// Detailed status information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
