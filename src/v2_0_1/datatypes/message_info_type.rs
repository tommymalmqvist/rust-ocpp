use chrono::DateTime;
use chrono::Utc;

use super::component_type::ComponentType;
use super::message_content_type::MessageContentType;
use crate::v2_0_1::enumerations::message_priority_enum_type::MessagePriorityEnumType;
use crate::v2_0_1::enumerations::message_state_enum_type::MessageStateEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Contains message details, for a message to be displayed on a Charging Station.
/// MessageInfoType is used by: SetDisplayMessageRequest , NotifyDisplayMessagesRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    pub id: i32,
    pub priority: MessagePriorityEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub start_date_time: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub end_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub message: MessageContentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<ComponentType>,
}
