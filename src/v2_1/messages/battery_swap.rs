use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, IdTokenType},
    enumerations::BatterySwapEventEnumType,
};

/// Battery data information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatteryDataType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Slot number where battery is inserted or removed.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Optional. Production date of battery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_date: Option<DateTime<Utc>>,

    /// Required. Serial number of battery.
    #[validate(length(max = 50))]
    pub serial_number: String,

    /// Required. State of charge.
    #[validate(range(min = 0.0, max = 100.0))]
    pub so_c: f64,

    /// Required. State of health.
    #[validate(range(min = 0.0, max = 100.0))]
    pub so_h: f64,

    /// Optional. Vendor-specific info from battery in undefined format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub vendor_info: Option<String>,
}

/// Request body for the BatterySwap request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapRequest {
    /// Required. Array of battery data.
    #[validate(length(min = 1))]
    pub battery_data: Vec<BatteryDataType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Type of battery swap event.
    pub event_type: BatterySwapEventEnumType,

    /// Required. Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Required. RequestId to correlate BatteryIn/Out events and optional RequestBatterySwapRequest.
    pub request_id: i32,
}

/// Response body for the BatterySwap response.
/// This is an empty response that just acknowledges receipt of the request. (The request cannot be rejected).
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BatterySwapResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
