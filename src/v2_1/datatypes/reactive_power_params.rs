use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Parameters for reactive power control.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReactivePowerParamsType {
    /// Only for VoltVar curve: The nominal ac voltage (rms) adjustment to the voltage curve points for Volt-Var curves (percentage).
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub v_ref: Option<Decimal>,

    /// Only for VoltVar: Enable/disable autonomous VRef adjustment
    pub autonomous_vref_enable: Option<bool>,

    /// Only for VoltVar: Adjustment range for VRef time constant
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub autonomous_vref_time_constant: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReactivePowerParamsType {
    /// Creates a new `ReactivePowerParamsType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `ReactivePowerParamsType` with all optional fields set to `None`
    pub fn new() -> Self {
        Self {
            v_ref: None,
            autonomous_vref_enable: None,
            autonomous_vref_time_constant: None,
            custom_data: None,
        }
    }

    /// Sets the VRef value.
    ///
    /// # Arguments
    ///
    /// * `v_ref` - The nominal ac voltage (rms) adjustment for Volt-Var curves
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_v_ref(mut self, v_ref: Decimal) -> Self {
        self.v_ref = Some(v_ref);
        self
    }

    /// Sets the autonomous VRef enable flag.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable/disable autonomous VRef adjustment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_autonomous_v_ref_enable(mut self, enable: bool) -> Self {
        self.autonomous_vref_enable = Some(enable);
        self
    }

    /// Sets the autonomous VRef time constant.
    ///
    /// # Arguments
    ///
    /// * `time_constant` - Adjustment range for VRef time constant
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_autonomous_v_ref_time_constant(mut self, time_constant: Decimal) -> Self {
        self.autonomous_vref_time_constant = Some(time_constant);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these reactive power parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the VRef value.
    ///
    /// # Returns
    ///
    /// An optional reference to the VRef value
    pub fn v_ref(&self) -> Option<&Decimal> {
        self.v_ref.as_ref()
    }

    /// Sets the VRef value.
    ///
    /// # Arguments
    ///
    /// * `v_ref` - The nominal ac voltage (rms) adjustment for Volt-Var curves, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_v_ref(&mut self, v_ref: Option<Decimal>) -> &mut Self {
        self.v_ref = v_ref;
        self
    }

    /// Gets the autonomous VRef enable flag.
    ///
    /// # Returns
    ///
    /// An optional reference to the autonomous VRef enable flag
    pub fn autonomous_v_ref_enable(&self) -> Option<&bool> {
        self.autonomous_vref_enable.as_ref()
    }

    /// Sets the autonomous VRef enable flag.
    ///
    /// # Arguments
    ///
    /// * `enable` - Enable/disable autonomous VRef adjustment, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_autonomous_v_ref_enable(&mut self, enable: Option<bool>) -> &mut Self {
        self.autonomous_vref_enable = enable;
        self
    }

    /// Gets the autonomous VRef time constant.
    ///
    /// # Returns
    ///
    /// An optional reference to the autonomous VRef time constant
    pub fn autonomous_v_ref_time_constant(&self) -> Option<&Decimal> {
        self.autonomous_vref_time_constant.as_ref()
    }

    /// Sets the autonomous VRef time constant.
    ///
    /// # Arguments
    ///
    /// * `time_constant` - Adjustment range for VRef time constant, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_autonomous_v_ref_time_constant(
        &mut self,
        time_constant: Option<Decimal>,
    ) -> &mut Self {
        self.autonomous_vref_time_constant = time_constant;
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
    /// * `custom_data` - Custom data for these reactive power parameters, or None to clear
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
    use rust_decimal::Decimal;

    #[test]
    fn test_new_reactive_power_params() {
        let params = ReactivePowerParamsType::new();

        assert_eq!(params.v_ref(), None);
        assert_eq!(params.autonomous_v_ref_enable(), None);
        assert_eq!(params.autonomous_v_ref_time_constant(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let v_ref = Decimal::new(500, 1); // 50.0
        let time_constant = Decimal::new(100, 1); // 10.0
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = ReactivePowerParamsType::new()
            .with_v_ref(v_ref.clone())
            .with_autonomous_v_ref_enable(true)
            .with_autonomous_v_ref_time_constant(time_constant.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(params.v_ref(), Some(&v_ref));
        assert_eq!(params.autonomous_v_ref_enable(), Some(&true));
        assert_eq!(
            params.autonomous_v_ref_time_constant(),
            Some(&time_constant)
        );
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let v_ref = Decimal::new(500, 1); // 50.0
        let time_constant = Decimal::new(100, 1); // 10.0
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = ReactivePowerParamsType::new();

        params
            .set_v_ref(Some(v_ref.clone()))
            .set_autonomous_v_ref_enable(Some(true))
            .set_autonomous_v_ref_time_constant(Some(time_constant.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.v_ref(), Some(&v_ref));
        assert_eq!(params.autonomous_v_ref_enable(), Some(&true));
        assert_eq!(
            params.autonomous_v_ref_time_constant(),
            Some(&time_constant)
        );
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_v_ref(None);
        params.set_autonomous_v_ref_enable(None);
        params.set_autonomous_v_ref_time_constant(None);
        params.set_custom_data(None);

        assert_eq!(params.v_ref(), None);
        assert_eq!(params.autonomous_v_ref_enable(), None);
        assert_eq!(params.autonomous_v_ref_time_constant(), None);
        assert_eq!(params.custom_data(), None);
    }
}
