#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UnlockStatusEnumType {
    #[default]
    #[serde(rename = "Unlocked")]
    Unlocked,
    #[serde(rename = "UnlockFailed")]
    UnlockFailed,
    #[serde(rename = "OngoingAuthorizedTransaction")]
    OngoingAuthorizedTransaction,
    #[serde(rename = "UnknownConnector")]
    UnknownConnector,
}
