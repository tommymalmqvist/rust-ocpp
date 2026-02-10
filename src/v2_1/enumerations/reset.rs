#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ResetEnumType {
    #[default]
    #[serde(rename = "Immediate")]
    Immediate,
    #[serde(rename = "OnIdle")]
    OnIdle,
}
