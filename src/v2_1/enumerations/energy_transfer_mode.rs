use serde::{Deserialize, Serialize};

/// Defines the energy transfer modes that are allowed by the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnergyTransferModeEnumType {
    #[serde(rename = "AC_single_phase")]
    ACSinglePhase,
    #[serde(rename = "AC_two_phase")]
    ACTwoPhase,
    #[serde(rename = "AC_three_phase")]
    ACThreePhase,
    #[serde(rename = "DC")]
    DC,
    #[serde(rename = "AC_BPT")]
    ACBPT,
    #[serde(rename = "AC_BPT_DER")]
    ACBPTDER,
    #[serde(rename = "AC_DER")]
    ACDER,
    #[serde(rename = "DC_BPT")]
    DCBPT,
    #[serde(rename = "DC_ACDP")]
    DCACDP,
    #[serde(rename = "DC_ACDP_BPT")]
    DCACDPBPT,
    #[serde(rename = "WPT")]
    WPT,
}
