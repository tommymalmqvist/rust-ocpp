#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum NotifyEVChargingNeedsStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Processing")]
    Processing,
    #[serde(rename = "NoChargingProfile")]
    NoChargingProfile,
}
