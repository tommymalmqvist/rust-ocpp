use serde::{Deserialize, Serialize};

/// This field contains criteria for components for which a report is requested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentCriterionEnumType {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Available")]
    Available,
    #[serde(rename = "Enabled")]
    Enabled,
    #[serde(rename = "Problem")]
    Problem,
}

impl Default for ComponentCriterionEnumType {
    fn default() -> Self {
        Self::Available
    }
}
