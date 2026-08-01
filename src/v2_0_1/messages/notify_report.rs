#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::report_data_type::ReportDataType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// This contains the field definition of the NotifyReportRequest PDU sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportRequest {
    pub request_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,
    pub seq_no: i32,
    #[serde(with = "datetime_rfc3339 ")]
    pub generated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_data: Option<Vec<ReportDataType>>,
}

/// Response to a NotifyReportRequest message. No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotifyReportResponse {}
