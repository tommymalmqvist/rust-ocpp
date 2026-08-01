use crate::v2_1::helpers::datetime_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::v2_1::{
    datatypes::{CustomDataType, IdTokenType, StatusInfoType},
    enumerations::{connector::ConnectorEnumType, ReserveNowStatusEnumType},
};

/// Request body for the ReserveNow request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Id of reservation.
    #[cfg_attr(feature = "std", validate(range(min = 0)))]
    pub id: i32,

    /// Required. Date and time at which the reservation expires.
    #[serde(with = "datetime_rfc3339")]
    pub expiry_date_time: DateTime<Utc>,

    /// Optional. This field specifies the connector type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<ConnectorEnumType>,

    /// Required. Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Optional. This contains ID of the evse to be reserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "std", validate(range(min = 0)))]
    pub evse_id: Option<i32>,

    /// Optional. Group authorization reference to use when starting a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
}

/// Response body for the ReserveNow response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct ReserveNowResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This indicates the success or failure of the reservation.
    pub status: ReserveNowStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
