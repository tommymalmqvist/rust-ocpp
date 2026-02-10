use serde::{Deserialize, Serialize};

/// Type of EVSE (AC, DC) this tariff applies to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvseKindEnumType {
    #[serde(rename = "AC")]
    AC,
    #[serde(rename = "DC")]
    DC,
}
