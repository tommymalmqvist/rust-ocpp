#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SetMonitoringStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,
    #[serde(rename = "UnsupportedMonitorType")]
    UnsupportedMonitorType,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "OutOfRange")]
    OutOfRange,
}
