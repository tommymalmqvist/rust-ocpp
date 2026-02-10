use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{GenericDeviceModelStatusEnumType, ReportBaseEnumType},
};

/// Request to get a base report from the Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportRequest {
    /// Required. The Id of the request.
    #[validate(range(min = 0))]
    pub request_id: i32,

    /// Required. This field specifies the report base.
    pub report_base: ReportBaseEnumType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetBaseReportRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetBaseReportResponse {
    /// Required. This indicates whether the Charging Station is able to accept this request.
    pub status: GenericDeviceModelStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
