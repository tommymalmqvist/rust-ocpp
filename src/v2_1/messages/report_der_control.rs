use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::{
    CustomDataType, DERCurveGetType, EnterServiceGetType, FixedPFGetType, FixedVarGetType,
    FreqDroopGetType, GradientGetType, LimitMaxDischargeGetType,
};

/// Request body for the ReportDERControl request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDERControlRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. RequestId from GetDERControlRequest.
    pub request_id: i32,

    /// Optional. To Be Continued. Default value when omitted: false. false indicates that there are no further messages as part of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbc: Option<bool>,

    /// Optional. Array of DER curve settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub curve: Option<Vec<DERCurveGetType>>,

    /// Optional. Array of enter service settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub enter_service: Option<Vec<EnterServiceGetType>>,

    /// Optional. Array of fixed power factor settings for absorbing reactive power.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub fixed_pf_absorb: Option<Vec<FixedPFGetType>>,

    /// Optional. Array of fixed power factor settings for injecting reactive power.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub fixed_pf_inject: Option<Vec<FixedPFGetType>>,

    /// Optional. Array of fixed var settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub fixed_var: Option<Vec<FixedVarGetType>>,

    /// Optional. Array of frequency droop settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub freq_droop: Option<Vec<FreqDroopGetType>>,

    /// Optional. Array of gradient settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub gradient: Option<Vec<GradientGetType>>,

    /// Optional. Array of maximum discharge limit settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 24))]
    pub limit_max_discharge: Option<Vec<LimitMaxDischargeGetType>>,
}

/// Response body for the ReportDERControl response.
/// This contains no fields as per the OCPP 2.1 specification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDERControlResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
