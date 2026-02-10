use serde::{Deserialize, Serialize};

/// Indicates the kind of schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChargingProfileKindEnumType {
    #[serde(rename = "Absolute")]
    Absolute,
    #[serde(rename = "Recurring")]
    Recurring,
    #[serde(rename = "Relative")]
    Relative,
    #[serde(rename = "Dynamic")]
    Dynamic,
}
