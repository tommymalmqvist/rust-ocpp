use super::IdTagInfo;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Elements that constitute an entry of a Local Authorization List update.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    /// Required. The identifier to which this authorization applies.
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 20)))]
    pub id_tag: String,
    /// Optional. (Required when UpdateType is Full) This contains information about authorization status, expiry and parent id. For a Differential update the following applies: If this element is present, then this entry SHALL be added or updated in the Local Authorization List. If this element is absent, than the entry for this idtag in the Local Authorization List SHALL be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_tag_info: Option<IdTagInfo>,
}
