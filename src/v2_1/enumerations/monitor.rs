#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MonitorEnumType {
    #[default]
    #[serde(rename = "UpperThreshold")]
    UpperThreshold,
    #[serde(rename = "LowerThreshold")]
    LowerThreshold,
    #[serde(rename = "Delta")]
    Delta,
    #[serde(rename = "Periodic")]
    Periodic,
    #[serde(rename = "PeriodicClockAligned")]
    PeriodicClockAligned,
    #[serde(rename = "TargetDelta")]
    TargetDelta,
    #[serde(rename = "TargetDeltaRelative")]
    TargetDeltaRelative,
}
