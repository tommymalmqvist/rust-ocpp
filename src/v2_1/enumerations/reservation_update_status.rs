#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReservationUpdateStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "Rejected")]
    Rejected,
}
