#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MutabilityEnumType {
    #[serde(rename = "ReadOnly")]
    ReadOnly,
    #[serde(rename = "WriteOnly")]
    WriteOnly,
    #[default]
    #[serde(rename = "ReadWrite")]
    ReadWrite,
}
