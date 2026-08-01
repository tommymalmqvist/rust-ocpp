#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use chrono::DateTime;
use chrono::Utc;

use super::SampledValue;

/// Collection of one or more sampled values in MeterValues.req and StopTransaction.req. All sampled values in a MeterValue are sampled at the same point in time.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    /// Required. Timestamp for measured value(s).
    pub timestamp: DateTime<Utc>,
    /// Required. One or more measured values
    pub sampled_value: Vec<SampledValue>,
}
