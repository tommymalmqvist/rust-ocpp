use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// Request to notify the CSMS about customer information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationRequest {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// (Part of) the requested data. No format specified in which the data is returned. Should be human readable.
    #[validate(length(max = 512))]
    pub data: String,

    /// "to be continued" indicator. Indicates whether another part of the monitoringData follows in an upcoming notifyMonitoringReportRequest message. Default value when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Sequence number of this message. First message starts at 0.
    #[validate(range(min = 0))]
    pub seq_no: i32,

    /// Timestamp of the moment this message was generated at the Charging Station.
    pub generated_at: DateTime<Utc>,

    /// The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,
}

/// Response to a NotifyCustomerInformationRequest. This message has no fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyCustomerInformationResponse {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
