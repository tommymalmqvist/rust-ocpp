use crate::v1_6::types::MeterValue;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// This contains the field definition of the MeterValues.req PDU sent by the Charge Point to the Central System. See also Meter Values
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    /// Required. This contains a number (>0) designating a connector of the Charge Point.‘0’ (zero) is used to designate the main powermeter.
    pub connector_id: u32,
    /// Optional. The transaction to which these meter samples are related.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i32>,
    /// Required. The sampled meter values with timestamps.
    pub meter_value: Vec<MeterValue>,
}

/// This contains the field definition of the MeterValues.conf PDU sent by the Central System to the Charge Point in response to a MeterValuesRequest PDU. See also Meter Values
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesResponse {
    // No fields are defined.
}
