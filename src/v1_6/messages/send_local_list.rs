use crate::v1_6::types::{AuthorizationData, UpdateStatus, UpdateType};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// This contains the field definition of the SendLocalListRequest PDU sent by the Central System to the Charge Point. If no (empty) local_authorization_list is given and the updateType is Full, all identifications are removed from the list. Requesting a Differential update without (empty) local_authorization_list will have no effect on the list. All idTags in the local_authorization_list MUST be unique, no duplicate values are allowed. See also Send Local List
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListRequest {
    /// Required. In case of a full update this is the version number of the full list. In case of a differential update it is the version number of the list after the update has been applied.
    pub list_version: i32,
    /// Optional. In case of a full update this contains the list of values that form the new local authorization list. In case of a differential update it contains the changes to be applied to the local authorization list in the Charge Point. Maximum number of AuthorizationData elements is available in the configuration key: SendLocalListMaxLength
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_authorization_list: Option<Vec<AuthorizationData>>,
    /// Required. This contains the type of update (full or differential) of this request.
    pub update_type: UpdateType,
}

/// This contains the field definition of the ResetResponse PDU sent by the Charge Point to the Central System inresponse to a ResetRequest PDU. See also Reset
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SendLocalListResponse {
    // Required. This indicates whether the Charge Point is able to perform the reset.
    pub status: UpdateStatus,
}
