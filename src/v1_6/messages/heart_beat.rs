use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// # From OCPP Specification
    /// Required. This contains the current time of the Central System.
    pub current_time: DateTime<Utc>,
}
