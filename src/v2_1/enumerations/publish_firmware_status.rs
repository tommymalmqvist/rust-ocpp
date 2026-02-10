#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum PublishFirmwareStatusEnumType {
    #[default]
    #[serde(rename = "Published")]
    Published,
    #[serde(rename = "DownloadScheduled")]
    DownloadScheduled,
    #[serde(rename = "InvalidChecksum")]
    InvalidChecksum,
    #[serde(rename = "NotDownloaded")]
    NotDownloaded,
    #[serde(rename = "DownloadFailed")]
    DownloadFailed,
    #[serde(rename = "Downloaded")]
    Downloaded,
    #[serde(rename = "Downloading")]
    Downloading,
}
