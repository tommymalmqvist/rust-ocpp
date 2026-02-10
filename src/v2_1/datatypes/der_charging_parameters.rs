use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::der_control::DERControlEnumType;
use crate::v2_1::enumerations::islanding_detection::IslandingDetectionEnumType;

/// DERChargingParametersType is used in ChargingNeedsType during an ISO 15118-20 session for AC_BPT_DER
/// to report the inverter settings related to DER control that were agreed between EVSE and EV.
///
/// Fields starting with "ev" contain values from the EV.
/// Other fields contain a value that is supported by both EV and EVSE.
///
/// DERChargingParametersType type is only relevant in case of an ISO 15118-20 AC_BPT_DER/AC_DER charging session.
///
/// NOTE: All these fields have values greater or equal to zero (i.e. are non-negative)
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERChargingParametersType {
    /// DER control functions supported by EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType:DERControlFunctions (bitmap)
    #[validate(length(min = 1))]
    pub ev_supported_der_control: Vec<DERControlEnumType>,

    /// Rated maximum injected active power by EV, at specified over-excited power factor (overExcitedPowerFactor).
    /// It can also be defined as the rated maximum discharge power at the rated minimum injected reactive power value.
    /// This means that if the EV is providing reactive power support, and it is requested to discharge at max power (e.g. to satisfy an EMS request),
    /// the EV may override the request and discharge up to overExcitedMaximumDischargePower to meet the minimum reactive power requirements.
    /// Corresponds to the WOvPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVOverExcitedMaximumDischargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_over_excited_max_discharge_power: Option<Decimal>,

    /// EV power factor when injecting (over excited) the minimum reactive power.
    /// Corresponds to the OvPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVOverExcitedPowerFactor
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_over_excited_power_factor: Option<Decimal>,

    /// Rated maximum injected active power by EV supported at specified under-excited power factor (EVUnderExcitedPowerFactor).
    /// It can also be defined as the rated maximum dischargePower at the rated minimum absorbed reactive power value.
    /// This means that if the EV is providing reactive power support, and it is requested to discharge at max power (e.g. to satisfy an EMS request),
    /// the EV may override the request and discharge up to underExcitedMaximumDischargePower to meet the minimum reactive power requirements.
    /// This corresponds to the WUnPF attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVUnderExcitedMaximumDischargePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_under_excited_max_discharge_power: Option<Decimal>,

    /// EV power factor when injecting (under excited) the minimum reactive power.
    /// Corresponds to the OvPF attribute in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVUnderExcitedPowerFactor
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_under_excited_power_factor: Option<Decimal>,

    /// Rated maximum total apparent power, defined by min(EV, EVSE) in va.
    /// Corresponds to the VAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumApparentPower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_apparent_power: Option<Decimal>,

    /// Rated maximum absorbed apparent power, defined by min(EV, EVSE) in va.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the ChaVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_apparent_power: Option<Decimal>,

    /// Rated maximum absorbed apparent power on phase L2, defined by min(EV, EVSE) in va.
    /// Corresponds to the ChaVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_apparent_power_l2: Option<Decimal>,

    /// Rated maximum absorbed apparent power on phase L3, defined by min(EV, EVSE) in va.
    /// Corresponds to the ChaVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeApparentPower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_apparent_power_l3: Option<Decimal>,

    /// Rated maximum injected apparent power, defined by min(EV, EVSE) in va.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_apparent_power: Option<Decimal>,

    /// Rated maximum injected apparent power on phase L2, defined by min(EV, EVSE) in va.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_apparent_power_l2: Option<Decimal>,

    /// Rated maximum injected apparent power on phase L3, defined by min(EV, EVSE) in va.
    /// Corresponds to the DisVAMaxRtg in IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeApparentPower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_apparent_power_l3: Option<Decimal>,

    /// Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_reactive_power: Option<Decimal>,

    /// Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars on phase L2.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_reactive_power_l2: Option<Decimal>,

    /// Rated maximum absorbed reactive power, defined by min(EV, EVSE), in vars on phase L3.
    /// Corresponds to the AvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumChargeReactivePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_charge_reactive_power_l3: Option<Decimal>,

    /// Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_reactive_power: Option<Decimal>,

    /// Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars on phase L2.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_reactive_power_l2: Option<Decimal>,

    /// Rated minimum absorbed reactive power, defined by max(EV, EVSE), in vars on phase L3.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumChargeReactivePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charge_reactive_power_l3: Option<Decimal>,

    /// Rated maximum injected reactive power, defined by min(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_reactive_power: Option<Decimal>,

    /// Rated maximum injected reactive power, defined by min(EV, EVSE), in vars on phase L2.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_reactive_power_l2: Option<Decimal>,

    /// Rated maximum injected reactive power, defined by min(EV, EVSE), in vars on phase L3.
    /// Corresponds to the IvarMax attribute in the IEC 61850.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumDischargeReactivePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_discharge_reactive_power_l3: Option<Decimal>,

    /// Rated minimum injected reactive power, defined by max(EV, EVSE), in vars.
    /// This field represents the sum of all phases, unless values are provided for L2 and L3,
    /// in which case this field represents phase L1.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_reactive_power: Option<Decimal>,

    /// Rated minimum injected reactive power, defined by max(EV, EVSE), in var on phase L2.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower_L2
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_reactive_power_l2: Option<Decimal>,

    /// Rated minimum injected reactive power, defined by max(EV, EVSE), in var on phase L3.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumDischargeReactivePower_L3
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_discharge_reactive_power_l3: Option<Decimal>,

    /// Line voltage supported by EVSE and EV.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVNominalVoltage
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub nominal_voltage: Option<Decimal>,

    /// The nominal AC voltage (rms) offset between the Charging Station's electrical connection point and the utility's point of common coupling.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVNominalVoltageOffset
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub nominal_voltage_offset: Option<Decimal>,

    /// Maximum AC rms voltage, as defined by min(EV, EVSE) to operate with.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumNominalVoltage
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_nominal_voltage: Option<Decimal>,

    /// Minimum AC rms voltage, as defined by max(EV, EVSE) to operate with.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMinimumNominalVoltage
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_nominal_voltage: Option<Decimal>,

    /// Manufacturer of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterManufacturer
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub ev_inverter_manufacturer: Option<String>,

    /// Model name of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterModel
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub ev_inverter_model: Option<String>,

    /// Serial number of the EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterSerialNumber
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub ev_inverter_serial_number: Option<String>,

    /// Software version of EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterSwVersion
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub ev_inverter_sw_version: Option<String>,

    /// Hardware version of EV inverter.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVInverterHwVersion
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub ev_inverter_hw_version: Option<String>,

    /// Type of islanding detection method. Only mandatory when islanding detection is required at the site,
    /// as set in the ISO 15118 Service Details configuration.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVIslandingDetectionMethod
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub ev_islanding_detection_method: Option<Vec<IslandingDetectionEnumType>>,

    /// Time after which EV will trip if an island has been detected.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVIslandingTripTime
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_islanding_trip_time: Option<Decimal>,

    /// Maximum injected DC current allowed at level 1 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumLevel1DCInjection
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_maximum_level1_dc_injection: Option<Decimal>,

    /// Maximum allowed duration of DC injection at level 1 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVDurationLevel1DCInjection
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_duration_level1_dc_injection: Option<Decimal>,

    /// Maximum injected DC current allowed at level 2 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVMaximumLevel2DCInjection
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_maximum_level2_dc_injection: Option<Decimal>,

    /// Maximum allowed duration of DC injection at level 2 charging.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVDurationLevel2DCInjection
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_duration_level2_dc_injection: Option<Decimal>,

    /// Measure of the susceptibility of the circuit to reactance, in Siemens (S).
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVReactiveSusceptance
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_reactive_susceptance: Option<Decimal>,

    /// Total energy value, in Wh, that EV is allowed to provide during the entire V2G session.
    /// The value is independent of the V2X Cycling area. Once this value reaches the value of 0,
    /// the EV may block any attempt to discharge in order to protect the battery health.
    /// ISO 15118-20: DER_BPT_AC_CPDReqEnergyTransferModeType: EVSessionTotalDischargeEnergyAvailable
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ev_session_total_discharge_energy_available: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl DERChargingParametersType {
    /// Creates a new `DERChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_supported_der_control` - DER control functions supported by EV
    ///
    /// # Returns
    ///
    /// A new instance of `DERChargingParametersType` with optional fields set to `None`
    pub fn new(ev_supported_der_control: Vec<DERControlEnumType>) -> Self {
        Self {
            ev_supported_der_control,
            ev_over_excited_max_discharge_power: None,
            ev_over_excited_power_factor: None,
            ev_under_excited_max_discharge_power: None,
            ev_under_excited_power_factor: None,
            max_apparent_power: None,
            max_charge_apparent_power: None,
            max_charge_apparent_power_l2: None,
            max_charge_apparent_power_l3: None,
            max_discharge_apparent_power: None,
            max_discharge_apparent_power_l2: None,
            max_discharge_apparent_power_l3: None,
            max_charge_reactive_power: None,
            max_charge_reactive_power_l2: None,
            max_charge_reactive_power_l3: None,
            min_charge_reactive_power: None,
            min_charge_reactive_power_l2: None,
            min_charge_reactive_power_l3: None,
            max_discharge_reactive_power: None,
            max_discharge_reactive_power_l2: None,
            max_discharge_reactive_power_l3: None,
            min_discharge_reactive_power: None,
            min_discharge_reactive_power_l2: None,
            min_discharge_reactive_power_l3: None,
            nominal_voltage: None,
            nominal_voltage_offset: None,
            max_nominal_voltage: None,
            min_nominal_voltage: None,
            ev_inverter_manufacturer: None,
            ev_inverter_model: None,
            ev_inverter_serial_number: None,
            ev_inverter_sw_version: None,
            ev_inverter_hw_version: None,
            ev_islanding_detection_method: None,
            ev_islanding_trip_time: None,
            ev_maximum_level1_dc_injection: None,
            ev_duration_level1_dc_injection: None,
            ev_maximum_level2_dc_injection: None,
            ev_duration_level2_dc_injection: None,
            ev_reactive_susceptance: None,
            ev_session_total_discharge_energy_available: None,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the DER control functions supported by EV.
    ///
    /// # Returns
    ///
    /// A reference to the vector of DER control functions
    pub fn ev_supported_der_control(&self) -> &Vec<DERControlEnumType> {
        &self.ev_supported_der_control
    }

    /// Sets the DER control functions supported by EV.
    ///
    /// # Arguments
    ///
    /// * `ev_supported_der_control` - DER control functions supported by EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_supported_der_control(
        &mut self,
        ev_supported_der_control: Vec<DERControlEnumType>,
    ) -> &mut Self {
        self.ev_supported_der_control = ev_supported_der_control;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the rated maximum injected active power by EV at specified over-excited power factor.
    ///
    /// # Returns
    ///
    /// An optional reference to the value
    pub fn ev_over_excited_max_discharge_power(&self) -> Option<&Decimal> {
        self.ev_over_excited_max_discharge_power.as_ref()
    }

    /// Sets the rated maximum injected active power by EV at specified over-excited power factor.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_over_excited_max_discharge_power(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_over_excited_max_discharge_power = value;
        self
    }

    /// Sets the rated maximum injected active power by EV at specified over-excited power factor.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ev_over_excited_max_discharge_power(mut self, value: Decimal) -> Self {
        self.ev_over_excited_max_discharge_power = Some(value);
        self
    }

    /// Gets the EV power factor when injecting (over excited) the minimum reactive power.
    ///
    /// # Returns
    ///
    /// An optional reference to the value
    pub fn ev_over_excited_power_factor(&self) -> Option<&Decimal> {
        self.ev_over_excited_power_factor.as_ref()
    }

    /// Sets the EV power factor when injecting (over excited) the minimum reactive power.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_over_excited_power_factor(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_over_excited_power_factor = value;
        self
    }

    /// Sets the EV power factor when injecting (over excited) the minimum reactive power.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ev_over_excited_power_factor(mut self, value: Decimal) -> Self {
        self.ev_over_excited_power_factor = Some(value);
        self
    }

    /// Gets the rated maximum injected active power by EV at specified under-excited power factor.
    ///
    /// # Returns
    ///
    /// An optional reference to the value
    pub fn ev_under_excited_max_discharge_power(&self) -> Option<&Decimal> {
        self.ev_under_excited_max_discharge_power.as_ref()
    }

    /// Sets the rated maximum injected active power by EV at specified under-excited power factor.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_under_excited_max_discharge_power(
        &mut self,
        value: Option<Decimal>,
    ) -> &mut Self {
        self.ev_under_excited_max_discharge_power = value;
        self
    }

    /// Sets the rated maximum injected active power by EV at specified under-excited power factor.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ev_under_excited_max_discharge_power(mut self, value: Decimal) -> Self {
        self.ev_under_excited_max_discharge_power = Some(value);
        self
    }

    /// Gets the EV power factor when injecting (under excited) the minimum reactive power.
    ///
    /// # Returns
    ///
    /// An optional reference to the value
    pub fn ev_under_excited_power_factor(&self) -> Option<&Decimal> {
        self.ev_under_excited_power_factor.as_ref()
    }

    /// Sets the EV power factor when injecting (under excited) the minimum reactive power.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_under_excited_power_factor(&mut self, value: Option<Decimal>) -> &mut Self {
        self.ev_under_excited_power_factor = value;
        self
    }

    /// Sets the EV power factor when injecting (under excited) the minimum reactive power.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ev_under_excited_power_factor(mut self, value: Decimal) -> Self {
        self.ev_under_excited_power_factor = Some(value);
        self
    }

    /// Gets the type of islanding detection method.
    ///
    /// # Returns
    ///
    /// An optional reference to the vector of islanding detection methods
    pub fn ev_islanding_detection_method(&self) -> Option<&Vec<IslandingDetectionEnumType>> {
        self.ev_islanding_detection_method.as_ref()
    }

    /// Sets the type of islanding detection method.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_islanding_detection_method(
        &mut self,
        value: Option<Vec<IslandingDetectionEnumType>>,
    ) -> &mut Self {
        self.ev_islanding_detection_method = value;
        self
    }

    /// Sets the type of islanding detection method.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ev_islanding_detection_method(
        mut self,
        value: Vec<IslandingDetectionEnumType>,
    ) -> Self {
        self.ev_islanding_detection_method = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_der_charging_parameters() {
        let der_controls = vec![
            DERControlEnumType::FreqDroop,
            DERControlEnumType::PowerFactor,
        ];
        let params = DERChargingParametersType::new(der_controls.clone());

        assert_eq!(params.ev_supported_der_control(), &der_controls);
        assert_eq!(params.custom_data(), None);
        assert_eq!(params.ev_over_excited_max_discharge_power(), None);
        assert_eq!(params.ev_over_excited_power_factor(), None);
        assert_eq!(params.ev_under_excited_max_discharge_power(), None);
        assert_eq!(params.ev_under_excited_power_factor(), None);
        assert_eq!(params.ev_islanding_detection_method(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let der_controls = vec![DERControlEnumType::FreqDroop];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = DERChargingParametersType::new(der_controls.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(params.ev_supported_der_control(), &der_controls);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_with_methods() {
        let der_controls = vec![DERControlEnumType::FreqDroop];
        let islanding_methods = vec![
            IslandingDetectionEnumType::RoCoF,
            IslandingDetectionEnumType::UvpOvp,
        ];

        let params = DERChargingParametersType::new(der_controls.clone())
            .with_ev_over_excited_max_discharge_power(dec!(100.5))
            .with_ev_over_excited_power_factor(dec!(0.95))
            .with_ev_under_excited_max_discharge_power(dec!(90.0))
            .with_ev_under_excited_power_factor(dec!(0.9))
            .with_ev_islanding_detection_method(islanding_methods.clone());

        assert_eq!(params.ev_supported_der_control(), &der_controls);
        assert_eq!(
            params.ev_over_excited_max_discharge_power(),
            Some(&dec!(100.5))
        );
        assert_eq!(params.ev_over_excited_power_factor(), Some(&dec!(0.95)));
        assert_eq!(
            params.ev_under_excited_max_discharge_power(),
            Some(&dec!(90.0))
        );
        assert_eq!(params.ev_under_excited_power_factor(), Some(&dec!(0.9)));
        assert_eq!(
            params.ev_islanding_detection_method(),
            Some(&islanding_methods)
        );
    }

    #[test]
    fn test_setter_methods() {
        let der_controls1 = vec![DERControlEnumType::FreqDroop];
        let der_controls2 = vec![
            DERControlEnumType::PowerFactor,
            DERControlEnumType::FixedVar,
        ];
        let custom_data = CustomDataType::new("VendorX".to_string());
        let islanding_methods = vec![IslandingDetectionEnumType::RoCoF];

        let mut params = DERChargingParametersType::new(der_controls1.clone());

        params
            .set_ev_supported_der_control(der_controls2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_ev_over_excited_max_discharge_power(Some(dec!(100.5)))
            .set_ev_over_excited_power_factor(Some(dec!(0.95)))
            .set_ev_under_excited_max_discharge_power(Some(dec!(90.0)))
            .set_ev_under_excited_power_factor(Some(dec!(0.9)))
            .set_ev_islanding_detection_method(Some(islanding_methods.clone()));

        assert_eq!(params.ev_supported_der_control(), &der_controls2);
        assert_eq!(params.custom_data(), Some(&custom_data));
        assert_eq!(
            params.ev_over_excited_max_discharge_power(),
            Some(&dec!(100.5))
        );
        assert_eq!(params.ev_over_excited_power_factor(), Some(&dec!(0.95)));
        assert_eq!(
            params.ev_under_excited_max_discharge_power(),
            Some(&dec!(90.0))
        );
        assert_eq!(params.ev_under_excited_power_factor(), Some(&dec!(0.9)));
        assert_eq!(
            params.ev_islanding_detection_method(),
            Some(&islanding_methods)
        );

        // Test clearing optional fields
        params
            .set_custom_data(None)
            .set_ev_over_excited_max_discharge_power(None)
            .set_ev_over_excited_power_factor(None)
            .set_ev_under_excited_max_discharge_power(None)
            .set_ev_under_excited_power_factor(None)
            .set_ev_islanding_detection_method(None);

        assert_eq!(params.custom_data(), None);
        assert_eq!(params.ev_over_excited_max_discharge_power(), None);
        assert_eq!(params.ev_over_excited_power_factor(), None);
        assert_eq!(params.ev_under_excited_max_discharge_power(), None);
        assert_eq!(params.ev_under_excited_power_factor(), None);
        assert_eq!(params.ev_islanding_detection_method(), None);
    }
}
