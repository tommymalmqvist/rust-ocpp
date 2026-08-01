use crate::v2_1::helpers::datetime_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::GenericStatusEnumType,
};

/// Request body for the AFRRSignal request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Value of signal in v2xSignalWattCurve.
    pub signal: i32,

    /// Required. Time when signal becomes active.
    #[serde(with = "datetime_rfc3339")]
    pub timestamp: DateTime<Utc>,
}

/// Response body for the AFRRSignal response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AFRRSignalResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: GenericStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
