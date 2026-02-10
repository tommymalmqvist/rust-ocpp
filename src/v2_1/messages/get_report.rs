use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{ComponentVariableType, CustomDataType, StatusInfoType},
    enumerations::{ComponentCriterionEnumType, GenericDeviceModelStatusEnumType},
};

/// Request to get a report from a Charging Station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetReportRequest {
    /// Required. The Id of the request.
    pub request_id: i32,

    /// Optional. This field contains criteria for components for which a report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4))]
    pub component_criteria: Option<Vec<ComponentCriterionEnumType>>,

    /// Optional. This field specifies the components and variables for which a report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub component_variable: Option<Vec<ComponentVariableType>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a GetReportRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetReportResponse {
    /// Required. This field indicates whether the Charging Station was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,

    /// Optional. Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
