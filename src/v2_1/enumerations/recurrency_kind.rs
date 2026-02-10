#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum RecurrencyKindEnumType {
    #[default]
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekly")]
    Weekly,
}
