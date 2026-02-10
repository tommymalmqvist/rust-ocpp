use super::{CustomData, UnpublishFirmwareStatusEnum};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UnpublishFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub checksum: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UnpublishFirmwareResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub status: UnpublishFirmwareStatusEnum,
}

impl UnpublishFirmwareRequest {
    pub fn new(checksum: String) -> Self {
        Self {
            custom_data: None,
            checksum,
        }
    }
}

impl UnpublishFirmwareResponse {
    pub fn new(status: UnpublishFirmwareStatusEnum) -> Self {
        Self {
            custom_data: None,
            status,
        }
    }
}
