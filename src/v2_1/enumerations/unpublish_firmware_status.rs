#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UnpublishFirmwareStatusEnumType {
    #[default]
    #[serde(rename = "DownloadOngoing")]
    DownloadOngoing,
    #[serde(rename = "NoFirmware")]
    NoFirmware,
    #[serde(rename = "Unpublished")]
    Unpublished,
}
