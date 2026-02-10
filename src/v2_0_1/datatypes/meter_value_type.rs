use chrono::DateTime;
use chrono::Utc;

use super::sampled_value_type::SampledValueType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Collection of one or more sampled values in MeterValuesRequest and TransactionEvent. All sampled values in a MeterValue are sampled at the same point in time.
/// MeterValueType is used by: MeterValuesRequest , TransactionEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    #[serde(with = "datetime_rfc3339 ")]
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValueType>,
}
