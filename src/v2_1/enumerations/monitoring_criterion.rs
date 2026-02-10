#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MonitoringCriterionEnumType {
    #[default]
    #[serde(rename = "ThresholdMonitoring")]
    ThresholdMonitoring,
    #[serde(rename = "DeltaMonitoring")]
    DeltaMonitoring,
    #[serde(rename = "PeriodicMonitoring")]
    PeriodicMonitoring,
}
