//! GetDisplayMessages
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::get_display_messages_status_enum_type::GetDisplayMessagesStatusEnumType;
use crate::v2_0_1::enumerations::message_priority_enum_type::MessagePriorityEnumType;
use crate::v2_0_1::enumerations::message_state_enum_type::MessageStateEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// GetCompositeScheduleRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    /// If provided the Charging Station shall returnDisplay Messages of the given ids. This field SHALL NOTcontain more ids than set inNumberOfDisplayMessages.maxLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<i32>>,
    /// The Id of this request
    pub request_id: i32,
    /// If provided the Charging Station shall returnDisplay Messages with the given priority only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,
    /// If provided the Charging Station shall returnDisplay Messages with the given state only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,
}

/// GetCompositeScheduleResponse, sent by the Charging Station to the CSMS in response to a GetCompositeScheduleRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    ///  Indicates if the Charging Station has DisplayMessages that match the request criteria in theGetDisplayMessagesRequest
    pub status: GetDisplayMessagesStatusEnumType,
    /// Detailed status information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
