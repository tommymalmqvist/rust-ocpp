use crate::v2_0_1::enumerations::message_format_enum_type::MessageFormatEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Contains message details, for a message to be displayed on a Charging Station.
/// MessageContentType is used by: Common:IdTokenInfoType , Common:MessageInfoType , TransactionEventResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    pub format: MessageFormatEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub content: String,
}
