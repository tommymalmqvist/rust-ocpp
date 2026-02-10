use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ComponentType, CustomDataType, MessageContentType},
    enumerations::{MessagePriorityEnumType, MessageStateEnumType},
};

/// Contains message details, for a message to be displayed on a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Optional. Display component that this message concerns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,

    /// Required. Unique id within an exchange context. It is defined within the OCPP context as a positive Integer value (greater or equal to zero).
    #[validate(range(min = 0))]
    pub id: i32,

    /// Required. With what priority should this message be shown.
    pub priority: MessagePriorityEnumType,

    /// Optional. During what state should this message be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,

    /// Optional. From what date-time should this message be shown. If omitted: directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<DateTime<Utc>>,

    /// Optional. Until what date-time should this message be shown, after this date/time this message SHALL be removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<DateTime<Utc>>,

    /// Optional. During which transaction shall this message be shown.
    /// Message SHALL be removed by the Charging Station after transaction has ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36))]
    pub transaction_id: Option<String>,

    /// Required. Contains message details.
    pub message: MessageContentType,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to notify the CSMS about display messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesRequest {
    /// Required. Id of this request for identification in response.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Optional. "to be continued" indicator. Indicates whether another part of the report follows in an upcoming notifyDisplayMessagesRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Required. Array of message info objects.
    #[validate(length(min = 1))]
    pub message_info: Vec<MessageInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyDisplayMessagesRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyDisplayMessagesResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
