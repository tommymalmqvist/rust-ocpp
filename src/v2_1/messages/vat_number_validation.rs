use super::{Address, CustomData, GenericStatusEnum, StatusInfo};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VatNumberValidationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfo>,
    pub vat_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<i32>,
    pub status: GenericStatusEnum,
}

impl VatNumberValidationRequest {
    pub fn new(vat_number: String) -> Self {
        Self {
            custom_data: None,
            vat_number,
            evse_id: None,
        }
    }
}

impl VatNumberValidationResponse {
    pub fn new(vat_number: String, status: GenericStatusEnum) -> Self {
        Self {
            custom_data: None,
            company: None,
            status_info: None,
            vat_number,
            evse_id: None,
            status,
        }
    }
}
