use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to accept this request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenericDeviceModelStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "NotSupported")]
    NotSupported,
    #[serde(rename = "EmptyResultSet")]
    EmptyResultSet,
}

impl Default for GenericDeviceModelStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
