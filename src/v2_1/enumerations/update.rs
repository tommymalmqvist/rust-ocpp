#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UpdateEnumType {
    #[default]
    #[serde(rename = "Differential")]
    Differential,
    #[serde(rename = "Full")]
    Full,
}
