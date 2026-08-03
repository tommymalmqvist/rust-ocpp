#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::monitoring_data_type::MonitoringDataType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// This contains the field definition of the NotifyMonitoringRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(with = "datetime_rfc3339 ")]
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor: Option<Vec<MonitoringDataType>>,
}

/// Response to a NotifyMonitoringRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {}
