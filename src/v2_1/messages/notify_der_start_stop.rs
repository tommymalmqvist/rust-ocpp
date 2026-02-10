use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::CustomData;

/// Request message for NotifyDERStartStop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,

    /// Id of the started or stopped DER control.
    /// Corresponds to the _controlId_ of the SetDERControlRequest.
    #[validate(length(max = 36))]
    pub control_id: String,

    /// True if DER control has started. False if it has ended.
    pub started: bool,

    /// Time of start or end of event.
    pub timestamp: DateTime<Utc>,

    /// List of controlIds that are superseded as a result of this control starting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub superseded_ids: Option<Vec<String>>,
}

/// Response message for NotifyDERStartStop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}
