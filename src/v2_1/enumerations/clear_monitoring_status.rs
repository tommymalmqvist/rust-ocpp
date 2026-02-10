use serde::{Deserialize, Serialize};

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearMonitoringStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotFound")]
    NotFound,
}

impl Default for ClearMonitoringStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
