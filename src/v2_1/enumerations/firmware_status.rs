use serde::{Deserialize, Serialize};

/// This contains the progress status of the firmware installation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FirmwareStatusEnumType {
    #[serde(rename = "Downloaded")]
    Downloaded,
    #[serde(rename = "DownloadFailed")]
    DownloadFailed,
    #[serde(rename = "Downloading")]
    Downloading,
    #[serde(rename = "DownloadScheduled")]
    DownloadScheduled,
    #[serde(rename = "DownloadPaused")]
    DownloadPaused,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "InstallationFailed")]
    InstallationFailed,
    #[serde(rename = "Installing")]
    Installing,
    #[serde(rename = "Installed")]
    Installed,
    #[serde(rename = "InstallRebooting")]
    InstallRebooting,
    #[serde(rename = "InstallScheduled")]
    InstallScheduled,
    #[serde(rename = "InstallVerificationFailed")]
    InstallVerificationFailed,
    #[serde(rename = "InvalidSignature")]
    InvalidSignature,
    #[serde(rename = "SignatureVerified")]
    SignatureVerified,
}

impl Default for FirmwareStatusEnumType {
    fn default() -> Self {
        Self::Idle
    }
}
