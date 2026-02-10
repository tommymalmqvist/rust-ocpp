#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum SetVariableStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "UnknownComponent")]
    UnknownComponent,
    #[serde(rename = "UnknownVariable")]
    UnknownVariable,
    #[serde(rename = "NotSupportedAttributeType")]
    NotSupportedAttributeType,
    #[serde(rename = "RebootRequired")]
    RebootRequired,
}
