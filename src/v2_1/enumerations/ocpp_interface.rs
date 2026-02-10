use serde::{Deserialize, Serialize};

/// Applicable Network Interface. Charging Station is allowed to use a different network interface
/// to connect if the given one does not work.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OCPPInterfaceEnumType {
    #[serde(rename = "Wired0")]
    Wired0,
    #[serde(rename = "Wired1")]
    Wired1,
    #[serde(rename = "Wired2")]
    Wired2,
    #[serde(rename = "Wired3")]
    Wired3,
    #[serde(rename = "Wireless0")]
    Wireless0,
    #[serde(rename = "Wireless1")]
    Wireless1,
    #[serde(rename = "Wireless2")]
    Wireless2,
    #[serde(rename = "Wireless3")]
    Wireless3,
    #[serde(rename = "Any")]
    Any,
}
