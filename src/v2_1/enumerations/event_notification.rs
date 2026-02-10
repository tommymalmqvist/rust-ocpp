use serde::{Deserialize, Serialize};

/// Specifies the event notification type of the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventNotificationEnumType {
    #[serde(rename = "HardWiredNotification")]
    HardWiredNotification,
    #[serde(rename = "HardWiredMonitor")]
    HardWiredMonitor,
    #[serde(rename = "PreconfiguredMonitor")]
    PreconfiguredMonitor,
    #[serde(rename = "CustomMonitor")]
    CustomMonitor,
}
