#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReportBaseEnumType {
    #[default]
    #[serde(rename = "ConfigurationInventory")]
    ConfigurationInventory,
    #[serde(rename = "FullInventory")]
    FullInventory,
    #[serde(rename = "SummaryInventory")]
    SummaryInventory,
}
