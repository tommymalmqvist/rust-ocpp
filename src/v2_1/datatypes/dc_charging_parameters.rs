use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::datatypes::CustomDataType;

/// EV DC charging parameters for ISO 15118-2
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DCChargingParametersType {
    /// Maximum current (in A) supported by the electric vehicle. Includes cable capacity.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType:EVMaximumCurrentLimit
    pub ev_max_current: Decimal,

    /// Maximum voltage supported by the electric vehicle.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumVoltageLimit
    pub ev_max_voltage: Decimal,

    /// Maximum power (in W) supported by the electric vehicle. Required for DC charging.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVMaximumPowerLimit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_max_power: Option<Decimal>,

    /// Capacity of the electric vehicle battery (in Wh).
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyCapacity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_capacity: Option<Decimal>,

    /// Amount of energy requested (in Wh). This includes energy required for preconditioning.
    /// Relates to:
    /// *ISO 15118-2*: DC_EVChargeParameterType: EVEnergyRequest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_amount: Option<Decimal>,

    /// Energy available in the battery (in percent of the
    /// battery capacity) Relates to:
    /// ISO 15118-2: DC_EVChargeParameterType:
    /// DC_EVStatus: EVRESSSOC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub state_of_charge: Option<i32>,

    /// Percentage of SoC at which the EV considers
    /// the battery fully charged. (possible values: 0 - 100)
    /// Relates to:
    /// ISO 15118-2: DC_EVChargeParameterType: FullSOC
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub full_so_c: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DCChargingParametersType {
    /// Creates a new `DCChargingParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_max_voltage` - Maximum voltage supported by the electric vehicle
    /// * `ev_max_current` - Maximum current (in A) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// A new instance of `DCChargingParametersType` with optional fields set to `None`
    pub fn new(ev_max_voltage: Decimal, ev_max_current: Decimal) -> Self {
        Self {
            ev_max_voltage,
            ev_max_current,
            ev_max_power: None,
            ev_energy_capacity: None,
            energy_amount: None,
            state_of_charge: None,
            full_so_c: None,
            custom_data: None,
        }
    }

    /// Sets the maximum power supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_power` - Maximum power (in W) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_power(mut self, ev_max_power: Decimal) -> Self {
        self.ev_max_power = Some(ev_max_power);
        self
    }

    /// Sets the capacity of the electric vehicle battery.
    ///
    /// # Arguments
    ///
    /// * `ev_energy_capacity` - Capacity of the electric vehicle battery (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_capacity(mut self, ev_energy_capacity: Decimal) -> Self {
        self.ev_energy_capacity = Some(ev_energy_capacity);
        self
    }

    /// Sets the amount of energy requested.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy_amount(mut self, energy_amount: Decimal) -> Self {
        self.energy_amount = Some(energy_amount);
        self
    }

    /// Sets the state of charge.
    ///
    /// # Arguments
    ///
    /// * `state_of_charge` - State of charge in percent (0-100)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_state_of_charge(mut self, state_of_charge: i32) -> Self {
        self.state_of_charge = Some(state_of_charge);
        self
    }

    /// Sets the full state of charge.
    ///
    /// # Arguments
    ///
    /// * `full_so_c` - Percentage of SoC at which the EV considers the battery fully charged (0-100)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_full_so_c(mut self, full_so_c: i32) -> Self {
        self.full_so_c = Some(full_so_c);
        self
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

    /// Gets the maximum voltage supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum voltage supported by the electric vehicle
    pub fn ev_max_voltage(&self) -> Decimal {
        self.ev_max_voltage
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

    /// Gets the maximum current supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// The maximum current (in A) supported by the electric vehicle
    pub fn ev_max_current(&self) -> Decimal {
        self.ev_max_current
    }

    /// Sets the maximum current supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_current` - Maximum current (in A) supported by the electric vehicle
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_current(&mut self, ev_max_current: Decimal) -> &mut Self {
        self.ev_max_current = ev_max_current;
        self
    }

    /// Gets the maximum power supported by the electric vehicle.
    ///
    /// # Returns
    ///
    /// An optional value representing the maximum power (in W) supported by the electric vehicle
    pub fn ev_max_power(&self) -> Option<Decimal> {
        self.ev_max_power
    }

    /// Sets the maximum power supported by the electric vehicle.
    ///
    /// # Arguments
    ///
    /// * `ev_max_power` - Maximum power (in W) supported by the electric vehicle, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_max_power(&mut self, ev_max_power: Option<Decimal>) -> &mut Self {
        self.ev_max_power = ev_max_power;
        self
    }

    /// Gets the capacity of the electric vehicle battery.
    ///
    /// # Returns
    ///
    /// An optional value representing the capacity of the electric vehicle battery (in Wh)
    pub fn ev_energy_capacity(&self) -> Option<Decimal> {
        self.ev_energy_capacity
    }

    /// Sets the capacity of the electric vehicle battery.
    ///
    /// # Arguments
    ///
    /// * `ev_energy_capacity` - Capacity of the electric vehicle battery (in Wh), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_energy_capacity(&mut self, ev_energy_capacity: Option<Decimal>) -> &mut Self {
        self.ev_energy_capacity = ev_energy_capacity;
        self
    }

    /// Gets the amount of energy requested.
    ///
    /// # Returns
    ///
    /// An optional value representing the amount of energy requested (in Wh)
    pub fn energy_amount(&self) -> Option<Decimal> {
        self.energy_amount
    }

    /// Sets the amount of energy requested.
    ///
    /// # Arguments
    ///
    /// * `energy_amount` - Amount of energy requested (in Wh), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_amount(&mut self, energy_amount: Option<Decimal>) -> &mut Self {
        self.energy_amount = energy_amount;
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

    /// Gets the state of charge.
    ///
    /// # Returns
    ///
    /// An optional value representing the state of charge (in percent)
    pub fn state_of_charge(&self) -> Option<i32> {
        self.state_of_charge
    }

    /// Sets the state of charge.
    ///
    /// # Arguments
    ///
    /// * `state_of_charge` - State of charge in percent (0-100), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_state_of_charge(&mut self, state_of_charge: Option<i32>) -> &mut Self {
        self.state_of_charge = state_of_charge;
        self
    }

    /// Gets the full state of charge.
    ///
    /// # Returns
    ///
    /// An optional value representing the percentage of SoC at which the EV considers
    /// the battery fully charged (0-100)
    pub fn full_so_c(&self) -> Option<i32> {
        self.full_so_c
    }

    /// Sets the full state of charge.
    ///
    /// # Arguments
    ///
    /// * `full_so_c` - Percentage of SoC at which the EV considers the battery fully charged (0-100),
    ///                 or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_full_so_c(&mut self, full_so_c: Option<i32>) -> &mut Self {
        self.full_so_c = full_so_c;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_new_dc_charging_parameters() {
        let params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125));

        assert_eq!(params.ev_max_voltage(), Decimal::from(500));
        assert_eq!(params.ev_max_current(), Decimal::from(125));
        assert_eq!(params.ev_max_power(), None);
        assert_eq!(params.ev_energy_capacity(), None);
        assert_eq!(params.energy_amount(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125))
            .with_max_power(Decimal::from(50000))
            .with_energy_capacity(Decimal::from(75000))
            .with_energy_amount(Decimal::from(20000))
            .with_state_of_charge(80)
            .with_full_so_c(95)
            .with_custom_data(custom_data.clone());

        assert_eq!(params.ev_max_voltage(), Decimal::from(500));
        assert_eq!(params.ev_max_current(), Decimal::from(125));
        assert_eq!(params.ev_max_power(), Some(Decimal::from(50000)));
        assert_eq!(params.ev_energy_capacity(), Some(Decimal::from(75000)));
        assert_eq!(params.energy_amount(), Some(Decimal::from(20000)));
        assert_eq!(params.state_of_charge(), Some(80));
        assert_eq!(params.full_so_c(), Some(95));
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125));

        params
            .set_ev_max_voltage(Decimal::from(550))
            .set_ev_max_current(Decimal::from(150))
            .set_ev_max_power(Some(Decimal::from(60000)))
            .set_ev_energy_capacity(Some(Decimal::from(80000)))
            .set_energy_amount(Some(Decimal::from(25000)))
            .set_state_of_charge(Some(70))
            .set_full_so_c(Some(90))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.ev_max_voltage(), Decimal::from(550));
        assert_eq!(params.ev_max_current(), Decimal::from(150));
        assert_eq!(params.ev_max_power(), Some(Decimal::from(60000)));
        assert_eq!(params.ev_energy_capacity(), Some(Decimal::from(80000)));
        assert_eq!(params.energy_amount(), Some(Decimal::from(25000)));
        assert_eq!(params.state_of_charge(), Some(70));
        assert_eq!(params.full_so_c(), Some(90));
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params
            .set_ev_max_power(None)
            .set_ev_energy_capacity(None)
            .set_energy_amount(None)
            .set_state_of_charge(None)
            .set_full_so_c(None)
            .set_custom_data(None);

        assert_eq!(params.ev_max_power(), None);
        assert_eq!(params.ev_energy_capacity(), None);
        assert_eq!(params.energy_amount(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_basic_validation() {
        // Valid parameters - should pass validation
        let params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125));
        assert!(
            params.validate().is_ok(),
            "Valid DC charging parameters should pass validation"
        );

        // Add optional fields - should still pass validation
        let params_with_optionals =
            DCChargingParametersType::new(Decimal::from(500), Decimal::from(125))
                .with_max_power(Decimal::from(50000))
                .with_energy_capacity(Decimal::from(75000))
                .with_energy_amount(Decimal::from(20000));
        assert!(
            params_with_optionals.validate().is_ok(),
            "DC charging parameters with optional fields should pass validation"
        );
    }

    #[test]
    fn test_state_of_charge_validation() {
        let mut params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125));

        // Valid state of charge values (0-100)
        params.state_of_charge = Some(0);
        assert!(
            params.validate().is_ok(),
            "State of charge = 0 should be valid"
        );

        params.state_of_charge = Some(50);
        assert!(
            params.validate().is_ok(),
            "State of charge = 50 should be valid"
        );

        params.state_of_charge = Some(100);
        assert!(
            params.validate().is_ok(),
            "State of charge = 100 should be valid"
        );

        // Invalid state of charge values (outside 0-100 range)
        params.state_of_charge = Some(-1);
        assert!(
            params.validate().is_err(),
            "State of charge = -1 should be invalid"
        );
        let error = params.validate().unwrap_err();
        assert!(
            error.to_string().contains("state_of_charge"),
            "Error should mention state_of_charge: {}",
            error
        );

        params.state_of_charge = Some(101);
        assert!(
            params.validate().is_err(),
            "State of charge = 101 should be invalid"
        );
        let error = params.validate().unwrap_err();
        assert!(
            error.to_string().contains("state_of_charge"),
            "Error should mention state_of_charge: {}",
            error
        );
    }

    #[test]
    fn test_full_soc_validation() {
        let mut params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125));

        // Valid full_so_c values (0-100)
        params.full_so_c = Some(0);
        assert!(params.validate().is_ok(), "full_so_c = 0 should be valid");

        params.full_so_c = Some(80);
        assert!(params.validate().is_ok(), "full_so_c = 80 should be valid");

        params.full_so_c = Some(100);
        assert!(params.validate().is_ok(), "full_so_c = 100 should be valid");

        // Invalid full_so_c values (outside 0-100 range)
        params.full_so_c = Some(-1);
        assert!(
            params.validate().is_err(),
            "full_so_c = -1 should be invalid"
        );
        let error = params.validate().unwrap_err();
        assert!(
            error.to_string().contains("full_so_c"),
            "Error should mention full_so_c: {}",
            error
        );

        params.full_so_c = Some(101);
        assert!(
            params.validate().is_err(),
            "full_so_c = 101 should be invalid"
        );
        let error = params.validate().unwrap_err();
        assert!(
            error.to_string().contains("full_so_c"),
            "Error should mention full_so_c: {}",
            error
        );
    }

    #[test]
    fn test_custom_data_validation() {
        // Create custom data with invalid vendor_id (too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125))
            .with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = params.validate();
        assert!(
            validation_result.is_err(),
            "Invalid custom_data should cause validation failure"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }

    #[test]
    fn test_combined_validation() {
        // Test with multiple validation issues
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let mut params = DCChargingParametersType::new(Decimal::from(500), Decimal::from(125))
            .with_custom_data(invalid_custom_data);

        // Add invalid state_of_charge and full_so_c
        params.state_of_charge = Some(101);
        params.full_so_c = Some(101);

        // Validation should fail
        let validation_result = params.validate();
        assert!(
            validation_result.is_err(),
            "Multiple invalid fields should cause validation failure"
        );

        // Error message should contain all validation failures
        let error = validation_result.unwrap_err().to_string();
        assert!(
            error.contains("custom_data")
                || error.contains("state_of_charge")
                || error.contains("full_so_c"),
            "Error should mention at least one invalid field: {}",
            error
        );
    }
}
