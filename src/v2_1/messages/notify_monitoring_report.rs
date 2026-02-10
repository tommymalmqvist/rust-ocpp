use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{ComponentType, CustomDataType, VariableMonitoringType, VariableType};

/// Class to hold parameters of SetVariableMonitoring request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringDataType {
    /// Required. Component for which monitoring is configured.
    pub component: ComponentType,

    /// Required. Variable for which monitoring is configured.
    pub variable: VariableType,

    /// Required. List of configured monitoring settings.
    #[validate(length(min = 1))]
    pub variable_monitoring: Vec<VariableMonitoringType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to notify the CSMS about monitoring events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportRequest {
    /// Optional. List of monitoring data.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub monitor: Option<Vec<MonitoringDataType>>,

    /// Required. The id of the GetMonitoringRequest that requested this report.
    pub request_id: i32,

    /// Optional. "to be continued" indicator. Indicates whether another part of the monitoring data follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Required. Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Required. Timestamp of when this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyMonitoringReportRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyMonitoringReportResponse {
    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
