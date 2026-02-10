use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// The updated reservation status enumeration type.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReservationUpdateStatusEnumType {
    Expired,
    Removed,
    NoTransaction,
}

/// Request body for the ReservationStatusUpdate request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The ID of the reservation.
    #[validate(range(min = 0))]
    pub reservation_id: i32,

    /// Required. The updated reservation status.
    pub reservation_update_status: ReservationUpdateStatusEnumType,
}

/// Response body for the ReservationStatusUpdate response.
/// This contains no fields as per the OCPP 2.1 specification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReservationStatusUpdateResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
