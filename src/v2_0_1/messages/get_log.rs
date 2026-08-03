//! GetLog
use crate::v2_0_1::datatypes::log_parameters_type::LogParametersType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::log_enum_type::LogEnumType;
use crate::v2_0_1::enumerations::log_status_enum_type::LogStatusEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// GetLogRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetLogRequest {
    /// This contains the type of log file that theCharging Station should send
    pub log_type: LogEnumType,
    /// The Id of this request
    pub request_id: i32,
    /// This specifies how many times the ChargingStation must try to upload the log before giving up. If thisfield is not present, it is left to Charging Station to decidehow many times it wants to retry
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// The interval in seconds after which a retry maybe attempted. If this field is not present, it is left toCharging Station to decide how long to wait between attempts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    /// This field specifies the requested log and thelocation to which the log should be sent.
    pub log: LogParametersType,
}

/// GetLogResponse, sent by the Charging Station to the CSMS in response to a GetLogRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetLogResponse {
    /// This field indicates whether the ChargingStation was able to accept the request.
    pub status: LogStatusEnumType,
    /// This contains the name of the log file that willbe uploaded. This field is not present when no logginginformation is available.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 255)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
