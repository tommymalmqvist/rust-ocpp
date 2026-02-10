use serde::{Deserialize, Serialize};

/// Type of message to be triggered.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageTriggerEnumType {
    #[serde(rename = "BootNotification")]
    BootNotification,
    #[serde(rename = "LogStatusNotification")]
    LogStatusNotification,
    #[serde(rename = "FirmwareStatusNotification")]
    FirmwareStatusNotification,
    #[serde(rename = "Heartbeat")]
    Heartbeat,
    #[serde(rename = "MeterValues")]
    MeterValues,
    #[serde(rename = "SignChargingStationCertificate")]
    SignChargingStationCertificate,
    #[serde(rename = "SignV2GCertificate")]
    SignV2GCertificate,
    #[serde(rename = "SignV2G20Certificate")]
    SignV2G20Certificate,
    #[serde(rename = "StatusNotification")]
    StatusNotification,
    #[serde(rename = "TransactionEvent")]
    TransactionEvent,
    #[serde(rename = "SignCombinedCertificate")]
    SignCombinedCertificate,
    #[serde(rename = "PublishFirmwareStatusNotification")]
    PublishFirmwareStatusNotification,
    #[serde(rename = "CustomTrigger")]
    CustomTrigger,
}
