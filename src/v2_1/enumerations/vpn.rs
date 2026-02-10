use serde::{Deserialize, Serialize};

/// Type of VPN
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VPNEnumType {
    #[serde(rename = "IKEv2")]
    IKEv2,
    #[serde(rename = "IPSec")]
    IPSec,
    #[serde(rename = "L2TP")]
    L2TP,
    #[serde(rename = "PPTP")]
    PPTP,
}
