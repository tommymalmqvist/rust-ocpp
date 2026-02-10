use super::custom_data::CustomDataType;
use crate::v2_1::helpers::validator::validate_discharge_limit;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Updates to a ChargingSchedulePeriodType for dynamic charging profiles.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleUpdateType {
    /// Optional only when not required by the _operationMode_, as in CentralSetpoint, ExternalSetpoint, ExternalLimits, LocalFrequency,  LocalLoadBalancing.
    /// Charging rate limit during the schedule period, in the applicable _chargingRateUnit_.
    /// This SHOULD be a non-negative value; a negative value is only supported for backwards compatibility with older systems that use a negative value to specify a discharging limit.
    /// For AC this field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L2 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l2: Option<f32>,

    /// *(2.1)* Charging rate limit on phase L3 in the applicable _chargingRateUnit_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_l3: Option<f32>,

    /// *(2.1)* Limit in _chargingRateUnit_ that the EV is allowed to discharge with. Note, these are negative values in order to be consistent with _setpoint_, which can be positive and negative.  +\r\nFor AC this field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit: Option<Decimal>,

    /// *(2.1)* Limit in _chargingRateUnit_ on phase L2 that the EV is allowed to discharge with.
    #[serde(rename = "dischargeLimit_L2")]
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit_l2: Option<Decimal>,

    /// *(2.1)* Limit in _chargingRateUnit_ on phase L3 that the EV is allowed to discharge with.
    #[serde(rename = "dischargeLimit_L3")]
    #[validate(custom(function = "validate_discharge_limit"))]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub discharge_limit_l3: Option<Decimal>,

    /// *(2.1)* Setpoint in _chargingRateUnit_ that the EV should follow as close as possible. Use negative values for discharging. +\r\nWhen a limit and/or _dischargeLimit_ are given the overshoot when following _setpoint_ must remain within these values.\r\nThis field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint: Option<Decimal>,

    /// *(2.1)* Setpoint in _chargingRateUnit_ on phase L2 that the EV should follow as close as possible. Use negative values for discharging.
    #[serde(rename = "setpoint_L2")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_l2: Option<Decimal>,

    /// *(2.1)* Setpoint in _chargingRateUnit_ on phase L3 that the EV should follow as close as possible. Use negative values for discharging.
    #[serde(rename = "setpoint_L3")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_l3: Option<Decimal>,

    /// *(2.1)* Setpoint for reactive power (or current) in _chargingRateUnit_ that the EV should follow as closely as possible. Positive values for inductive, negative for capacitive reactive power or current.  +\r\nThis field represents the sum of all phases, unless values are provided for L2 and L3, in which case this field represents phase L1.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive: Option<Decimal>,

    /// *(2.1)* Setpoint for reactive power (or current) on phase L2 in _chargingRateUnit_ that the EV should follow as closely as possible. Positive values for inductive, negative for capacitive reactive power or current.
    #[serde(rename = "setpointReactive_L2")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive_l2: Option<Decimal>,

    /// *(2.1)* Setpoint for reactive power (or current) on phase L3 in _chargingRateUnit_ that the EV should follow as closely as possible. Positive values for inductive, negative for capacitive reactive power or current.
    #[serde(rename = "setpointReactive_L3")]
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub setpoint_reactive_l3: Option<Decimal>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingScheduleUpdateType {
    /// Creates a new `ChargingScheduleUpdateType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingScheduleUpdateType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            custom_data: None,
            limit: None,
            limit_l2: None,
            limit_l3: None,
            discharge_limit: None,
            discharge_limit_l2: None,
            discharge_limit_l3: None,
            setpoint: None,
            setpoint_l2: None,
            setpoint_l3: None,
            setpoint_reactive: None,
            setpoint_reactive_l2: None,
            setpoint_reactive_l3: None,
        }
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
    pub fn with_limit(mut self, limit: f32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the charging rate limit on phase L2.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit on phase L2
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l2(mut self, limit_l2: f32) -> Self {
        self.limit_l2 = Some(limit_l2);
        self
    }

    /// Sets the charging rate limit on phase L3.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit on phase L3
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_limit_l3(mut self, limit_l3: f32) -> Self {
        self.limit_l3 = Some(limit_l3);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule update
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the charging rate limit.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit
    pub fn limit(&self) -> Option<f32> {
        self.limit
    }

    /// Sets the charging rate limit.
    ///
    /// # Arguments
    ///
    /// * `limit` - Charging rate limit during the schedule period, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit(&mut self, limit: Option<f32>) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Gets the charging rate limit on phase L2.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit on phase L2
    pub fn limit_l2(&self) -> Option<f32> {
        self.limit_l2
    }

    /// Sets the charging rate limit on phase L2.
    ///
    /// # Arguments
    ///
    /// * `limit_l2` - Charging rate limit on phase L2, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l2(&mut self, limit_l2: Option<f32>) -> &mut Self {
        self.limit_l2 = limit_l2;
        self
    }

    /// Gets the charging rate limit on phase L3.
    ///
    /// # Returns
    ///
    /// An optional charging rate limit on phase L3
    pub fn limit_l3(&self) -> Option<f32> {
        self.limit_l3
    }

    /// Sets the charging rate limit on phase L3.
    ///
    /// # Arguments
    ///
    /// * `limit_l3` - Charging rate limit on phase L3, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_l3(&mut self, limit_l3: Option<f32>) -> &mut Self {
        self.limit_l3 = limit_l3;
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
    /// * `custom_data` - Custom data for this charging schedule update, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
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
    /// * `discharge_limit` - Discharge limit (must be non-positive), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit(&mut self, discharge_limit: Option<Decimal>) -> &mut Self {
        self.discharge_limit = discharge_limit;
        self
    }

    /// Sets the discharge limit.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit` - Discharge limit (must be non-positive)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit(mut self, discharge_limit: Decimal) -> Self {
        self.discharge_limit = Some(discharge_limit);
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
    /// * `discharge_limit_l2` - Discharge limit for phase L2 (must be non-positive), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit_l2(&mut self, discharge_limit_l2: Option<Decimal>) -> &mut Self {
        self.discharge_limit_l2 = discharge_limit_l2;
        self
    }

    /// Sets the discharge limit for phase L2.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l2` - Discharge limit for phase L2 (must be non-positive)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l2(mut self, discharge_limit_l2: Decimal) -> Self {
        self.discharge_limit_l2 = Some(discharge_limit_l2);
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
    /// * `discharge_limit_l3` - Discharge limit for phase L3 (must be non-positive), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_discharge_limit_l3(&mut self, discharge_limit_l3: Option<Decimal>) -> &mut Self {
        self.discharge_limit_l3 = discharge_limit_l3;
        self
    }

    /// Sets the discharge limit for phase L3.
    ///
    /// # Arguments
    ///
    /// * `discharge_limit_l3` - Discharge limit for phase L3 (must be non-positive)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_discharge_limit_l3(mut self, discharge_limit_l3: Decimal) -> Self {
        self.discharge_limit_l3 = Some(discharge_limit_l3);
        self
    }

    /// Gets the setpoint.
    ///
    /// # Returns
    ///
    /// An optional setpoint
    pub fn setpoint(&self) -> Option<&Decimal> {
        self.setpoint.as_ref()
    }

    /// Sets the setpoint.
    ///
    /// # Arguments
    ///
    /// * `setpoint` - Setpoint, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint(&mut self, setpoint: Option<Decimal>) -> &mut Self {
        self.setpoint = setpoint;
        self
    }

    /// Sets the setpoint.
    ///
    /// # Arguments
    ///
    /// * `setpoint` - Setpoint
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint(mut self, setpoint: Decimal) -> Self {
        self.setpoint = Some(setpoint);
        self
    }

    /// Gets the setpoint for phase L2.
    ///
    /// # Returns
    ///
    /// An optional setpoint for phase L2
    pub fn setpoint_l2(&self) -> Option<&Decimal> {
        self.setpoint_l2.as_ref()
    }

    /// Sets the setpoint for phase L2.
    ///
    /// # Arguments
    ///
    /// * `setpoint_l2` - Setpoint for phase L2, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint_l2(&mut self, setpoint_l2: Option<Decimal>) -> &mut Self {
        self.setpoint_l2 = setpoint_l2;
        self
    }

    /// Sets the setpoint for phase L2.
    ///
    /// # Arguments
    ///
    /// * `setpoint_l2` - Setpoint for phase L2
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint_l2(mut self, setpoint_l2: Decimal) -> Self {
        self.setpoint_l2 = Some(setpoint_l2);
        self
    }

    /// Gets the setpoint for phase L3.
    ///
    /// # Returns
    ///
    /// An optional setpoint for phase L3
    pub fn setpoint_l3(&self) -> Option<&Decimal> {
        self.setpoint_l3.as_ref()
    }

    /// Sets the setpoint for phase L3.
    ///
    /// # Arguments
    ///
    /// * `setpoint_l3` - Setpoint for phase L3, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint_l3(&mut self, setpoint_l3: Option<Decimal>) -> &mut Self {
        self.setpoint_l3 = setpoint_l3;
        self
    }

    /// Sets the setpoint for phase L3.
    ///
    /// # Arguments
    ///
    /// * `setpoint_l3` - Setpoint for phase L3
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint_l3(mut self, setpoint_l3: Decimal) -> Self {
        self.setpoint_l3 = Some(setpoint_l3);
        self
    }

    /// Gets the reactive power setpoint.
    ///
    /// # Returns
    ///
    /// An optional reactive power setpoint
    pub fn setpoint_reactive(&self) -> Option<&Decimal> {
        self.setpoint_reactive.as_ref()
    }

    /// Sets the reactive power setpoint.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive` - Reactive power setpoint, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint_reactive(&mut self, setpoint_reactive: Option<Decimal>) -> &mut Self {
        self.setpoint_reactive = setpoint_reactive;
        self
    }

    /// Sets the reactive power setpoint.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive` - Reactive power setpoint
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint_reactive(mut self, setpoint_reactive: Decimal) -> Self {
        self.setpoint_reactive = Some(setpoint_reactive);
        self
    }

    /// Gets the reactive power setpoint for phase L2.
    ///
    /// # Returns
    ///
    /// An optional reactive power setpoint for phase L2
    pub fn setpoint_reactive_l2(&self) -> Option<&Decimal> {
        self.setpoint_reactive_l2.as_ref()
    }

    /// Sets the reactive power setpoint for phase L2.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive_l2` - Reactive power setpoint for phase L2, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint_reactive_l2(&mut self, setpoint_reactive_l2: Option<Decimal>) -> &mut Self {
        self.setpoint_reactive_l2 = setpoint_reactive_l2;
        self
    }

    /// Sets the reactive power setpoint for phase L2.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive_l2` - Reactive power setpoint for phase L2
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint_reactive_l2(mut self, setpoint_reactive_l2: Decimal) -> Self {
        self.setpoint_reactive_l2 = Some(setpoint_reactive_l2);
        self
    }

    /// Gets the reactive power setpoint for phase L3.
    ///
    /// # Returns
    ///
    /// An optional reactive power setpoint for phase L3
    pub fn setpoint_reactive_l3(&self) -> Option<&Decimal> {
        self.setpoint_reactive_l3.as_ref()
    }

    /// Sets the reactive power setpoint for phase L3.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive_l3` - Reactive power setpoint for phase L3, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_setpoint_reactive_l3(&mut self, setpoint_reactive_l3: Option<Decimal>) -> &mut Self {
        self.setpoint_reactive_l3 = setpoint_reactive_l3;
        self
    }

    /// Sets the reactive power setpoint for phase L3.
    ///
    /// # Arguments
    ///
    /// * `setpoint_reactive_l3` - Reactive power setpoint for phase L3
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_setpoint_reactive_l3(mut self, setpoint_reactive_l3: Decimal) -> Self {
        self.setpoint_reactive_l3 = Some(setpoint_reactive_l3);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use serde_json::{from_str, to_string};
    use validator::Validate;

    #[test]
    fn test_new_charging_schedule_update() {
        let update = ChargingScheduleUpdateType::new();

        assert_eq!(update.limit(), None);
        assert_eq!(update.limit_l2(), None);
        assert_eq!(update.limit_l3(), None);
        assert_eq!(update.discharge_limit(), None);
        assert_eq!(update.discharge_limit_l2(), None);
        assert_eq!(update.discharge_limit_l3(), None);
        assert_eq!(update.setpoint(), None);
        assert_eq!(update.setpoint_l2(), None);
        assert_eq!(update.setpoint_l3(), None);
        assert_eq!(update.setpoint_reactive(), None);
        assert_eq!(update.setpoint_reactive_l2(), None);
        assert_eq!(update.setpoint_reactive_l3(), None);
        assert_eq!(update.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let update = ChargingScheduleUpdateType::new()
            .with_limit(16.0)
            .with_limit_l2(16.0)
            .with_limit_l3(16.0)
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-10.0))
            .with_discharge_limit_l3(dec!(-10.0))
            .with_setpoint(dec!(20.0))
            .with_setpoint_l2(dec!(20.0))
            .with_setpoint_l3(dec!(20.0))
            .with_setpoint_reactive(dec!(5.0))
            .with_setpoint_reactive_l2(dec!(5.0))
            .with_setpoint_reactive_l3(dec!(5.0))
            .with_custom_data(custom_data.clone());

        assert_eq!(update.limit(), Some(16.0));
        assert_eq!(update.limit_l2(), Some(16.0));
        assert_eq!(update.limit_l3(), Some(16.0));
        assert_eq!(update.discharge_limit(), Some(&dec!(-10.0)));
        assert_eq!(update.discharge_limit_l2(), Some(&dec!(-10.0)));
        assert_eq!(update.discharge_limit_l3(), Some(&dec!(-10.0)));
        assert_eq!(update.setpoint(), Some(&dec!(20.0)));
        assert_eq!(update.setpoint_l2(), Some(&dec!(20.0)));
        assert_eq!(update.setpoint_l3(), Some(&dec!(20.0)));
        assert_eq!(update.setpoint_reactive(), Some(&dec!(5.0)));
        assert_eq!(update.setpoint_reactive_l2(), Some(&dec!(5.0)));
        assert_eq!(update.setpoint_reactive_l3(), Some(&dec!(5.0)));
        assert_eq!(update.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut update = ChargingScheduleUpdateType::new();

        update
            .set_limit(Some(32.0))
            .set_limit_l2(Some(32.0))
            .set_limit_l3(Some(32.0))
            .set_discharge_limit(Some(dec!(-15.0)))
            .set_discharge_limit_l2(Some(dec!(-15.0)))
            .set_discharge_limit_l3(Some(dec!(-15.0)))
            .set_setpoint(Some(dec!(25.0)))
            .set_setpoint_l2(Some(dec!(25.0)))
            .set_setpoint_l3(Some(dec!(25.0)))
            .set_setpoint_reactive(Some(dec!(8.0)))
            .set_setpoint_reactive_l2(Some(dec!(8.0)))
            .set_setpoint_reactive_l3(Some(dec!(8.0)))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(update.limit(), Some(32.0));
        assert_eq!(update.limit_l2(), Some(32.0));
        assert_eq!(update.limit_l3(), Some(32.0));
        assert_eq!(update.discharge_limit(), Some(&dec!(-15.0)));
        assert_eq!(update.discharge_limit_l2(), Some(&dec!(-15.0)));
        assert_eq!(update.discharge_limit_l3(), Some(&dec!(-15.0)));
        assert_eq!(update.setpoint(), Some(&dec!(25.0)));
        assert_eq!(update.setpoint_l2(), Some(&dec!(25.0)));
        assert_eq!(update.setpoint_l3(), Some(&dec!(25.0)));
        assert_eq!(update.setpoint_reactive(), Some(&dec!(8.0)));
        assert_eq!(update.setpoint_reactive_l2(), Some(&dec!(8.0)));
        assert_eq!(update.setpoint_reactive_l3(), Some(&dec!(8.0)));
        assert_eq!(update.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        update
            .set_limit(None)
            .set_limit_l2(None)
            .set_limit_l3(None)
            .set_discharge_limit(None)
            .set_discharge_limit_l2(None)
            .set_discharge_limit_l3(None)
            .set_setpoint(None)
            .set_setpoint_l2(None)
            .set_setpoint_l3(None)
            .set_setpoint_reactive(None)
            .set_setpoint_reactive_l2(None)
            .set_setpoint_reactive_l3(None)
            .set_custom_data(None);

        assert_eq!(update.limit(), None);
        assert_eq!(update.limit_l2(), None);
        assert_eq!(update.limit_l3(), None);
        assert_eq!(update.discharge_limit(), None);
        assert_eq!(update.discharge_limit_l2(), None);
        assert_eq!(update.discharge_limit_l3(), None);
        assert_eq!(update.setpoint(), None);
        assert_eq!(update.setpoint_l2(), None);
        assert_eq!(update.setpoint_l3(), None);
        assert_eq!(update.setpoint_reactive(), None);
        assert_eq!(update.setpoint_reactive_l2(), None);
        assert_eq!(update.setpoint_reactive_l3(), None);
        assert_eq!(update.custom_data(), None);
    }

    #[test]
    fn test_discharge_limit_validation() {
        // Valid case: discharge_limit is negative
        let valid_update_negative =
            ChargingScheduleUpdateType::new().with_discharge_limit(dec!(-10.0));
        assert!(
            valid_update_negative.validate().is_ok(),
            "Update with negative discharge limit should be valid"
        );

        // Valid case: discharge_limit is zero
        let valid_update_zero = ChargingScheduleUpdateType::new().with_discharge_limit(dec!(0.0));
        assert!(
            valid_update_zero.validate().is_ok(),
            "Update with zero discharge limit should be valid"
        );

        // Invalid case: discharge_limit is positive
        let invalid_update = ChargingScheduleUpdateType::new().with_discharge_limit(dec!(5.0));
        assert!(
            invalid_update.validate().is_err(),
            "Update with positive discharge limit should be invalid"
        );

        // Test discharge_limit_l2 validation
        let invalid_update_l2 =
            ChargingScheduleUpdateType::new().with_discharge_limit_l2(dec!(5.0));
        assert!(
            invalid_update_l2.validate().is_err(),
            "Update with positive discharge_limit_l2 should be invalid"
        );

        // Test discharge_limit_l3 validation
        let invalid_update_l3 =
            ChargingScheduleUpdateType::new().with_discharge_limit_l3(dec!(5.0));
        assert!(
            invalid_update_l3.validate().is_err(),
            "Update with positive discharge_limit_l3 should be invalid"
        );

        // Test multiple discharge limits
        let valid_update_multiple = ChargingScheduleUpdateType::new()
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-5.0))
            .with_discharge_limit_l3(dec!(-2.0));
        assert!(
            valid_update_multiple.validate().is_ok(),
            "Update with all negative discharge limits should be valid"
        );

        let invalid_update_multiple = ChargingScheduleUpdateType::new()
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(5.0)) // Invalid: positive
            .with_discharge_limit_l3(dec!(-2.0));
        assert!(
            invalid_update_multiple.validate().is_err(),
            "Update with one positive discharge limit should be invalid"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let update = ChargingScheduleUpdateType::new()
            .with_limit(16.0)
            .with_limit_l2(16.0)
            .with_limit_l3(16.0)
            .with_discharge_limit(dec!(-10.0))
            .with_discharge_limit_l2(dec!(-10.0))
            .with_discharge_limit_l3(dec!(-10.0))
            .with_setpoint(dec!(20.0))
            .with_setpoint_l2(dec!(20.0))
            .with_setpoint_l3(dec!(20.0))
            .with_setpoint_reactive(dec!(5.0))
            .with_setpoint_reactive_l2(dec!(5.0))
            .with_setpoint_reactive_l3(dec!(5.0));

        let json = to_string(&update).unwrap();
        let deserialized: ChargingScheduleUpdateType = from_str(&json).unwrap();

        assert_eq!(deserialized.limit(), update.limit());
        assert_eq!(deserialized.limit_l2(), update.limit_l2());
        assert_eq!(deserialized.limit_l3(), update.limit_l3());
        assert_eq!(deserialized.discharge_limit(), update.discharge_limit());
        assert_eq!(
            deserialized.discharge_limit_l2(),
            update.discharge_limit_l2()
        );
        assert_eq!(
            deserialized.discharge_limit_l3(),
            update.discharge_limit_l3()
        );
        assert_eq!(deserialized.setpoint(), update.setpoint());
        assert_eq!(deserialized.setpoint_l2(), update.setpoint_l2());
        assert_eq!(deserialized.setpoint_l3(), update.setpoint_l3());
        assert_eq!(deserialized.setpoint_reactive(), update.setpoint_reactive());
        assert_eq!(
            deserialized.setpoint_reactive_l2(),
            update.setpoint_reactive_l2()
        );
        assert_eq!(
            deserialized.setpoint_reactive_l3(),
            update.setpoint_reactive_l3()
        );
    }

    #[test]
    fn test_json_field_names() {
        let update = ChargingScheduleUpdateType::new()
            .with_discharge_limit_l2(dec!(-10.0))
            .with_discharge_limit_l3(dec!(-10.0))
            .with_setpoint_l2(dec!(20.0))
            .with_setpoint_l3(dec!(20.0))
            .with_setpoint_reactive_l2(dec!(5.0))
            .with_setpoint_reactive_l3(dec!(5.0));

        let json = to_string(&update).unwrap();

        // Check that the field names in the JSON match the expected camelCase format
        assert!(
            json.contains(r#""dischargeLimit_L2":"#),
            "JSON should contain dischargeLimit_L2 field"
        );
        assert!(
            json.contains(r#""dischargeLimit_L3":"#),
            "JSON should contain dischargeLimit_L3 field"
        );
        assert!(
            json.contains(r#""setpoint_L2":"#),
            "JSON should contain setpoint_L2 field"
        );
        assert!(
            json.contains(r#""setpoint_L3":"#),
            "JSON should contain setpoint_L3 field"
        );
        assert!(
            json.contains(r#""setpointReactive_L2":"#),
            "JSON should contain setpointReactive_L2 field"
        );
        assert!(
            json.contains(r#""setpointReactive_L3":"#),
            "JSON should contain setpointReactive_L3 field"
        );
    }

    #[test]
    fn test_mixed_values() {
        // Test with a mix of positive and negative values for different fields
        let update = ChargingScheduleUpdateType::new()
            .with_limit(16.0) // Positive charging limit
            .with_discharge_limit(dec!(-10.0)) // Negative discharge limit
            .with_setpoint(dec!(-5.0)) // Negative setpoint (discharging)
            .with_setpoint_reactive(dec!(3.0)); // Positive reactive power (inductive)

        assert_eq!(update.limit(), Some(16.0));
        assert_eq!(update.discharge_limit(), Some(&dec!(-10.0)));
        assert_eq!(update.setpoint(), Some(&dec!(-5.0)));
        assert_eq!(update.setpoint_reactive(), Some(&dec!(3.0)));

        // Validation should pass
        assert!(update.validate().is_ok());
    }
}
