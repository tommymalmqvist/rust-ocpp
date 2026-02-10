use serde::{Deserialize, Serialize};

/// Defines the transport protocol (e.g. SOAP or JSON).
/// Note: SOAP is not supported in OCPP 2.x, but is supported by earlier versions of OCPP.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OCPPTransportEnumType {
    #[serde(rename = "SOAP")]
    SOAP,
    #[serde(rename = "JSON")]
    JSON,
}
