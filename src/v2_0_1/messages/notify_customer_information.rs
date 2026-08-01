#[cfg(not(feature = "std"))]
use alloc::string::String;
use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::helpers::datetime_rfc3339;

/// This contains the field definition of the NotifyCustomerInformationRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(with = "datetime_rfc3339 ")]
    pub generated_at: DateTime<Utc>,
    pub request_id: i32,
}

/// The NotifyChargingLimitResponse message is sent by the CSMS to the Charging Station in response to a NotifyChargingLimitsRequest. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationResponse {}
