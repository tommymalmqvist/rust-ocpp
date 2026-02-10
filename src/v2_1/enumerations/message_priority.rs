#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessagePriorityEnumType {
    #[serde(rename = "AlwaysFront")]
    AlwaysFront,
    #[serde(rename = "InFront")]
    InFront,
    #[default]
    #[serde(rename = "NormalCycle")]
    NormalCycle,
}
