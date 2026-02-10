#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReserveNowStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Faulted")]
    Faulted,
    #[serde(rename = "Occupied")]
    Occupied,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "Unavailable")]
    Unavailable,
}
