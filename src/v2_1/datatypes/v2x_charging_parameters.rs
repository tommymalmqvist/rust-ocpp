use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Charging parameters for ISO 15118-20, also supporting V2X charging/discharging.
/// All values are greater or equal to zero, with the exception of EVMinEnergyRequest, EVMaxEnergyRequest, EVTargetEnergyRequest, EVMinV2XEnergyRequest and EVMaxV2XEnergyRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct V2XChargingParametersType {
    /// Minimum charge power in W, defined by max(EV, EVSE).
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumChargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_power: Option<Decimal>,

    /// Minimum charge power on phase L2 in W, defined by max(EV, EVSE).
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumChargePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_power_l2: Option<Decimal>,

    /// Minimum charge power on phase L3 in W, defined by max(EV, EVSE).
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumChargePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_power_l3: Option<Decimal>,

    /// Maximum charge (absorbed) power in W, defined by min(EV, EVSE) at unity power factor.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    /// It corresponds to the ChaWMax attribute in the IEC 61850.
    /// It is usually equivalent to the rated apparent power of the EV when discharging (ChaVAMax) in IEC 61850.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumChargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_power: Option<Decimal>,

    /// Maximum charge power on phase L2 in W, defined by min(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumChargePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_power_l2: Option<Decimal>,

    /// Maximum charge power on phase L3 in W, defined by min(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumChargePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_power_l3: Option<Decimal>,

    /// Minimum discharge (injected) power in W, defined by max(EV, EVSE) at unity power factor. Value >= 0.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    /// It corresponds to the WMax attribute in the IEC 61850.
    /// It is usually equivalent to the rated apparent power of the EV when discharging (VAMax attribute in the IEC 61850).
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumDischargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_power: Option<Decimal>,

    /// Minimum discharge power on phase L2 in W, defined by max(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumDischargePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_power_l2: Option<Decimal>,

    /// Minimum discharge power on phase L3 in W, defined by max(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMinimumDischargePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_power_l3: Option<Decimal>,

    /// Maximum discharge (injected) power in W, defined by min(EV, EVSE) at unity power factor. Value >= 0.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumDischargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_power: Option<Decimal>,

    /// Maximum discharge power on phase L2 in W, defined by min(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumDischargePowe_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_power_l2: Option<Decimal>,

    /// Maximum discharge power on phase L3 in W, defined by min(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_AC/DC_CPDReqEnergyTransferModeType: EVMaximumDischargePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_power_l3: Option<Decimal>,

    /// Minimum charge current in A, defined by max(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMinimumChargeCurrent
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_current: Option<Decimal>,

    /// Maximum charge current in A, defined by min(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMaximumChargeCurrent
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_current: Option<Decimal>,

    /// Minimum discharge current in A, defined by max(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMinimumDischargeCurrent
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_current: Option<Decimal>,

    /// Maximum discharge current in A, defined by min(EV, EVSE). Value >= 0.
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMaximumDischargeCurrent
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_current: Option<Decimal>,

    /// Minimum voltage in V, defined by max(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMinimumVoltage
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_voltage: Option<Decimal>,

    /// Maximum voltage in V, defined by min(EV, EVSE)
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: EVMaximumVoltage
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_voltage: Option<Decimal>,

    /// Energy to requested state of charge in Wh
    /// Relates to:
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVTargetEnergyRequest
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_target_energy_request: Option<Decimal>,

    /// Energy to minimum allowed state of charge in Wh
    /// Relates to:
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVMinimumEnergyRequest
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_min_energy_request: Option<Decimal>,

    /// Energy to maximum state of charge in Wh
    /// Relates to:
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVMaximumEnergyRequest
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_max_energy_request: Option<Decimal>,

    /// Energy (in Wh) to minimum state of charge for cycling (V2X) activity.
    /// Positive value means that current state of charge is below V2X range.
    /// Relates to:
    /// *ISO 15118-20*: Dynamic_SEReqControlModeType: EVMinimumV2XEnergyRequest
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_min_v2x_energy_request: Option<Decimal>,

    /// Energy (in Wh) to maximum state of charge for cycling (V2X) activity.
    /// Negative value indicates that current state of charge is above V2X range.
    /// Relates to:
    /// *ISO 15118-20*: Dynamic_SEReqControlModeType: EVMaximumV2XEnergyRequest
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_max_v2x_energy_request: Option<Decimal>,

    /// Target state of charge at departure as percentage.
    /// Relates to:
    /// *ISO 15118-20*: BPT_DC_CPDReqEnergyTransferModeType: TargetSOC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub target_so_c: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl V2XChargingParametersType {
    /// Creates a new empty `V2XChargingParametersType` with all fields set to `None`.
    pub fn new() -> Self {
        Self {
            min_charge_power: None,
            min_charge_power_l2: None,
            min_charge_power_l3: None,
            max_charge_power: None,
            max_charge_power_l2: None,
            max_charge_power_l3: None,
            min_discharge_power: None,
            min_discharge_power_l2: None,
            min_discharge_power_l3: None,
            max_discharge_power: None,
            max_discharge_power_l2: None,
            max_discharge_power_l3: None,
            min_charge_current: None,
            max_charge_current: None,
            min_discharge_current: None,
            max_discharge_current: None,
            min_voltage: None,
            max_voltage: None,
            ev_target_energy_request: None,
            ev_min_energy_request: None,
            ev_max_energy_request: None,
            ev_min_v2x_energy_request: None,
            ev_max_v2x_energy_request: None,
            target_so_c: None,
            custom_data: None,
        }
    }

    /// Gets the minimum charge power.
    pub fn min_charge_power(&self) -> Option<Decimal> {
        self.min_charge_power
    }

    /// Sets the minimum charge power.
    pub fn set_min_charge_power(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_charge_power = value;
        self
    }

    /// Gets the minimum charge power on phase L2.
    pub fn min_charge_power_l2(&self) -> Option<Decimal> {
        self.min_charge_power_l2
    }

    /// Sets the minimum charge power on phase L2.
    pub fn set_min_charge_power_l2(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_charge_power_l2 = value;
        self
    }

    /// Gets the minimum charge power on phase L3.
    pub fn min_charge_power_l3(&self) -> Option<Decimal> {
        self.min_charge_power_l3
    }

    /// Sets the minimum charge power on phase L3.
    pub fn set_min_charge_power_l3(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_charge_power_l3 = value;
        self
    }

    /// Gets the maximum charge power.
    pub fn max_charge_power(&self) -> Option<Decimal> {
        self.max_charge_power
    }

    /// Sets the maximum charge power.
    pub fn set_max_charge_power(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_charge_power = value;
        self
    }

    /// Gets the maximum charge power on phase L2.
    pub fn max_charge_power_l2(&self) -> Option<Decimal> {
        self.max_charge_power_l2
    }

    /// Sets the maximum charge power on phase L2.
    pub fn set_max_charge_power_l2(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_charge_power_l2 = value;
        self
    }

    /// Gets the maximum charge power on phase L3.
    pub fn max_charge_power_l3(&self) -> Option<Decimal> {
        self.max_charge_power_l3
    }

    /// Sets the maximum charge power on phase L3.
    pub fn set_max_charge_power_l3(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_charge_power_l3 = value;
        self
    }

    /// Gets the minimum discharge power.
    pub fn min_discharge_power(&self) -> Option<Decimal> {
        self.min_discharge_power
    }

    /// Sets the minimum discharge power.
    pub fn set_min_discharge_power(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_discharge_power = value;
        self
    }

    /// Gets the minimum discharge power on phase L2.
    pub fn min_discharge_power_l2(&self) -> Option<Decimal> {
        self.min_discharge_power_l2
    }

    /// Sets the minimum discharge power on phase L2.
    pub fn set_min_discharge_power_l2(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_discharge_power_l2 = value;
        self
    }

    /// Gets the minimum discharge power on phase L3.
    pub fn min_discharge_power_l3(&self) -> Option<Decimal> {
        self.min_discharge_power_l3
    }

    /// Sets the minimum discharge power on phase L3.
    pub fn set_min_discharge_power_l3(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_discharge_power_l3 = value;
        self
    }

    /// Gets the maximum discharge power.
    pub fn max_discharge_power(&self) -> Option<Decimal> {
        self.max_discharge_power
    }

    /// Sets the maximum discharge power.
    pub fn set_max_discharge_power(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_discharge_power = value;
        self
    }

    /// Gets the maximum discharge power on phase L2.
    pub fn max_discharge_power_l2(&self) -> Option<Decimal> {
        self.max_discharge_power_l2
    }

    /// Sets the maximum discharge power on phase L2.
    pub fn set_max_discharge_power_l2(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_discharge_power_l2 = value;
        self
    }

    /// Gets the maximum discharge power on phase L3.
    pub fn max_discharge_power_l3(&self) -> Option<Decimal> {
        self.max_discharge_power_l3
    }

    /// Sets the maximum discharge power on phase L3.
    pub fn set_max_discharge_power_l3(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_discharge_power_l3 = value;
        self
    }

    /// Gets the minimum charge current.
    pub fn min_charge_current(&self) -> Option<Decimal> {
        self.min_charge_current
    }

    /// Sets the minimum charge current.
    pub fn set_min_charge_current(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_charge_current = value;
        self
    }

    /// Gets the maximum charge current.
    pub fn max_charge_current(&self) -> Option<Decimal> {
        self.max_charge_current
    }

    /// Sets the maximum charge current.
    pub fn set_max_charge_current(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_charge_current = value;
        self
    }

    /// Gets the minimum discharge current.
    pub fn min_discharge_current(&self) -> Option<Decimal> {
        self.min_discharge_current
    }

    /// Sets the minimum discharge current.
    pub fn set_min_discharge_current(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_discharge_current = value;
        self
    }

    /// Gets the maximum discharge current.
    pub fn max_discharge_current(&self) -> Option<Decimal> {
        self.max_discharge_current
    }

    /// Sets the maximum discharge current.
    pub fn set_max_discharge_current(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_discharge_current = value;
        self
    }

    /// Gets the minimum voltage.
    pub fn min_voltage(&self) -> Option<Decimal> {
        self.min_voltage
    }

    /// Sets the minimum voltage.
    pub fn set_min_voltage(&mut self, value: Option<Decimal>) -> &mut Self {
        self.min_voltage = value;
        self
    }

    /// Gets the maximum voltage.
    pub fn max_voltage(&self) -> Option<Decimal> {
        self.max_voltage
    }

    /// Sets the maximum voltage.
    pub fn set_max_voltage(&mut self, value: Option<Decimal>) -> &mut Self {
        self.max_voltage = value;
        self
    }

    /// Gets the target energy request.
    pub fn ev_target_energy_request(&self) -> Option<Decimal> {
        self.ev_target_energy_request
    }

    /// Sets the target energy request.
    pub fn set_ev_target_energy_request(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_target_energy_request = value;
        self
    }

    /// Gets the minimum energy request.
    pub fn ev_min_energy_request(&self) -> Option<Decimal> {
        self.ev_min_energy_request
    }

    /// Sets the minimum energy request.
    pub fn set_ev_min_energy_request(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_min_energy_request = value;
        self
    }

    /// Gets the maximum energy request.
    pub fn ev_max_energy_request(&self) -> Option<Decimal> {
        self.ev_max_energy_request
    }

    /// Sets the maximum energy request.
    pub fn set_ev_max_energy_request(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_max_energy_request = value;
        self
    }

    /// Gets the minimum V2X energy request.
    pub fn ev_min_v2x_energy_request(&self) -> Option<Decimal> {
        self.ev_min_v2x_energy_request
    }

    /// Sets the minimum V2X energy request.
    pub fn set_ev_min_v2x_energy_request(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_min_v2x_energy_request = value;
        self
    }

    /// Gets the maximum V2X energy request.
    pub fn ev_max_v2x_energy_request(&self) -> Option<Decimal> {
        self.ev_max_v2x_energy_request
    }

    /// Sets the maximum V2X energy request.
    pub fn set_ev_max_v2x_energy_request(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_max_v2x_energy_request = value;
        self
    }

    /// Gets the target SoC.
    pub fn target_so_c(&self) -> Option<i32> {
        self.target_so_c
    }

    /// Sets the target SoC.
    pub fn set_target_so_c(&mut self, value: Option<i32>) -> &mut Self {
        self.target_so_c = value;
        self
    }

    /// Gets the custom data.
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    pub fn set_custom_data(&mut self, value: Option<CustomDataType>) -> &mut Self {
        self.custom_data = value;
        self
    }

    // Builder pattern methods

    /// Sets the minimum charge power using the builder pattern.
    pub fn with_min_charge_power(mut self, value: Decimal) -> Self {
        self.min_charge_power = Some(value);
        self
    }

    /// Sets the minimum charge power on phase L2 using the builder pattern.
    pub fn with_min_charge_power_l2(mut self, value: Decimal) -> Self {
        self.min_charge_power_l2 = Some(value);
        self
    }

    /// Sets the minimum charge power on phase L3 using the builder pattern.
    pub fn with_min_charge_power_l3(mut self, value: Decimal) -> Self {
        self.min_charge_power_l3 = Some(value);
        self
    }

    /// Sets the maximum charge power using the builder pattern.
    pub fn with_max_charge_power(mut self, value: Decimal) -> Self {
        self.max_charge_power = Some(value);
        self
    }

    /// Sets the maximum charge power on phase L2 using the builder pattern.
    pub fn with_max_charge_power_l2(mut self, value: Decimal) -> Self {
        self.max_charge_power_l2 = Some(value);
        self
    }

    /// Sets the maximum charge power on phase L3 using the builder pattern.
    pub fn with_max_charge_power_l3(mut self, value: Decimal) -> Self {
        self.max_charge_power_l3 = Some(value);
        self
    }

    /// Sets the minimum discharge power using the builder pattern.
    pub fn with_min_discharge_power(mut self, value: Decimal) -> Self {
        self.min_discharge_power = Some(value);
        self
    }

    /// Sets the minimum discharge power on phase L2 using the builder pattern.
    pub fn with_min_discharge_power_l2(mut self, value: Decimal) -> Self {
        self.min_discharge_power_l2 = Some(value);
        self
    }

    /// Sets the minimum discharge power on phase L3 using the builder pattern.
    pub fn with_min_discharge_power_l3(mut self, value: Decimal) -> Self {
        self.min_discharge_power_l3 = Some(value);
        self
    }

    /// Sets the maximum discharge power using the builder pattern.
    pub fn with_max_discharge_power(mut self, value: Decimal) -> Self {
        self.max_discharge_power = Some(value);
        self
    }

    /// Sets the maximum discharge power on phase L2 using the builder pattern.
    pub fn with_max_discharge_power_l2(mut self, value: Decimal) -> Self {
        self.max_discharge_power_l2 = Some(value);
        self
    }

    /// Sets the maximum discharge power on phase L3 using the builder pattern.
    pub fn with_max_discharge_power_l3(mut self, value: Decimal) -> Self {
        self.max_discharge_power_l3 = Some(value);
        self
    }

    /// Sets the minimum charge current using the builder pattern.
    pub fn with_min_charge_current(mut self, value: Decimal) -> Self {
        self.min_charge_current = Some(value);
        self
    }

    /// Sets the maximum charge current using the builder pattern.
    pub fn with_max_charge_current(mut self, value: Decimal) -> Self {
        self.max_charge_current = Some(value);
        self
    }

    /// Sets the minimum discharge current using the builder pattern.
    pub fn with_min_discharge_current(mut self, value: Decimal) -> Self {
        self.min_discharge_current = Some(value);
        self
    }

    /// Sets the maximum discharge current using the builder pattern.
    pub fn with_max_discharge_current(mut self, value: Decimal) -> Self {
        self.max_discharge_current = Some(value);
        self
    }

    /// Sets the minimum voltage using the builder pattern.
    pub fn with_min_voltage(mut self, value: Decimal) -> Self {
        self.min_voltage = Some(value);
        self
    }

    /// Sets the maximum voltage using the builder pattern.
    pub fn with_max_voltage(mut self, value: Decimal) -> Self {
        self.max_voltage = Some(value);
        self
    }

    /// Sets the target energy request using the builder pattern.
    pub fn with_ev_target_energy_request(mut self, value: Decimal) -> Self {
        self.ev_target_energy_request = Some(value);
        self
    }

    /// Sets the minimum energy request using the builder pattern.
    pub fn with_ev_min_energy_request(mut self, value: Decimal) -> Self {
        self.ev_min_energy_request = Some(value);
        self
    }

    /// Sets the maximum energy request using the builder pattern.
    pub fn with_ev_max_energy_request(mut self, value: Decimal) -> Self {
        self.ev_max_energy_request = Some(value);
        self
    }

    /// Sets the minimum V2X energy request using the builder pattern.
    pub fn with_ev_min_v2x_energy_request(mut self, value: Decimal) -> Self {
        self.ev_min_v2x_energy_request = Some(value);
        self
    }

    /// Sets the maximum V2X energy request using the builder pattern.
    pub fn with_ev_max_v2x_energy_request(mut self, value: Decimal) -> Self {
        self.ev_max_v2x_energy_request = Some(value);
        self
    }

    /// Sets the target SoC using the builder pattern.
    pub fn with_target_so_c(mut self, value: i32) -> Self {
        self.target_so_c = Some(value);
        self
    }

    /// Sets the custom data using the builder pattern.
    pub fn with_custom_data(mut self, value: CustomDataType) -> Self {
        self.custom_data = Some(value);
        self
    }
}

impl Default for V2XChargingParametersType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;
    #[test]
    fn test_new() {
        let params = V2XChargingParametersType::new();

        assert_eq!(params.min_charge_power(), None);
        assert_eq!(params.max_discharge_power(), None);
        assert_eq!(params.target_so_c(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let min_charge_power = Decimal::from_f64(1000.0).unwrap();
        let max_charge_power = Decimal::from_f64(10000.0).unwrap();
        let min_charge_current = Decimal::from_f64(10.0).unwrap();
        let max_charge_current = Decimal::from_f64(32.0).unwrap();
        let min_voltage = Decimal::from_f64(200.0).unwrap();
        let max_voltage = Decimal::from_f64(400.0).unwrap();
        let target_so_c = 80;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = V2XChargingParametersType::new()
            .with_min_charge_power(min_charge_power)
            .with_max_charge_power(max_charge_power)
            .with_min_charge_current(min_charge_current)
            .with_max_charge_current(max_charge_current)
            .with_min_voltage(min_voltage)
            .with_max_voltage(max_voltage)
            .with_target_so_c(target_so_c)
            .with_custom_data(custom_data.clone());

        assert_eq!(params.min_charge_power(), Some(min_charge_power));
        assert_eq!(params.max_charge_power(), Some(max_charge_power));
        assert_eq!(params.min_charge_current(), Some(min_charge_current));
        assert_eq!(params.max_charge_current(), Some(max_charge_current));
        assert_eq!(params.min_voltage(), Some(min_voltage));
        assert_eq!(params.max_voltage(), Some(max_voltage));
        assert_eq!(params.target_so_c(), Some(target_so_c));
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let min_charge_power = Decimal::from_f64(1000.0).unwrap();
        let max_charge_power = Decimal::from_f64(10000.0).unwrap();
        let target_so_c = 80;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = V2XChargingParametersType::new();

        params
            .set_min_charge_power(Some(min_charge_power))
            .set_max_charge_power(Some(max_charge_power))
            .set_target_so_c(Some(target_so_c))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.min_charge_power(), Some(min_charge_power));
        assert_eq!(params.max_charge_power(), Some(max_charge_power));
        assert_eq!(params.target_so_c(), Some(target_so_c));
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params
            .set_min_charge_power(None)
            .set_max_charge_power(None)
            .set_target_so_c(None)
            .set_custom_data(None);

        assert_eq!(params.min_charge_power(), None);
        assert_eq!(params.max_charge_power(), None);
        assert_eq!(params.target_so_c(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Test valid target_so_c
        let valid_params = V2XChargingParametersType::new().with_target_so_c(80);
        assert!(valid_params.validate().is_ok());

        // Test invalid target_so_c (over 100)
        let invalid_params = V2XChargingParametersType {
            min_charge_power: None,
            min_charge_power_l2: None,
            min_charge_power_l3: None,
            max_charge_power: None,
            max_charge_power_l2: None,
            max_charge_power_l3: None,
            min_discharge_power: None,
            min_discharge_power_l2: None,
            min_discharge_power_l3: None,
            max_discharge_power: None,
            max_discharge_power_l2: None,
            max_discharge_power_l3: None,
            min_charge_current: None,
            max_charge_current: None,
            min_discharge_current: None,
            max_discharge_current: None,
            min_voltage: None,
            max_voltage: None,
            ev_target_energy_request: None,
            ev_min_energy_request: None,
            ev_max_energy_request: None,
            ev_min_v2x_energy_request: None,
            ev_max_v2x_energy_request: None,
            target_so_c: Some(101),
            custom_data: None,
        };
        assert!(invalid_params.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let params = V2XChargingParametersType::new()
            .with_min_charge_power(Decimal::from_f64(1000.0).unwrap())
            .with_max_charge_power(Decimal::from_f64(10000.0).unwrap())
            .with_target_so_c(80);

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: V2XChargingParametersType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, params);
    }
}
