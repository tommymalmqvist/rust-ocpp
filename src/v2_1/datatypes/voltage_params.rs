use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::power_during_cessation::PowerDuringCessationEnumType;

/// Parameters for voltage-based control.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VoltageParamsType {
    /// EN 50549-1 chapter 4.9.3.4
    /// Voltage threshold for the 10 min time window mean value monitoring.
    /// The 10 min mean is recalculated up to every 3 s.
    /// If the present voltage is above this threshold for more than the time defined by _hv10MinMeanValue_, the EV must trip.
    /// This value is mandatory if _hv10MinMeanTripDelay_ is set.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hv10_min_mean_value: Option<Decimal>,

    /// Time for which the voltage is allowed to stay above the 10 min mean value.
    /// After this time, the EV must trip.
    /// This value is mandatory if OverVoltageMeanValue10min is set.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hv10_min_mean_trip_delay: Option<Decimal>,

    /// Power behavior during cessation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_during_cessation: Option<PowerDuringCessationEnumType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VoltageParamsType {
    /// Creates a new `VoltageParamsType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `VoltageParamsType` with all fields set to `None`
    pub fn new() -> Self {
        Self {
            hv10_min_mean_value: None,
            hv10_min_mean_trip_delay: None,
            power_during_cessation: None,
            custom_data: None,
        }
    }

    /// Sets the HV 10 min mean value.
    ///
    /// # Arguments
    ///
    /// * `hv10_min_mean_value` - Voltage threshold for the 10 min time window mean value monitoring
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hv10_min_mean_value(mut self, hv10_min_mean_value: Decimal) -> Self {
        self.hv10_min_mean_value = Some(hv10_min_mean_value);
        self
    }

    /// Sets the HV 10 min mean trip delay.
    ///
    /// # Arguments
    ///
    /// * `hv10_min_mean_trip_delay` - Time for which the voltage is allowed to stay above the 10 min mean value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_hv10_min_mean_trip_delay(mut self, hv10_min_mean_trip_delay: Decimal) -> Self {
        self.hv10_min_mean_trip_delay = Some(hv10_min_mean_trip_delay);
        self
    }

    /// Sets the power during cessation.
    ///
    /// # Arguments
    ///
    /// * `power_during_cessation` - Power behavior during cessation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_power_during_cessation(
        mut self,
        power_during_cessation: PowerDuringCessationEnumType,
    ) -> Self {
        self.power_during_cessation = Some(power_during_cessation);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these voltage parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the HV 10 min mean value.
    ///
    /// # Returns
    ///
    /// The voltage threshold for the 10 min time window mean value monitoring
    pub fn hv10_min_mean_value(&self) -> Option<&Decimal> {
        self.hv10_min_mean_value.as_ref()
    }

    /// Sets the HV 10 min mean value.
    ///
    /// # Arguments
    ///
    /// * `hv10_min_mean_value` - Voltage threshold for the 10 min time window mean value monitoring, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hv10_min_mean_value(&mut self, hv10_min_mean_value: Option<Decimal>) -> &mut Self {
        self.hv10_min_mean_value = hv10_min_mean_value;
        self
    }

    /// Gets the HV 10 min mean trip delay.
    ///
    /// # Returns
    ///
    /// The time for which the voltage is allowed to stay above the 10 min mean value
    pub fn hv10_min_mean_trip_delay(&self) -> Option<&Decimal> {
        self.hv10_min_mean_trip_delay.as_ref()
    }

    /// Sets the HV 10 min mean trip delay.
    ///
    /// # Arguments
    ///
    /// * `hv10_min_mean_trip_delay` - Time for which the voltage is allowed to stay above the 10 min mean value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hv10_min_mean_trip_delay(
        &mut self,
        hv10_min_mean_trip_delay: Option<Decimal>,
    ) -> &mut Self {
        self.hv10_min_mean_trip_delay = hv10_min_mean_trip_delay;
        self
    }

    /// Gets the power during cessation.
    ///
    /// # Returns
    ///
    /// The power behavior during cessation
    pub fn power_during_cessation(&self) -> Option<&PowerDuringCessationEnumType> {
        self.power_during_cessation.as_ref()
    }

    /// Sets the power during cessation.
    ///
    /// # Arguments
    ///
    /// * `power_during_cessation` - Power behavior during cessation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_during_cessation(
        &mut self,
        power_during_cessation: Option<PowerDuringCessationEnumType>,
    ) -> &mut Self {
        self.power_during_cessation = power_during_cessation;
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
    /// * `custom_data` - Custom data for these voltage parameters, or None to clear
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
    fn test_new_voltage_params() {
        let params = VoltageParamsType::new();

        assert_eq!(params.hv10_min_mean_value(), None);
        assert_eq!(params.hv10_min_mean_trip_delay(), None);
        assert_eq!(params.power_during_cessation(), None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let hv10_min_mean_value = dec!(253.0);
        let hv10_min_mean_trip_delay = dec!(3.0);
        let power_during_cessation = PowerDuringCessationEnumType::Active;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params = VoltageParamsType::new()
            .with_hv10_min_mean_value(hv10_min_mean_value)
            .with_hv10_min_mean_trip_delay(hv10_min_mean_trip_delay)
            .with_power_during_cessation(power_during_cessation.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(params.hv10_min_mean_value(), Some(&hv10_min_mean_value));
        assert_eq!(
            params.hv10_min_mean_trip_delay(),
            Some(&hv10_min_mean_trip_delay)
        );
        assert_eq!(
            params.power_during_cessation(),
            Some(&power_during_cessation)
        );
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let hv10_min_mean_value1 = dec!(253.0);
        let hv10_min_mean_trip_delay1 = dec!(3.0);
        let power_during_cessation1 = PowerDuringCessationEnumType::Active;

        let hv10_min_mean_value2 = dec!(260.0);
        let hv10_min_mean_trip_delay2 = dec!(5.0);
        let power_during_cessation2 = PowerDuringCessationEnumType::Reactive;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = VoltageParamsType::new()
            .with_hv10_min_mean_value(hv10_min_mean_value1)
            .with_hv10_min_mean_trip_delay(hv10_min_mean_trip_delay1)
            .with_power_during_cessation(power_during_cessation1);

        params
            .set_hv10_min_mean_value(Some(hv10_min_mean_value2))
            .set_hv10_min_mean_trip_delay(Some(hv10_min_mean_trip_delay2))
            .set_power_during_cessation(Some(power_during_cessation2.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.hv10_min_mean_value(), Some(&hv10_min_mean_value2));
        assert_eq!(
            params.hv10_min_mean_trip_delay(),
            Some(&hv10_min_mean_trip_delay2)
        );
        assert_eq!(
            params.power_during_cessation(),
            Some(&power_during_cessation2)
        );
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params
            .set_hv10_min_mean_value(None)
            .set_hv10_min_mean_trip_delay(None)
            .set_power_during_cessation(None)
            .set_custom_data(None);

        assert_eq!(params.hv10_min_mean_value(), None);
        assert_eq!(params.hv10_min_mean_trip_delay(), None);
        assert_eq!(params.power_during_cessation(), None);
        assert_eq!(params.custom_data(), None);
    }
}
