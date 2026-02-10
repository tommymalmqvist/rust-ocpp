use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{custom_data::CustomDataType, meter_value::MeterValueType};

/// Request sent by the Charging Station to the CSMS to provide meter values for a specific EVSE.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains a number (>0) designating an EVSE of the Charging Station.
    /// '0' (zero) is used to designate the main power meter.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Required. The sampled meter values with timestamps.
    #[validate(length(min = 1))]
    pub meter_value: Vec<MeterValueType>,
}

/// Response sent by the CSMS to the Charging Station in response to a MeterValuesRequest.
/// This message is deprecated. This message might be removed in a future version of OCPP.
/// It will be replaced by Device Management Monitoring events.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {
    /// Custom data from the CSMS.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
