use super::super::datatypes::CustomDataType;
use super::super::datatypes::StatusInfoType;
use super::super::enumerations::CancelReservationStatusEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Request to cancel a reservation.
///
/// This message is sent by the CSMS to the Charging Station to cancel an existing reservation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct CancelReservationRequest {
    /// Id of the reservation to cancel.
    #[validate(range(min = 0))]
    #[serde(rename = "reservationId")]
    pub reservation_id: i32,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a CancelReservationRequest.
///
/// This message is sent by the Charging Station to the CSMS in response to a CancelReservationRequest.
/// It indicates whether the Charging Station was able to cancel the reservation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CancelReservationResponse {
    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// This indicates the success or failure of the canceling of a reservation by CSMS.
    pub status: CancelReservationStatusEnumType,

    /// Detailed status information.
    ///
    /// This field can be used to provide additional information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl CancelReservationRequest {
    pub fn new(reservation_id: i32) -> Self {
        Self {
            reservation_id,
            custom_data: None,
        }
    }
}

impl CancelReservationResponse {
    pub fn new(status: CancelReservationStatusEnumType) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
