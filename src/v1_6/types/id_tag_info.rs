use chrono::{DateTime, Utc};

use super::AuthorizationStatus;
use crate::v1_6::helpers::datetime_rfc3339;
use validator::Validate;

/// Contains status information about an identifier. It is returned in Authorize, Start Transaction and Stop Transaction responses. If expiryDate is not given, the status has no end date.
#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    /// Optional. This contains the date at which idTag should be removed from the Authorization Cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "datetime_rfc3339::option")]
    pub expiry_date: Option<DateTime<Utc>>,
    /// Optional. This contains the parent-identifier. IdToken
    #[validate(length(min = 1, max = 20))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id_tag: Option<String>,
    /// Required. This contains whether the idTag has been accepted or not by the Central System.
    pub status: AuthorizationStatus,
}
