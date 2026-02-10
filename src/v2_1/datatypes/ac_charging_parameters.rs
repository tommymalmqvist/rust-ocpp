use crate::v2_1::datatypes::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// EV AC charging parameters for ISO 15118-2
///
/// Contains parameters specific to AC charging according to ISO 15118-2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ACChargingParametersType {
    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EAmount
    /// *ISO 15118-20*: Dynamic/Scheduled_SEReqControlModeType: EVTargetEnergyRequest
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub energy_amount: Decimal,

    /// Minimum current (amps) supported by the electric vehicle (per phase).
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMinCurrent
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub ev_min_current: Decimal,

    /// Maximum current (amps) supported by the electric vehicle (per phase). Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxCurrent
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub ev_max_current: Decimal,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: AC_EVChargeParameterType: EVMaxVoltage
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub ev_max_voltage: Decimal,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ACChargingParametersType {
    /// Creates a new `ACChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// A new instance of `ACChargingParametersType` with optional fields set to `None`
    pub fn new(
        energy_amount: Decimal,
        ev_min_current: Decimal,
        ev_max_current: Decimal,
        ev_max_voltage: Decimal,
    ) -> Self {
        Self {
            energy_amount,
            ev_min_current,
            ev_max_current,
            ev_max_voltage,
            custom_data: None,
        }
    }

    /// Creates a new `ACChargingParametersType` from f64 values.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// A new instance of `ACChargingParametersType` with optional fields set to `None`
    pub fn new_from_f64(
        energy_amount: f64,
        ev_min_current: f64,
        ev_max_current: f64,
        ev_max_voltage: f64,
    ) -> Self {
        Self {
            energy_amount: Decimal::try_from(energy_amount).unwrap_or_default(),
            ev_min_current: Decimal::try_from(ev_min_current).unwrap_or_default(),
            ev_max_current: Decimal::try_from(ev_max_current).unwrap_or_default(),
            ev_max_voltage: Decimal::try_from(ev_max_voltage).unwrap_or_default(),
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the energy amount.
    ///
    /// # Returns
    ///
    /// The amount of energy requested (in Wh)
    pub fn energy_amount(&self) -> &Decimal {
        &self.energy_amount
    }

    /// Sets the energy amount.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_amount(&mut self, energy_amount: Decimal) -> &mut Self {
        self.energy_amount = energy_amount;
        self
    }

    /// Sets the energy amount from f64.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_amount_f64(&mut self, energy_amount: f64) -> &mut Self {
        self.energy_amount = Decimal::try_from(energy_amount).unwrap_or_default();
        self
    }

    /// Gets the minimum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The minimum current (amps) supported by the electric vehicle (per phase)
    pub fn ev_min_current(&self) -> &Decimal {
        &self.ev_min_current
    }

    /// Sets the minimum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_min_current(&mut self, ev_min_current: Decimal) -> &mut Self {
        self.ev_min_current = ev_min_current;
        self
    }

    /// Sets the minimum current supported by the electric vehicle from f64.
    ///
    /// # Arguments
    ///
    /// * `ev_min_current` - Minimum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_min_current_f64(&mut self, ev_min_current: f64) -> &mut Self {
        self.ev_min_current = Decimal::try_from(ev_min_current).unwrap_or_default();
        self
    }

    /// Gets the maximum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum current (amps) supported by the electric vehicle (per phase)
    pub fn ev_max_current(&self) -> &Decimal {
        &self.ev_max_current
    }

    /// Sets the maximum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_current(&mut self, ev_max_current: Decimal) -> &mut Self {
        self.ev_max_current = ev_max_current;
        self
    }

    /// Sets the maximum current supported by the electric vehicle from f64.
    ///
    /// # Arguments
    ///
    /// * `ev_max_current` - Maximum current (amps) supported by the electric vehicle (per phase)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_current_f64(&mut self, ev_max_current: f64) -> &mut Self {
        self.ev_max_current = Decimal::try_from(ev_max_current).unwrap_or_default();
        self
    }

    /// Gets the maximum voltage supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum voltage supported by the electric vehicle
    pub fn ev_max_voltage(&self) -> &Decimal {
        &self.ev_max_voltage
    }

    /// Sets the maximum voltage supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_voltage(&mut self, ev_max_voltage: Decimal) -> &mut Self {
        self.ev_max_voltage = ev_max_voltage;
        self
    }

    /// Sets the maximum voltage supported by the electric vehicle from f64.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_voltage_f64(&mut self, ev_max_voltage: f64) -> &mut Self {
        self.ev_max_voltage = Decimal::try_from(ev_max_voltage).unwrap_or_default();
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
    /// * `custom_data` - Custom data for these charging parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_ac_charging_parameters() {
        let params = ACChargingParametersType::new(
            dec!(10000.0), // energy_amount
            dec!(10.0),    // ev_min_current
            dec!(32.0),    // ev_max_current
            dec!(400.0),   // ev_max_voltage
        );

        assert_eq!(params.energy_amount(), &dec!(10000.0));
        assert_eq!(params.ev_min_current(), &dec!(10.0));
        assert_eq!(params.ev_max_current(), &dec!(32.0));
        assert_eq!(params.ev_max_voltage(), &dec!(400.0));
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_new_from_f64_ac_charging_parameters() {
        let params = ACChargingParametersType::new_from_f64(
            10000.0, // energy_amount
            10.0,    // ev_min_current
            32.0,    // ev_max_current
            400.0,   // ev_max_voltage
        );

        assert_eq!(params.energy_amount(), &Decimal::try_from(10000.0).unwrap());
        assert_eq!(params.ev_min_current(), &Decimal::try_from(10.0).unwrap());
        assert_eq!(params.ev_max_current(), &Decimal::try_from(32.0).unwrap());
        assert_eq!(params.ev_max_voltage(), &Decimal::try_from(400.0).unwrap());
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = ACChargingParametersType::new(
            dec!(10000.0), // energy_amount
            dec!(10.0),    // ev_min_current
            dec!(32.0),    // ev_max_current
            dec!(400.0),   // ev_max_voltage
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(params.energy_amount(), &dec!(10000.0));
        assert_eq!(params.ev_min_current(), &dec!(10.0));
        assert_eq!(params.ev_max_current(), &dec!(32.0));
        assert_eq!(params.ev_max_voltage(), &dec!(400.0));
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = ACChargingParametersType::new(
            dec!(10000.0), // energy_amount
            dec!(10.0),    // ev_min_current
            dec!(32.0),    // ev_max_current
            dec!(400.0),   // ev_max_voltage
        );

        params
            .set_energy_amount(dec!(15000.0))
            .set_ev_min_current(dec!(15.0))
            .set_ev_max_current(dec!(40.0))
            .set_ev_max_voltage(dec!(415.0))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.energy_amount(), &dec!(15000.0));
        assert_eq!(params.ev_min_current(), &dec!(15.0));
        assert_eq!(params.ev_max_current(), &dec!(40.0));
        assert_eq!(params.ev_max_voltage(), &dec!(415.0));
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_setter_f64_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = ACChargingParametersType::new_from_f64(
            10000.0, // energy_amount
            10.0,    // ev_min_current
            32.0,    // ev_max_current
            400.0,   // ev_max_voltage
        );

        params
            .set_energy_amount_f64(15000.0)
            .set_ev_min_current_f64(15.0)
            .set_ev_max_current_f64(40.0)
            .set_ev_max_voltage_f64(415.0)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.energy_amount(), &Decimal::try_from(15000.0).unwrap());
        assert_eq!(params.ev_min_current(), &Decimal::try_from(15.0).unwrap());
        assert_eq!(params.ev_max_current(), &Decimal::try_from(40.0).unwrap());
        assert_eq!(params.ev_max_voltage(), &Decimal::try_from(415.0).unwrap());
        assert_eq!(params.custom_data(), Some(&custom_data));
    }
}
