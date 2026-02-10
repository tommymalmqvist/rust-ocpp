use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BootReasonEnumType {
    #[serde(rename = "ApplicationReset")]
    ApplicationReset,
    #[serde(rename = "FirmwareUpdate")]
    FirmwareUpdate,
    #[serde(rename = "LocalReset")]
    LocalReset,
    #[serde(rename = "PowerUp")]
    PowerUp,
    #[serde(rename = "RemoteReset")]
    RemoteReset,
    #[serde(rename = "ScheduledReset")]
    ScheduledReset,
    #[serde(rename = "Triggered")]
    Triggered,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Watchdog")]
    Watchdog,
}
