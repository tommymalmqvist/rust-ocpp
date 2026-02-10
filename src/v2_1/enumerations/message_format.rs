use serde::{Deserialize, Serialize};

/// Format of the message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageFormatEnumType {
    #[serde(rename = "ASCII")]
    ASCII,
    #[serde(rename = "HTML")]
    HTML,
    #[serde(rename = "URI")]
    URI,
    #[serde(rename = "UTF8")]
    UTF8,
    #[serde(rename = "QRCODE")]
    QRCODE,
}
