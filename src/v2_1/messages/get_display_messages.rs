use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{
        GetDisplayMessagesStatusEnumType, MessagePriorityEnumType, MessageStateEnumType,
    },
};

/// Request to get the display messages from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesRequest {
    /// Required. The Id of this request.
    pub request_id: i32,

    /// Optional. If provided the Charging Station shall return Display Messages of the given ids.
    /// This field SHALL NOT contain more ids than set in NumberOfDisplayMessages.maxLimit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub id: Option<Vec<i32>>,

    /// Optional. If provided the Charging Station shall return Display Messages with the given priority only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<MessagePriorityEnumType>,

    /// Optional. If provided the Charging Station shall return Display Messages with the given state only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MessageStateEnumType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetDisplayMessagesRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetDisplayMessagesResponse {
    /// Required. Indicates if the Charging Station has Display Messages that match
    /// the request criteria in the GetDisplayMessagesRequest.
    pub status: GetDisplayMessagesStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
