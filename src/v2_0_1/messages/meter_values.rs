//! MeterValues
use crate::v2_0_1::datatypes::meter_value_type::MeterValueType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// This contains a number (>0) designating an EVSE of the Charging Station. ‘0’ (zero) is used to designate the main power meter.
    pub evse_id: i32,
    /// The sampled meter values with timestamps.
    pub meter_value: Vec<MeterValueType>,
}

/// Sent by the CSMS to the Charging Station in response to a MeterValuesRequest.
///
/// This message is deprecated. This message might be removed in a future version of OCPP. It will be replaced by Device Management Monitoring events.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {
    // No fields are defined
}
