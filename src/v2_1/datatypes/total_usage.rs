use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// This contains the calculated usage of energy, charging time and idle time during a transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalUsageType {
    /// Energy usage in kWh.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub energy: Decimal,

    /// Total duration of the charging session (including the duration of charging and not charging), in seconds.
    pub charging_time: i32,

    /// Total duration of the charging session where the EV was not charging (no energy was transferred between EVSE and EV), in seconds.
    pub idle_time: i32,

    /// Total time of reservation in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_time: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TotalUsageType {
    /// Creates a new `TotalUsageType` with the required fields.
    pub fn new(energy: Decimal, charging_time: i32, idle_time: i32) -> Self {
        Self {
            energy,
            charging_time,
            idle_time,
            reservation_time: None,
            custom_data: None,
        }
    }

    /// Creates a new `TotalUsageType` from a floating-point energy value.
    pub fn new_from_f64(energy: f64, charging_time: i32, idle_time: i32) -> Self {
        Self {
            energy: Decimal::from_f64(energy).unwrap_or_else(|| Decimal::new(0, 0)),
            charging_time,
            idle_time,
            reservation_time: None,
            custom_data: None,
        }
    }

    /// Gets the energy value.
    pub fn energy(&self) -> Decimal {
        self.energy
    }

    /// Sets the energy value.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_energy(&mut self, energy: Decimal) -> &mut Self {
        self.energy = energy;
        self
    }

    /// Gets the charging time value in seconds.
    pub fn charging_time(&self) -> i32 {
        self.charging_time
    }

    /// Sets the charging time value in seconds.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_charging_time(&mut self, charging_time: i32) -> &mut Self {
        self.charging_time = charging_time;
        self
    }

    /// Gets the idle time value in seconds.
    pub fn idle_time(&self) -> i32 {
        self.idle_time
    }

    /// Sets the idle time value in seconds.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_idle_time(&mut self, idle_time: i32) -> &mut Self {
        self.idle_time = idle_time;
        self
    }

    /// Gets the reservation time value in seconds.
    pub fn reservation_time(&self) -> Option<i32> {
        self.reservation_time
    }

    /// Sets the reservation time value in seconds.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_reservation_time(&mut self, reservation_time: Option<i32>) -> &mut Self {
        self.reservation_time = reservation_time;
        self
    }

    /// Gets a reference to the custom data, if present.
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the reservation time value using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_reservation_time(mut self, reservation_time: i32) -> Self {
        self.reservation_time = Some(reservation_time);
        self
    }

    /// Sets the custom data using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_total_usage() {
        let energy = Decimal::from_f64(10.5).unwrap();
        let charging_time = 3600;
        let idle_time = 600;

        let total_usage = TotalUsageType::new(energy, charging_time, idle_time);

        assert_eq!(total_usage.energy(), energy);
        assert_eq!(total_usage.charging_time(), charging_time);
        assert_eq!(total_usage.idle_time(), idle_time);
        assert_eq!(total_usage.reservation_time(), None);
        assert_eq!(total_usage.custom_data(), None);
    }

    #[test]
    fn test_new_from_f64() {
        let energy_f64 = 10.5;
        let energy_decimal = Decimal::from_f64(energy_f64).unwrap();
        let charging_time = 3600;
        let idle_time = 600;

        let total_usage = TotalUsageType::new_from_f64(energy_f64, charging_time, idle_time);

        assert_eq!(total_usage.energy(), energy_decimal);
        assert_eq!(total_usage.charging_time(), charging_time);
        assert_eq!(total_usage.idle_time(), idle_time);
        assert_eq!(total_usage.reservation_time(), None);
        assert_eq!(total_usage.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let energy = Decimal::from_f64(10.5).unwrap();
        let charging_time = 3600;
        let idle_time = 600;
        let reservation_time = 1800;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let total_usage = TotalUsageType::new(energy, charging_time, idle_time)
            .with_reservation_time(reservation_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(total_usage.energy(), energy);
        assert_eq!(total_usage.charging_time(), charging_time);
        assert_eq!(total_usage.idle_time(), idle_time);
        assert_eq!(total_usage.reservation_time(), Some(reservation_time));
        assert_eq!(total_usage.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy = Decimal::from_f64(10.5).unwrap();
        let charging_time = 3600;
        let idle_time = 600;
        let new_energy = Decimal::from_f64(15.0).unwrap();
        let new_charging_time = 4800;
        let new_idle_time = 900;
        let reservation_time = 1800;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut total_usage = TotalUsageType::new(energy, charging_time, idle_time);

        total_usage
            .set_energy(new_energy)
            .set_charging_time(new_charging_time)
            .set_idle_time(new_idle_time)
            .set_reservation_time(Some(reservation_time))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(total_usage.energy(), new_energy);
        assert_eq!(total_usage.charging_time(), new_charging_time);
        assert_eq!(total_usage.idle_time(), new_idle_time);
        assert_eq!(total_usage.reservation_time(), Some(reservation_time));
        assert_eq!(total_usage.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        total_usage.set_reservation_time(None).set_custom_data(None);

        assert_eq!(total_usage.reservation_time(), None);
        assert_eq!(total_usage.custom_data(), None);
    }

    #[test]
    fn test_serde() {
        let energy = Decimal::from_f64(10.5).unwrap();
        let charging_time = 3600;
        let idle_time = 600;
        let reservation_time = 1800;

        let total_usage = TotalUsageType::new(energy, charging_time, idle_time)
            .with_reservation_time(reservation_time);

        let json = serde_json::to_string(&total_usage).unwrap();
        let deserialized: TotalUsageType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, total_usage);
    }
}
