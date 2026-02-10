#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PaymentStatusEnumType {
    #[serde(rename = "Settled")]
    Settled,
    #[serde(rename = "Canceled")]
    Canceled,
    #[serde(rename = "Rejected")]
    Rejected,
    #[default]
    #[serde(rename = "Failed")]
    Failed,
}
