use serde::{Deserialize, Serialize};

/// This contains the current status of the Connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectorStatusEnumType {
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Occupied")]
    Occupied,
    #[serde(rename = "Reserved")]
    Reserved,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Faulted")]
    Faulted,
}
