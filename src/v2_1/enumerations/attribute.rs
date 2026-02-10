use serde::{Deserialize, Serialize};

/// Attribute: Actual, Target, MinSet, MaxSet.
/// Defaults to Actual if absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeEnumType {
    #[serde(rename = "Actual")]
    Actual,
    #[serde(rename = "Target")]
    Target,
    #[serde(rename = "MinSet")]
    MinSet,
    #[serde(rename = "MaxSet")]
    MaxSet,
}

impl Default for AttributeEnumType {
    fn default() -> Self {
        Self::Actual
    }
}
