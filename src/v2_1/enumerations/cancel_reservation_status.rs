use serde::{Deserialize, Serialize};

/// This indicates the success or failure of the canceling of a reservation by CSMS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CancelReservationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
}
