#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessageStateEnumType {
    #[serde(rename = "Charging")]
    Charging,
    #[serde(rename = "Faulted")]
    Faulted,
    #[default]
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "Unavailable")]
    Unavailable,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "Discharging")]
    Discharging,
}
