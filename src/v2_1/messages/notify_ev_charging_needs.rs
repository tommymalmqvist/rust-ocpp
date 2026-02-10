use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        ACChargingParametersType, CustomDataType, DCChargingParametersType,
        DERChargingParametersType, EVEnergyOfferType, StatusInfoType, V2XChargingParametersType,
    },
    enumerations::{
        ControlModeEnumType, EnergyTransferModeEnumType, MobilityNeedsModeEnumType,
        NotifyEVChargingNeedsStatusEnumType,
    },
};

/// Charging needs of an EV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// Optional. AC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,

    /// Optional. DER charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub der_charging_parameters: Option<DERChargingParametersType>,

    /// Optional. EV energy offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_offer: Option<EVEnergyOfferType>,

    /// Required. Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,

    /// Optional. DC charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,

    /// Optional. V2X charging parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_charging_parameters: Option<V2XChargingParametersType>,

    /// Optional. Modes of energy transfer that are marked as available by EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub available_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,

    /// Optional. Control mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_mode: Option<ControlModeEnumType>,

    /// Optional. Mobility needs mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>,

    /// Optional. Estimated departure time of the EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to notify the CSMS about EV charging needs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsRequest {
    /// Optional. Contains the maximum schedule tuples the car supports per schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_schedule_tuples: Option<i32>,

    /// Required. Charging needs of the EV.
    pub charging_needs: ChargingNeedsType,

    /// Required. Defines the EVSE and connector to which the EV is connected. EvseId may not be 0.
    #[validate(range(min = 1))]
    pub evse_id: i32,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a NotifyEVChargingNeedsRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NotifyEVChargingNeedsResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: NotifyEVChargingNeedsStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
