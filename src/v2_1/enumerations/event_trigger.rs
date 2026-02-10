use serde::{Deserialize, Serialize};

/// Type of trigger for this event, e.g. exceeding a threshold value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventTriggerEnumType {
    #[serde(rename = "Alerting")]
    Alerting,
    #[serde(rename = "Delta")]
    Delta,
    #[serde(rename = "Periodic")]
    Periodic,
}

impl Default for EventTriggerEnumType {
    fn default() -> Self {
        Self::Alerting
    }
}
