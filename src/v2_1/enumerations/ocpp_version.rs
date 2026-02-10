use serde::{Deserialize, Serialize};

/// This field is ignored, since the OCPP version to use is determined during the websocket handshake.
/// The field is only kept for backwards compatibility with the OCPP 2.0.1 JSON schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OCPPVersionEnumType {
    #[serde(rename = "OCPP12")]
    OCPP12,
    #[serde(rename = "OCPP15")]
    OCPP15,
    #[serde(rename = "OCPP16")]
    OCPP16,
    #[serde(rename = "OCPP20")]
    OCPP20,
    #[serde(rename = "OCPP201")]
    OCPP201,
    #[serde(rename = "OCPP21")]
    OCPP21,
}
