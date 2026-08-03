//! GetMonitoringReport
use crate::v2_0_1::datatypes::component_variable_type::ComponentVariableType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::generic_device_model_status_enum_type::GenericDeviceModelStatusEnumType;
use crate::v2_0_1::enumerations::monitoring_criterion_enum_type::MonitoringCriterionEnumType;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// GetMonitoringReportRequest, sent by the CSMS to the Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportRequest {
    /// The Id of the request.
    pub request_id: i32,
    ///  This field contains criteria for components forwhich a monitoring report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_criteria: Option<Vec<MonitoringCriterionEnumType>>,
    /// This field specifies the components andvariables for which a monitoring report is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_variable: Option<Vec<ComponentVariableType>>,
}

/// GetMonitoringReportResponse, sent by the Charging Station to the CSMS.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitoringReportResponse {
    /// This field indicates whether the ChargingStation was able to accept the request.
    pub status: GenericDeviceModelStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
