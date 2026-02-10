use serde::{Deserialize, Serialize};

/// Type of DER curve
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DERControlEnumType {
    #[serde(rename = "EnterService")]
    EnterService,
    #[serde(rename = "FreqDroop")]
    FreqDroop,
    #[serde(rename = "FreqWatt")]
    FreqWatt,
    #[serde(rename = "FixedPFAbsorb")]
    FixedPFAbsorb,
    #[serde(rename = "FixedPFInject")]
    FixedPFInject,
    #[serde(rename = "FixedVar")]
    FixedVar,
    #[serde(rename = "Gradients")]
    Gradients,
    #[serde(rename = "HFMustTrip")]
    HFMustTrip,
    #[serde(rename = "HFMayTrip")]
    HFMayTrip,
    #[serde(rename = "HVMustTrip")]
    HVMustTrip,
    #[serde(rename = "HVMomCess")]
    HVMomCess,
    #[serde(rename = "HVMayTrip")]
    HVMayTrip,
    #[serde(rename = "LimitMaxDischarge")]
    LimitMaxDischarge,
    #[serde(rename = "LFMustTrip")]
    LFMustTrip,
    #[serde(rename = "LVMustTrip")]
    LVMustTrip,
    #[serde(rename = "LVMomCess")]
    LVMomCess,
    #[serde(rename = "LVMayTrip")]
    LVMayTrip,
    #[serde(rename = "PowerMonitoringMustTrip")]
    PowerMonitoringMustTrip,
    #[serde(rename = "VoltVar")]
    VoltVar,
    #[serde(rename = "VoltWatt")]
    VoltWatt,
    #[serde(rename = "WattPF")]
    WattPF,
    #[serde(rename = "WattVar")]
    WattVar,
    PowerLimitation,
    PowerTarget,
    PowerFactor,
    VoltageTarget,
    CurrentTarget,
    LoadPriority,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DERControlStatusEnumType {
    Accepted,
    Rejected,
    NotSupported,
}
