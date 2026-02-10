use serde::{Deserialize, Serialize};

/// This contains the type of availability change that the Charging Station should perform.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OperationalStatusEnumType {
    #[serde(rename = "Inoperative")]
    Inoperative,
    #[serde(rename = "Operative")]
    Operative,
}
