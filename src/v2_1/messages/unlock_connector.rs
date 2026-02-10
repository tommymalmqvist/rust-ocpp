use super::{CustomData, StatusInfo, UnlockStatusEnum};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UnlockConnectorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub evse_id: i32,
    pub connector_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct UnlockConnectorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub status: UnlockStatusEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfo>,
}

impl UnlockConnectorRequest {
    pub fn new(evse_id: i32, connector_id: i32) -> Self {
        Self {
            custom_data: None,
            evse_id,
            connector_id,
        }
    }
}

impl UnlockConnectorResponse {
    pub fn new(status: UnlockStatusEnum) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
