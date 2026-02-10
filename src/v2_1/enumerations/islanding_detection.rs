use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum IslandingDetectionEnumType {
    #[default]
    NoAntiIslandingSupport,
    RoCoF,
    #[serde(rename = "UVP_OVP")]
    UvpOvp,
    #[serde(rename = "UFP_OFP")]
    UfpOfp,
    VoltageVectorShift,
    ZeroCrossingDetection,
    OtherPassive,
    ImpedanceMeasurement,
    ImpedanceAtFrequency,
    SlipModeFrequencyShift,
    SandiaFrequencyShift,
    SandiaVoltageShift,
    FrequencyJump,
    RCLQFactor,
    OtherActive,
}
