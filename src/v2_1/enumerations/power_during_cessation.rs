#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PowerDuringCessationEnumType {
    #[default]
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Reactive")]
    Reactive,
}
