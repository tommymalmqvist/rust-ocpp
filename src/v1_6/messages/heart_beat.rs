use chrono::{DateTime, Utc};
use validator::Validate;

use crate::v1_6::helpers::datetime_rfc3339;

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatRequest {}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    /// # From OCPP Specification
    /// Required. This contains the current time of the Central System.
    #[serde(with = "datetime_rfc3339")]
    pub current_time: DateTime<Utc>,
}
