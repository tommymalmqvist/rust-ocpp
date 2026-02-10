use super::{CustomData, PriorityChargingStatusEnum, StatusInfo};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub transaction_id: String,
    pub activate: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UsePriorityChargingResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub status: PriorityChargingStatusEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfo>,
}

impl UsePriorityChargingRequest {
    pub fn new(transaction_id: String, activate: bool) -> Self {
        Self {
            custom_data: None,
            transaction_id,
            activate,
        }
    }
}

impl UsePriorityChargingResponse {
    pub fn new(status: PriorityChargingStatusEnum) -> Self {
        Self {
            custom_data: None,
            status,
            status_info: None,
        }
    }
}
