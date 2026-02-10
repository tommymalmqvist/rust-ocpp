use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ComponentType, CustomDataType, VariableType};
use crate::v2_1::enumerations::{EventNotificationEnumType, EventTriggerEnumType};

/// Class to report an event notification for a component-variable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    /// Required. Actual value of the variable.
    #[validate(length(max = 2500))]
    pub actual_value: String,

    /// Required. Identifies the event. This field can be referred to as a cause by other events.
    #[validate(range(min = 0))]
    pub event_id: i32,

    /// Required. Timestamp of when the event occurred.
    pub timestamp: DateTime<Utc>,

    /// Required. Trigger type of the event.
    pub trigger: EventTriggerEnumType,

    /// Optional. If an event notification is linked to a specific transaction, this field can be used to specify its transactionId.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Required. The component for which this event applies.
    pub component: ComponentType,

    /// Optional. Identifies the VariableMonitoring which triggered the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_monitoring_id: Option<i32>,

    /// Required. Type of notification of the event.
    pub event_notification: EventNotificationEnumType,

    /// Required. The variable for which this event applies.
    pub variable: VariableType,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to notify the CSMS about an event that occurred at the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventRequest {
    /// Required. The actual event data.
    #[validate(length(min = 1))]
    pub event_data: Vec<EventDataType>,

    /// Optional. "to be continued" indicator. Indicates whether another part of the report follows in an upcoming notifyEventRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Required. Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Required. Timestamp of when this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyEventRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEventResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
