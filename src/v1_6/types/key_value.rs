#[cfg(not(feature = "std"))]
use alloc::string::String;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
pub struct KeyValue {
    /// Required.
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 50)))]
    pub key: String,
    /// Required. False if the value can be set with the ChangeConfiguration message.
    pub readonly: bool,
    /// Optional. If key is known but not set, this field may be absent.
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 500)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
