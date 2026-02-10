#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SendLocalListStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Failed")]
    Failed,
    #[serde(rename = "VersionMismatch")]
    VersionMismatch,
}
