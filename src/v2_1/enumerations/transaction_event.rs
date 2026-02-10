#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TransactionEventEnumType {
    #[default]
    #[serde(rename = "Ended")]
    Ended,
    #[serde(rename = "Started")]
    Started,
    #[serde(rename = "Updated")]
    Updated,
}
