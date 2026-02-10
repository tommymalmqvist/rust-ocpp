use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::CustomDataType,
    enumerations::{DERControlEnumType, GridEventFaultEnumType},
};

/// Request to notify the CSMS about a DER alarm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERAlarmRequest {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Name of DER control, e.g. LFMustTrip.
    pub control_type: DERControlEnumType,

    /// Type of grid event that caused this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_event_fault: Option<GridEventFaultEnumType>,

    /// True when error condition has ended.
    /// Absent or false when alarm has started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_ended: Option<bool>,

    /// Time of start or end of alarm.
    pub timestamp: DateTime<Utc>,

    /// Optional info provided by EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 200))]
    pub extra_info: Option<String>,
}

/// Response to a NotifyDERAlarmRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDERAlarmResponse {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
