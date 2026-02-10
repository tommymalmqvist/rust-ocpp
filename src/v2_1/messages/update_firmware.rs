use super::{CustomData, Firmware, StatusInfo, UpdateFirmwareStatusEnum};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateFirmwareRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
    pub request_id: i32,
    pub firmware: Firmware,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateFirmwareResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub status: UpdateFirmwareStatusEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfo>,
}

impl UpdateFirmwareRequest {
    pub fn new(request_id: i32, firmware: Firmware) -> Self {
        Self {
            custom_data: None,
            retries: None,
            retry_interval: None,
            request_id,
            firmware,
        }
    }
}

impl UpdateFirmwareResponse {
    pub fn new(status: UpdateFirmwareStatusEnum) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
