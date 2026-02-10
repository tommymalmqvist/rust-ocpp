use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// This contains the current status of the Connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorStatusEnumType {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

/// Request to notify the CSMS about a status change of a connector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The time for which the status is reported.
    pub timestamp: DateTime<Utc>,

    /// Required. The current status of the Connector.
    pub connector_status: ConnectorStatusEnumType,

    /// Required. The id of the EVSE to which the connector belongs for which the status is reported.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. The id of the connector within the EVSE for which the status is reported.
    #[validate(range(min = 0))]
    pub connector_id: i32,
}

/// Response to a StatusNotificationRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
