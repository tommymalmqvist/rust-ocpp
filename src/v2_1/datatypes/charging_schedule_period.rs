use crate::v2_1::datatypes::CustomDataType;
use crate::v2_1::{
    datatypes::{V2XFreqWattPointType, V2XSignalWattPointType},
    enumerations::OperationModeEnumType,
    helpers::validator::validate_discharge_limit,
};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// Charging schedule period structure defines a time period in a charging schedule.
/// It is used in: CompositeScheduleType and in ChargingScheduleType.
/// When used in a NotifyEVChargingScheduleRequest only startPeriod, limit, limit_L2, limit_L3 are relevant.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriodType {
    /// Start of the period, in seconds from the start of schedule.
    /// The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,

    /// Optional only when not required by the operationMode, as in CentralSetpoint, ExternalSetpoint,
    /// ExternalLimits, LocalFrequency, LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable chargingRateUnit.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility
    /// with older systems that use a negative value to specify a discharging limit.
    /// When using chargingRateUnit = 'W', this field represents the sum of the power of all phases,
    /// unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub limit: Decimal,

    /// Charging rate limit on phase L2 in the applicable chargingRateUnit.
    #[serde(rename = "limit_L2")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub limit_l2: Option<Decimal>,

    /// Charging rate limit on phase L3 in the applicable chargingRateUnit.
    #[serde(rename = "limit_L3")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub limit_l3: Option<Decimal>,

    /// The number of phases that can be used for charging.
    /// If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 3))]
    pub number_phases: Option<i32>,

    /// Values: 1..3, Used if numberPhases=1 and if the EVSE is capable of switching the phase connected to the EV,
    /// i.e. ACPhaseSwitchingSupported is defined and true. It's not allowed unless both conditions above are true.
    /// If both conditions are true, and phaseToUse is omitted, the Charging Station / EVSE will make the selection on its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 1, max = 3))]
    pub phase_to_use: Option<i32>,

    /// Limit in _chargingRateUnit_ that the EV is allowed to discharge with. Note, these are negative values in order to be consistent with _setpoint_, which can be positive and negative.  +\r\nFor AC this field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit: Option<Decimal>,

    /// Limit in _chargingRateUnit_ that the EV is allowed to discharge with on phase L2.
    #[serde(rename = "dischargeLimit_L2")]
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit_l2: Option<Decimal>,

    /// Limit in _chargingRateUnit_ that the EV is allowed to discharge with on phase L3.
    #[serde(rename = "dischargeLimit_L3")]
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit_l3: Option<Decimal>,

    /// Setpoint in _chargingRateUnit_ that the EV should follow as close as possible. Use negative values for discharging. +\r\nWhen a limit and/or _dischargeLimit_ are given the overshoot when following _setpoint_ must remain within these values.\r\nThis field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint: Option<Decimal>,

    /// Setpoint in _chargingRateUnit_ that the EV should follow on phase L2 as close as possible.
    #[serde(rename = "setpoint_L2")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_l2: Option<Decimal>,

    /// Setpoint in _chargingRateUnit_ that the EV should follow on phase L3 as close as possible.
    #[serde(rename = "setpoint_L3")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_l3: Option<Decimal>,

    /// Setpoint for reactive power (or current) in _chargingRateUnit_ that the EV should follow as closely as possible. Positive values for inductive, negative for capacitive reactive power or current.  +\r\nThis field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive: Option<Decimal>,

    /// Setpoint for reactive power (or current) in _chargingRateUnit_ that the EV should follow on phase L2 as closely as possible.
    #[serde(rename = "setpointReactive_L2")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive_l2: Option<Decimal>,

    /// Setpoint for reactive power (or current) in _chargingRateUnit_ that the EV should follow on phase L3 as closely as possible.
    #[serde(rename = "setpointReactive_L3")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive_l3: Option<Decimal>,

    /// If  true, the EV should attempt to keep the BMS preconditioned for this time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preconditioning_request: Option<bool>,

    /// If true, the EVSE must turn off power electronics/modules associated with this transaction. Default value when absent is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_sleep: Option<bool>,

    /// Power value that, when present, is used as a baseline on top of which values from _v2xFreqWattCurve_ and _v2xSignalWattCurve_ are added.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub v2x_baseline: Option<Decimal>,

    /// Charging operation mode to use during this time interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_mode: Option<OperationModeEnumType>,

    /// Frequency-watt curve for V2X operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 20), nested)]
    pub v2x_freq_watt_curve: Option<Vec<V2XFreqWattPointType>>,

    /// Signal-watt curve for V2X operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 20), nested)]
    pub v2x_signal_watt_curve: Option<Vec<V2XSignalWattPointType>>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingSchedulePeriodType {
    /// Creates a new `ChargingSchedulePeriodType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start of the period, in seconds from the start of schedule
    /// * `limit` - Charging rate limit during the schedule period
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingSchedulePeriodType` with optional fields set to `None`
    pub fn new(start_period: i32, limit: Decimal) -> Self {
        Self {
            start_period,
            limit,
            limit_l2: None,
            limit_l3: None,
            number_phases: None,
            phase_to_use: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
            preconditioning_request: None,
            evse_sleep: None,
            v2x_baseline: None,
            operation_mode: None,
            v2x_freq_watt_curve: None,
            v2x_signal_watt_curve: None,
            custom_data: None,
        }
    }

    /// Creates a new `ChargingSchedulePeriodType` with required fields from f64 values.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start of the period, in seconds from the start of schedule
    /// * `limit` - Charging rate limit during the schedule period as f64
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingSchedulePeriodType` with optional fields set to `None`
    pub fn new_from_f64(start_period: i32, limit: f64) -> Self {
        Self::new(
            start_period,
            Decimal::from_f64(limit).unwrap_or(Decimal::ZERO),
        )
    }

    /// Sets the limit for the second phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit for the second phase
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l2(mut self, limit_l2: Decimal) -> Self {
        self.limit_l2 = Some(limit_l2);
        self
    }

    /// Sets the limit for the second phase from an f64 value.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit for the second phase as f64
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l2_f64(self, limit_l2: f64) -> Self {
        self.with_limit_l2(Decimal::from_f64(limit_l2).unwrap_or(Decimal::ZERO))
    }

    /// Sets the limit for the third phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit for the third phase
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l3(mut self, limit_l3: Decimal) -> Self {
        self.limit_l3 = Some(limit_l3);
        self
    }

    /// Sets the limit for the third phase from an f64 value.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit for the third phase as f64
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l3_f64(self, limit_l3: f64) -> Self {
        self.with_limit_l3(Decimal::from_f64(limit_l3).unwrap_or(Decimal::ZERO))
    }

    /// Sets the discharge limit.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit` - Limit that the EV is allowed to discharge with (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit(mut self, discharge_limit: Decimal) -> Self {
        self.discharge_limit = Some(discharge_limit);
        self
    }

    /// Sets the discharge limit from an f64 value.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit` - Limit that the EV is allowed to discharge with as f64 (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_f64(self, discharge_limit: f64) -> Self {
        self.with_discharge_limit(Decimal::from_f64(discharge_limit).unwrap_or(Decimal::ZERO))
    }

    /// Sets the discharge limit for phase L2.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l2` - Limit that the EV is allowed to discharge with on phase L2 (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l2(mut self, discharge_limit_l2: Decimal) -> Self {
        self.discharge_limit_l2 = Some(discharge_limit_l2);
        self
    }

    /// Sets the discharge limit for phase L2 from an f64 value.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l2` - Limit that the EV is allowed to discharge with on phase L2 as f64 (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l2_f64(self, discharge_limit_l2: f64) -> Self {
        self.with_discharge_limit_l2(Decimal::from_f64(discharge_limit_l2).unwrap_or(Decimal::ZERO))
    }

    /// Sets the discharge limit for phase L3.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l3` - Limit that the EV is allowed to discharge with on phase L3 (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l3(mut self, discharge_limit_l3: Decimal) -> Self {
        self.discharge_limit_l3 = Some(discharge_limit_l3);
        self
    }

    /// Sets the discharge limit for phase L3 from an f64 value.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l3` - Limit that the EV is allowed to discharge with on phase L3 as f64 (must be negative)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l3_f64(self, discharge_limit_l3: f64) -> Self {
        self.with_discharge_limit_l3(Decimal::from_f64(discharge_limit_l3).unwrap_or(Decimal::ZERO))
    }

    /// Sets the number of phases.
    ///
    /// # Arguments
    ///
    /// * `number_phases` - The number of phases that can be used for charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_number_phases(mut self, number_phases: i32) -> Self {
        self.number_phases = Some(number_phases);
        self
    }

    /// Sets the phase to use.
    ///
    /// # Arguments
    ///
    /// * `phase_to_use` - The phase to use (values: 1..3)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_phase_to_use(mut self, phase_to_use: i32) -> Self {
        self.phase_to_use = Some(phase_to_use);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start period.
    ///
    /// # Returns
    ///
    /// The start of the period, in seconds from the start of schedule
    pub fn start_period(&self) -> i32 {
        self.start_period
    }

    /// Sets the start period.
    ///
    /// # Arguments
    ///
    /// * `start_period` - Start of the period, in seconds from the start of schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_period(&mut self, start_period: i32) -> &mut Self {
        self.start_period = start_period;
        self
    }

    /// Gets the charging rate limit.
    ///
    /// # Returns
    ///
    /// The charging rate limit during the schedule period
    pub fn limit(&self) -> &Decimal {
        &self.limit
    }

    /// Sets the charging rate limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Charging rate limit during the schedule period
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit(&mut self, limit: Decimal) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Gets the limit for the second phase.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit for the second phase
    pub fn limit_l2(&self) -> Option<&Decimal> {
        self.limit_l2.as_ref()
    }

    /// Sets the limit for the second phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit for the second phase, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l2(&mut self, limit_l2: Option<Decimal>) -> &mut Self {
        self.limit_l2 = limit_l2;
        self
    }

    /// Gets the limit for the third phase.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit for the third phase
    pub fn limit_l3(&self) -> Option<&Decimal> {
        self.limit_l3.as_ref()
    }

    /// Sets the limit for the third phase.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit for the third phase, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l3(&mut self, limit_l3: Option<Decimal>) -> &mut Self {
        self.limit_l3 = limit_l3;
        self
    }

    /// Gets the discharge limit.
    ///
    /// # Returns
    ///
    /// An optional discharge limit
    pub fn discharge_limit(&self) -> Option<&Decimal> {
        self.discharge_limit.as_ref()
    }

    /// Sets the discharge limit.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit` - Discharge limit (must be negative), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit(&mut self, discharge_limit: Option<Decimal>) -> &mut Self {
        self.discharge_limit = discharge_limit;
        self
    }

    /// Gets the discharge limit for phase L2.
    ///
    /// # Returns
    ///
    /// An optional discharge limit for phase L2
    pub fn discharge_limit_l2(&self) -> Option<&Decimal> {
        self.discharge_limit_l2.as_ref()
    }

    /// Sets the discharge limit for phase L2.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l2` - Discharge limit for phase L2 (must be negative), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit_l2(&mut self, discharge_limit_l2: Option<Decimal>) -> &mut Self {
        self.discharge_limit_l2 = discharge_limit_l2;
        self
    }

    /// Gets the discharge limit for phase L3.
    ///
    /// # Returns
    ///
    /// An optional discharge limit for phase L3
    pub fn discharge_limit_l3(&self) -> Option<&Decimal> {
        self.discharge_limit_l3.as_ref()
    }

    /// Sets the discharge limit for phase L3.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l3` - Discharge limit for phase L3 (must be negative), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit_l3(&mut self, discharge_limit_l3: Option<Decimal>) -> &mut Self {
        self.discharge_limit_l3 = discharge_limit_l3;
        self
    }

    /// Gets the number of phases.
    ///
    /// # Returns
    ///
    /// An optional number of phases that can be used for charging
    pub fn number_phases(&self) -> Option<i32> {
        self.number_phases
    }

    /// Sets the number of phases.
    ///
    /// # Arguments
    ///
    /// * `number_phases` - The number of phases that can be used for charging, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_number_phases(&mut self, number_phases: Option<i32>) -> &mut Self {
        self.number_phases = number_phases;
        self
    }

    /// Gets the phase to use.
    ///
    /// # Returns
    ///
    /// An optional phase to use
    pub fn phase_to_use(&self) -> Option<i32> {
        self.phase_to_use
    }

    /// Sets the phase to use.
    ///
    /// # Arguments
    ///
    /// * `phase_to_use` - The phase to use (values: 1..3), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_phase_to_use(&mut self, phase_to_use: Option<i32>) -> &mut Self {
        self.phase_to_use = phase_to_use;
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
    /// * `custom_data` - Custom data for this charging schedule period, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Validates this instance according to the OCPP 2.1 specification.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the instance is valid, otherwise an error
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        // Validate number_phases (range 0-3)
        if let Some(phases) = self.number_phases {
            if phases < 0 || phases > 3 {
                let mut error = ValidationError::new("number_phases_range");
                error.message = Some("Number of phases must be between 0 and 3".into());
                errors.add("number_phases", error);
            }
        }

        // Validate phase_to_use (range 1-3)
        if let Some(phase) = self.phase_to_use {
            if phase < 1 || phase > 3 {
                let mut error = ValidationError::new("phase_to_use_range");
                error.message = Some("Phase to use must be between 1 and 3".into());
                errors.add("phase_to_use", error);
            }
        }

        // Check discharge_limit
        if let Some(value) = &self.discharge_limit {
            if let Err(e) = validate_discharge_limit(value) {
                errors.add("discharge_limit", e);
            }
        }

        // Check discharge_limit_l2
        if let Some(value) = &self.discharge_limit_l2 {
            if let Err(e) = validate_discharge_limit(value) {
                errors.add("discharge_limit_l2", e);
            }
        }

        // Check discharge_limit_l3
        if let Some(value) = &self.discharge_limit_l3 {
            if let Err(e) = validate_discharge_limit(value) {
                errors.add("discharge_limit_l3", e);
            }
        }

        // Validate v2x_freq_watt_curve length
        if let Some(curve) = &self.v2x_freq_watt_curve {
            if curve.is_empty() || curve.len() > 20 {
                let mut error = ValidationError::new("v2x_freq_watt_curve_length");
                error.message =
                    Some("v2x_freq_watt_curve must have between 1 and 20 points".into());
                errors.add("v2x_freq_watt_curve", error);
            }
        }

        // Validate v2x_signal_watt_curve length
        if let Some(curve) = &self.v2x_signal_watt_curve {
            if curve.is_empty() || curve.len() > 20 {
                let mut error = ValidationError::new("v2x_signal_watt_curve_length");
                error.message =
                    Some("v2x_signal_watt_curve must have between 1 and 20 points".into());
                errors.add("v2x_signal_watt_curve", error);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::OperationModeEnumType;
    use rust_decimal_macros::dec;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_new_charging_schedule_period() {
        let period = ChargingSchedulePeriodType::new(0, dec!(16.0));

        assert_eq!(period.start_period(), 0);
        assert_eq!(period.limit(), &dec!(16.0));
        assert_eq!(period.limit_l2(), None);
        assert_eq!(period.limit_l3(), None);
        assert_eq!(period.number_phases(), None);
        assert_eq!(period.phase_to_use(), None);
        assert_eq!(period.discharge_limit(), None);
        assert_eq!(period.discharge_limit_l2(), None);
        assert_eq!(period.discharge_limit_l3(), None);
        assert_eq!(period.setpoint, None);
        assert_eq!(period.setpoint_l2, None);
        assert_eq!(period.setpoint_l3, None);
        assert_eq!(period.setpoint_reactive, None);
        assert_eq!(period.setpoint_reactive_l2, None);
        assert_eq!(period.setpoint_reactive_l3, None);
        assert_eq!(period.preconditioning_request, None);
        assert_eq!(period.evse_sleep, None);
        assert_eq!(period.v2x_baseline, None);
        assert_eq!(period.operation_mode, None);
        assert_eq!(period.v2x_freq_watt_curve, None);
        assert_eq!(period.v2x_signal_watt_curve, None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_new_from_f64() {
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);

        assert_eq!(period.start_period(), 0);
        assert_eq!(period.limit(), &dec!(16.0));
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let period = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_limit_l2(dec!(16.0))
            .with_limit_l3(dec!(16.0))
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-10.0))
            .with_discharge_limit_l3(dec!(-10.0))
            .with_number_phases(3)
            .with_phase_to_use(1)
            .with_custom_data(custom_data.clone());

        assert_eq!(period.start_period(), 0);
        assert_eq!(period.limit(), &dec!(16.0));
        assert_eq!(period.limit_l2(), Some(&dec!(16.0)));
        assert_eq!(period.limit_l3(), Some(&dec!(16.0)));
        assert_eq!(period.discharge_limit(), Some(&dec!(-10.0)));
        assert_eq!(period.discharge_limit_l2(), Some(&dec!(-10.0)));
        assert_eq!(period.discharge_limit_l3(), Some(&dec!(-10.0)));
        assert_eq!(period.number_phases(), Some(3));
        assert_eq!(period.phase_to_use(), Some(1));
        assert_eq!(period.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_with_f64_methods() {
        let period = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_limit_l2_f64(16.0)
            .with_limit_l3_f64(16.0)
            .with_discharge_limit_f64(-10.0)
            .with_discharge_limit_l2_f64(-10.0)
            .with_discharge_limit_l3_f64(-10.0);

        assert_eq!(period.limit_l2(), Some(&dec!(16.0)));
        assert_eq!(period.limit_l3(), Some(&dec!(16.0)));
        assert_eq!(period.discharge_limit(), Some(&dec!(-10.0)));
        assert_eq!(period.discharge_limit_l2(), Some(&dec!(-10.0)));
        assert_eq!(period.discharge_limit_l3(), Some(&dec!(-10.0)));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));

        period
            .set_start_period(3600)
            .set_limit(dec!(32.0))
            .set_limit_l2(Some(dec!(32.0)))
            .set_limit_l3(Some(dec!(32.0)))
            .set_discharge_limit(Some(dec!(-15.0)))
            .set_discharge_limit_l2(Some(dec!(-15.0)))
            .set_discharge_limit_l3(Some(dec!(-15.0)))
            .set_number_phases(Some(3))
            .set_phase_to_use(Some(2))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(period.start_period(), 3600);
        assert_eq!(period.limit(), &dec!(32.0));
        assert_eq!(period.limit_l2(), Some(&dec!(32.0)));
        assert_eq!(period.limit_l3(), Some(&dec!(32.0)));
        assert_eq!(period.discharge_limit(), Some(&dec!(-15.0)));
        assert_eq!(period.discharge_limit_l2(), Some(&dec!(-15.0)));
        assert_eq!(period.discharge_limit_l3(), Some(&dec!(-15.0)));
        assert_eq!(period.number_phases(), Some(3));
        assert_eq!(period.phase_to_use(), Some(2));
        assert_eq!(period.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        period
            .set_limit_l2(None)
            .set_limit_l3(None)
            .set_discharge_limit(None)
            .set_discharge_limit_l2(None)
            .set_discharge_limit_l3(None)
            .set_number_phases(None)
            .set_phase_to_use(None)
            .set_custom_data(None);

        assert_eq!(period.limit_l2(), None);
        assert_eq!(period.limit_l3(), None);
        assert_eq!(period.discharge_limit(), None);
        assert_eq!(period.discharge_limit_l2(), None);
        assert_eq!(period.discharge_limit_l3(), None);
        assert_eq!(period.number_phases(), None);
        assert_eq!(period.phase_to_use(), None);
        assert_eq!(period.custom_data(), None);
    }

    #[test]
    fn test_discharge_limit_validation() {
        // Test the validate_discharge_limit function directly
        assert!(
            validate_discharge_limit(&dec!(-10.0)).is_ok(),
            "Negative discharge limit should be valid"
        );
        assert!(
            validate_discharge_limit(&dec!(0.0)).is_ok(),
            "Zero discharge limit should be valid"
        );
        assert!(
            validate_discharge_limit(&dec!(5.0)).is_err(),
            "Positive discharge limit should be invalid"
        );

        // Valid case: discharge_limit is negative
        let valid_period_negative =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit(dec!(-10.0));
        assert!(
            valid_period_negative.validate().is_ok(),
            "Period with negative discharge limit should be valid"
        );

        // Valid case: discharge_limit is zero
        let valid_period_zero =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit(dec!(0.0));
        assert!(
            valid_period_zero.validate().is_ok(),
            "Period with zero discharge limit should be valid"
        );

        // Invalid case: discharge_limit is positive
        let invalid_period =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit(dec!(5.0));
        assert!(
            invalid_period.validate().is_err(),
            "Period with positive discharge limit should be invalid"
        );

        // Test discharge_limit_l2 validation
        let invalid_period_l2 =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit_l2(dec!(5.0));
        assert!(
            invalid_period_l2.validate().is_err(),
            "Period with positive discharge_limit_l2 should be invalid"
        );

        // Test discharge_limit_l3 validation
        let invalid_period_l3 =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit_l3(dec!(5.0));
        assert!(
            invalid_period_l3.validate().is_err(),
            "Period with positive discharge_limit_l3 should be invalid"
        );

        // Test multiple discharge limits
        let valid_period_multiple = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-5.0))
            .with_discharge_limit_l3(dec!(-2.0));
        assert!(
            valid_period_multiple.validate().is_ok(),
            "Period with all negative discharge limits should be valid"
        );

        let invalid_period_multiple = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(5.0)) // Invalid: positive
            .with_discharge_limit_l3(dec!(-2.0));
        assert!(
            invalid_period_multiple.validate().is_err(),
            "Period with one positive discharge limit should be invalid"
        );
    }

    #[test]
    fn test_validate_method() {
        // Valid case
        let valid_period = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_discharge_limit(dec!(-10.0))
            .with_number_phases(3)
            .with_phase_to_use(2);

        assert!(valid_period.validate().is_ok());

        // Invalid case: discharge_limit is positive
        let invalid_period =
            ChargingSchedulePeriodType::new(0, dec!(16.0)).with_discharge_limit(dec!(10.0));

        assert!(invalid_period.validate().is_err());

        // Check the error message
        let err = invalid_period.validate().unwrap_err();
        let field_errors = err.field_errors();
        let discharge_errors = field_errors.get("discharge_limit");
        assert!(discharge_errors.is_some());
        assert!(discharge_errors.unwrap()[0]
            .message
            .as_ref()
            .unwrap()
            .contains("less than or equal to zero"));

        // Invalid case: number_phases out of range
        let invalid_period = ChargingSchedulePeriodType::new(0, dec!(16.0)).with_number_phases(4); // Valid range is 0-3

        assert!(invalid_period.validate().is_err());

        // Check the error message
        let err = invalid_period.validate().unwrap_err();
        let field_errors = err.field_errors();
        let phase_errors = field_errors.get("number_phases");
        assert!(phase_errors.is_some());
        assert!(phase_errors.unwrap()[0]
            .message
            .as_ref()
            .unwrap()
            .contains("between 0 and 3"));

        // Invalid case: phase_to_use out of range
        let invalid_period = ChargingSchedulePeriodType::new(0, dec!(16.0)).with_phase_to_use(0); // Valid range is 1-3

        assert!(invalid_period.validate().is_err());

        // Check the error message
        let err = invalid_period.validate().unwrap_err();
        let field_errors = err.field_errors();
        let phase_errors = field_errors.get("phase_to_use");
        assert!(phase_errors.is_some());
        assert!(phase_errors.unwrap()[0]
            .message
            .as_ref()
            .unwrap()
            .contains("between 1 and 3"));

        // Invalid case: v2x_freq_watt_curve empty
        let invalid_period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        let mut period_with_empty_curve = invalid_period.clone();
        period_with_empty_curve.v2x_freq_watt_curve = Some(vec![]);

        assert!(period_with_empty_curve.validate().is_err());

        // Check the error message
        let err = period_with_empty_curve.validate().unwrap_err();
        let field_errors = err.field_errors();
        let curve_errors = field_errors.get("v2x_freq_watt_curve");
        assert!(curve_errors.is_some());
        assert!(curve_errors.unwrap()[0]
            .message
            .as_ref()
            .unwrap()
            .contains("between 1 and 20"));

        // Multiple validation errors
        let invalid_period = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_discharge_limit(dec!(10.0))
            .with_number_phases(4)
            .with_phase_to_use(0);

        let err = invalid_period.validate().unwrap_err();
        assert_eq!(err.field_errors().len(), 3); // Should have 3 field errors
    }

    #[test]
    fn test_serialization_deserialization() {
        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0))
            .with_limit_l2(dec!(16.0))
            .with_limit_l3(dec!(16.0))
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-10.0))
            .with_discharge_limit_l3(dec!(-10.0))
            .with_number_phases(3)
            .with_phase_to_use(1);

        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        let json = to_string(&period).unwrap();
        println!("Serialized JSON: {}", json);

        // Create a JSON string for deserialization
        let json = r#"{
            "startPeriod": 0,
            "limit": 16.0,
            "limit_L2": 16.0,
            "limit_L3": 16.0,
            "numberPhases": 3,
            "phaseToUse": 1,
            "dischargeLimit": -10.0,
            "dischargeLimit_L2": -10.0,
            "dischargeLimit_L3": -10.0,
            "setpoint": 20.0,
            "setpoint_L2": 21.0,
            "setpoint_L3": 22.0,
            "setpointReactive": 5.0,
            "setpointReactive_L2": 6.0,
            "setpointReactive_L3": 7.0,
            "v2xBaseline": 50.0
        }"#;

        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();

        assert_eq!(deserialized.start_period(), period.start_period());
        assert_eq!(deserialized.limit(), period.limit());
        assert_eq!(deserialized.limit_l2(), period.limit_l2());
        assert_eq!(deserialized.limit_l3(), period.limit_l3());
        assert_eq!(deserialized.discharge_limit(), period.discharge_limit());
        assert_eq!(
            deserialized.discharge_limit_l2(),
            period.discharge_limit_l2()
        );
        assert_eq!(
            deserialized.discharge_limit_l3(),
            period.discharge_limit_l3()
        );
        assert_eq!(deserialized.number_phases(), period.number_phases());
        assert_eq!(deserialized.phase_to_use(), period.phase_to_use());
        assert_eq!(deserialized.setpoint, period.setpoint);
        assert_eq!(deserialized.setpoint_l2, period.setpoint_l2);
        assert_eq!(deserialized.setpoint_l3, period.setpoint_l3);
        assert_eq!(deserialized.setpoint_reactive, period.setpoint_reactive);
        assert_eq!(
            deserialized.setpoint_reactive_l2,
            period.setpoint_reactive_l2
        );
        assert_eq!(
            deserialized.setpoint_reactive_l3,
            period.setpoint_reactive_l3
        );
        assert_eq!(deserialized.v2x_baseline, period.v2x_baseline);
    }

    #[test]
    fn test_operation_mode() {
        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));

        // Test setting operation mode
        period.operation_mode = Some(OperationModeEnumType::CentralSetpoint);
        assert_eq!(
            period.operation_mode,
            Some(OperationModeEnumType::CentralSetpoint)
        );

        // Test serialization with operation mode
        let json = to_string(&period).unwrap();
        let expected_json_contains = r#""operationMode":"CentralSetpoint"#;
        assert!(json.contains(expected_json_contains));

        // Test deserialization with operation mode
        let json = r#"{
            "startPeriod":0,
            "limit":16.0,
            "limit_L2":16.0,
            "limit_L3":16.0,
            "dischargeLimit":-10.0,
            "dischargeLimit_L2":-10.0,
            "dischargeLimit_L3":-10.0,
            "setpoint": 20.0,
            "setpoint_L2": 21.0,
            "setpoint_L3": 22.0,
            "setpointReactive": 5.0,
            "setpointReactive_L2": 6.0,
            "setpointReactive_L3": 7.0,
            "v2xBaseline": 50.0,
            "operationMode":"LocalFrequency"
        }"#;
        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();
        assert_eq!(
            deserialized.operation_mode,
            Some(OperationModeEnumType::LocalFrequency)
        );
    }

    #[test]
    fn test_v2x_curves() {
        use crate::v2_1::datatypes::{V2XFreqWattPointType, V2XSignalWattPointType};

        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        // Create test points
        let freq_point1 = V2XFreqWattPointType::new_from_f64(50.0, -30.0);
        let freq_point2 = V2XFreqWattPointType::new_from_f64(51.0, -20.0);
        let signal_point1 = V2XSignalWattPointType::new_with_f64_power(75, -30.0);
        let signal_point2 = V2XSignalWattPointType::new_with_f64_power(80, -20.0);

        // Set the curves
        period.v2x_freq_watt_curve = Some(vec![freq_point1.clone(), freq_point2.clone()]);
        period.v2x_signal_watt_curve = Some(vec![signal_point1.clone(), signal_point2.clone()]);

        // Verify the curves
        assert_eq!(period.v2x_freq_watt_curve.as_ref().unwrap().len(), 2);
        assert_eq!(
            period.v2x_freq_watt_curve.as_ref().unwrap()[0].frequency(),
            Decimal::from_f64(50.0).unwrap()
        );
        assert_eq!(
            period.v2x_freq_watt_curve.as_ref().unwrap()[1].frequency(),
            Decimal::from_f64(51.0).unwrap()
        );

        assert_eq!(period.v2x_signal_watt_curve.as_ref().unwrap().len(), 2);
        assert_eq!(
            period.v2x_signal_watt_curve.as_ref().unwrap()[0].signal(),
            75
        );
        assert_eq!(
            period.v2x_signal_watt_curve.as_ref().unwrap()[1].signal(),
            80
        );

        // Test serialization with curves
        let json = to_string(&period).unwrap();
        println!("Serialized JSON: {}", json);

        // The actual serialization format might be different, so we check for the presence of key fields
        assert!(json.contains(r#""v2xFreqWattCurve""#));
        assert!(
            json.contains(r#""frequency":"50""#)
                || json.contains(r#""frequency":50"#)
                || json.contains(r#""frequency":50.0"#)
                || json.contains(r#""freq":50.0"#)
        );
        assert!(
            json.contains(r#""frequency":"51""#)
                || json.contains(r#""frequency":51"#)
                || json.contains(r#""frequency":51.0"#)
                || json.contains(r#""freq":51.0"#)
        );
        assert!(
            json.contains(r#""power":"-30""#)
                || json.contains(r#""power":-30"#)
                || json.contains(r#""power":-30.0"#)
        );
        assert!(
            json.contains(r#""power":"-20""#)
                || json.contains(r#""power":-20"#)
                || json.contains(r#""power":-20.0"#)
        );

        // Check for signal-watt curve, but be more flexible with the exact format
        assert!(json.contains(r#""v2xSignalWattCurve""#));
        assert!(json.contains(r#""signal":75"#) || json.contains(r#""signal":75.0"#));
        assert!(json.contains(r#""signal":80"#) || json.contains(r#""signal":80.0"#));

        // Test deserialization with curves
        let json = r#"{
            "startPeriod": 0,
            "limit": 16.0,
            "limit_L2": 16.0,
            "limit_L3": 16.0,
            "dischargeLimit": -10.0,
            "dischargeLimit_L2": -10.0,
            "dischargeLimit_L3": -10.0,
            "setpoint": 20.0,
            "setpoint_L2": 21.0,
            "setpoint_L3": 22.0,
            "setpointReactive": 5.0,
            "setpointReactive_L2": 6.0,
            "setpointReactive_L3": 7.0,
            "v2xBaseline": 50.0,
            "v2xFreqWattCurve": [{"frequency": 49.5, "power": -25.0}],
            "v2xSignalWattCurve": [{"signal": 60, "power": -15.0}]
        }"#;

        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();
        assert_eq!(deserialized.v2x_freq_watt_curve.as_ref().unwrap().len(), 1);
        assert_eq!(
            deserialized.v2x_freq_watt_curve.as_ref().unwrap()[0].frequency(),
            Decimal::from_f64(49.5).unwrap()
        );
        assert_eq!(
            deserialized.v2x_signal_watt_curve.as_ref().unwrap().len(),
            1
        );
        assert_eq!(
            deserialized.v2x_signal_watt_curve.as_ref().unwrap()[0].signal(),
            60
        );
    }

    #[test]
    fn test_setpoint_fields() {
        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.v2x_baseline = Some(dec!(50.0));

        // Set setpoint fields
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));

        // Verify setpoint fields
        assert_eq!(period.setpoint, Some(dec!(20.0)));
        assert_eq!(period.setpoint_l2, Some(dec!(21.0)));
        assert_eq!(period.setpoint_l3, Some(dec!(22.0)));
        assert_eq!(period.setpoint_reactive, Some(dec!(5.0)));
        assert_eq!(period.setpoint_reactive_l2, Some(dec!(6.0)));
        assert_eq!(period.setpoint_reactive_l3, Some(dec!(7.0)));

        // Test serialization with setpoint fields
        let json = to_string(&period).unwrap();
        println!("Serialized JSON: {}", json);
        assert!(json.contains(r#""setpoint":20.0"#));
        assert!(json.contains(r#""setpoint_L2":21.0"#));
        assert!(json.contains(r#""setpoint_L3":22.0"#));
        assert!(json.contains(r#""setpointReactive":5.0"#));
        assert!(json.contains(r#""setpointReactive_L2":6.0"#));
        assert!(json.contains(r#""setpointReactive_L3":7.0"#));

        // Test deserialization with setpoint fields
        let json = r#"{
            "startPeriod": 0,
            "limit": 16.0,
            "limit_L2": 16.0,
            "limit_L3": 16.0,
            "dischargeLimit": -10.0,
            "dischargeLimit_L2": -10.0,
            "dischargeLimit_L3": -10.0,
            "v2xBaseline": 50.0,
            "setpoint": 25.0,
            "setpoint_L2": 26.0,
            "setpoint_L3": 27.0,
            "setpointReactive": 8.0,
            "setpointReactive_L2": 9.0,
            "setpointReactive_L3": 10.0
        }"#;

        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();
        assert_eq!(deserialized.setpoint, Some(dec!(25.0)));
        assert_eq!(deserialized.setpoint_l2, Some(dec!(26.0)));
        assert_eq!(deserialized.setpoint_l3, Some(dec!(27.0)));
        assert_eq!(deserialized.setpoint_reactive, Some(dec!(8.0)));
        assert_eq!(deserialized.setpoint_reactive_l2, Some(dec!(9.0)));
        assert_eq!(deserialized.setpoint_reactive_l3, Some(dec!(10.0)));
    }

    #[test]
    fn test_boolean_fields() {
        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        // Set boolean fields
        period.preconditioning_request = Some(true);
        period.evse_sleep = Some(false);

        // Verify boolean fields
        assert_eq!(period.preconditioning_request, Some(true));
        assert_eq!(period.evse_sleep, Some(false));

        // Test serialization with boolean fields
        let json = to_string(&period).unwrap();
        assert!(json.contains(r#""preconditioningRequest":true"#));
        assert!(json.contains(r#""evseSleep":false"#));

        // Test deserialization with boolean fields
        let json = r#"{
            "startPeriod": 0,
            "limit": 16.0,
            "limit_L2": 16.0,
            "limit_L3": 16.0,
            "dischargeLimit": -10.0,
            "dischargeLimit_L2": -10.0,
            "dischargeLimit_L3": -10.0,
            "setpoint": 20.0,
            "setpoint_L2": 21.0,
            "setpoint_L3": 22.0,
            "setpointReactive": 5.0,
            "setpointReactive_L2": 6.0,
            "setpointReactive_L3": 7.0,
            "v2xBaseline": 50.0,
            "preconditioningRequest": false,
            "evseSleep": true
        }"#;

        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();
        assert_eq!(deserialized.preconditioning_request, Some(false));
        assert_eq!(deserialized.evse_sleep, Some(true));
    }

    #[test]
    fn test_v2x_baseline() {
        let mut period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));

        // Set v2x_baseline
        period.v2x_baseline = Some(dec!(50.0));

        // Verify v2x_baseline
        assert_eq!(period.v2x_baseline, Some(dec!(50.0)));

        // Test serialization with v2x_baseline
        let json = to_string(&period).unwrap();
        assert!(json.contains(r#""v2xBaseline":50.0"#));

        // Test deserialization with v2x_baseline
        let json = r#"{
            "startPeriod": 0,
            "limit": 16.0,
            "limit_L2": 16.0,
            "limit_L3": 16.0,
            "dischargeLimit": -10.0,
            "dischargeLimit_L2": -10.0,
            "dischargeLimit_L3": -10.0,
            "setpoint": 20.0,
            "setpoint_L2": 21.0,
            "setpoint_L3": 22.0,
            "setpointReactive": 5.0,
            "setpointReactive_L2": 6.0,
            "setpointReactive_L3": 7.0,
            "v2xBaseline": 75.0
        }"#;

        let deserialized: ChargingSchedulePeriodType = from_str(json).unwrap();
        assert_eq!(deserialized.v2x_baseline, Some(dec!(75.0)));
    }
}
