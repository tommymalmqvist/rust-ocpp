use crate::v2_1::helpers::datetime_rfc3339;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::CustomData;

/// Request message for NotifyDERStartStop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,

    /// Id of the started or stopped DER control.
    /// Corresponds to the _controlId_ of the SetDERControlRequest.
    #[cfg_attr(feature = "std", validate(length(max = 36)))]
    pub control_id: String,

    /// True if DER control has started. False if it has ended.
    pub started: bool,

    /// Time of start or end of event.
    #[serde(with = "datetime_rfc3339")]
    pub timestamp: DateTime<Utc>,

    /// List of controlIds that are superseded as a result of this control starting.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 24)))]
    pub superseded_ids: Option<Vec<String>>,
}

/// Response message for NotifyDERStartStop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERStartStopResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
}
