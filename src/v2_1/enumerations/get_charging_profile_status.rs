use serde::{Deserialize, Serialize};

/// This indicates whether the Charging Station is able to process this request and will send ReportChargingProfilesRequest messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetChargingProfileStatusEnumType {
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "NoProfiles")]
    NoProfiles,
}

impl Default for GetChargingProfileStatusEnumType {
    fn default() -> Self {
        Self::Accepted
    }
}
