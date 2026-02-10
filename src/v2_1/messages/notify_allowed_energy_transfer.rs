use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{EnergyTransferModeEnumType, NotifyAllowedEnergyTransferStatusEnumType},
};

/// Request to notify the Charging Station about the allowed energy transfer modes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. The transaction for which the allowed energy transfer is allowed.
    #[validate(length(max = 36))]
    pub transaction_id: String,

    /// Required. Modes of energy transfer that are accepted by CSMS.
    #[validate(length(min = 1))]
    pub allowed_energy_transfer: Vec<EnergyTransferModeEnumType>,
}

/// Response to a NotifyAllowedEnergyTransferRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyAllowedEnergyTransferResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: NotifyAllowedEnergyTransferStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
