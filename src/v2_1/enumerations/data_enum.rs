use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataEnumType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "decimal")]
    Decimal,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "dateTime")]
    DateTime,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "OptionList")]
    OptionList,
    #[serde(rename = "SequenceList")]
    SequenceList,
    #[serde(rename = "MemberList")]
    MemberList,
}

impl Default for DataEnumType {
    fn default() -> Self {
        DataEnumType::String
    }
}
