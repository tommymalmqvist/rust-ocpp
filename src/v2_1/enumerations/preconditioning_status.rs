#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PreconditioningStatusEnumType {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Ready")]
    Ready,
    #[serde(rename = "NotReady")]
    NotReady,
    #[serde(rename = "Preconditioning")]
    Preconditioning,
}
