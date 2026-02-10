use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RegistrationStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Rejected")]
    Rejected,
}
