use super::custom_data::CustomDataType;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Entry in the EVPowerSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPowerScheduleEntryType {
    /// Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Power in Watts (W).
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub power: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EVPowerScheduleEntryType {
    /// Creates a new `EVPowerScheduleEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `power` - Power in Watts (W)
    ///
    /// # Returns
    ///
    /// A new instance of `EVPowerScheduleEntryType` with optional fields set to `None`
    pub fn new(duration: i32, power: f64) -> Self {
        Self {
            duration,
            power: Decimal::from_f64(power).unwrap_or_default(),
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this schedule entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// The duration of the schedule entry in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the power.
    ///
    /// # Returns
    ///
    /// The power in Watts (W) as a Decimal
    pub fn power(&self) -> &Decimal {
        &self.power
    }

    /// Gets the power as f64.
    ///
    /// # Returns
    ///
    /// The power in Watts (W) as an f64, or 0.0 if conversion fails
    pub fn power_as_f64(&self) -> f64 {
        self.power.to_f64().unwrap_or(0.0)
    }

    /// Sets the power.
    ///
    /// # Arguments
    ///
    /// * `power` - Power in Watts (W)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power(&mut self, power: f64) -> &mut Self {
        self.power = Decimal::from_f64(power).unwrap_or_default();
        self
    }

    /// Sets the power from a Decimal.
    ///
    /// # Arguments
    ///
    /// * `power` - Power in Watts (W) as a Decimal
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_decimal(&mut self, power: Decimal) -> &mut Self {
        self.power = power;
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
    /// * `custom_data` - Custom data for this schedule entry, or None to clear
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
    fn test_new_ev_power_schedule_entry() {
        let duration = 3600;
        let power = 11000.0;
        let expected_decimal = Decimal::from_f64(power).unwrap();

        let entry = EVPowerScheduleEntryType::new(duration, power);

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.power(), &expected_decimal);
        assert_eq!(entry.power_as_f64(), power);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let power = 11000.0;
        let expected_decimal = Decimal::from_f64(power).unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let entry =
            EVPowerScheduleEntryType::new(duration, power).with_custom_data(custom_data.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.power(), &expected_decimal);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let power1 = 11000.0;
        let duration2 = 7200;
        let power2 = 7500.0;
        let expected_decimal2 = Decimal::from_f64(power2).unwrap();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut entry = EVPowerScheduleEntryType::new(duration1, power1);

        entry
            .set_duration(duration2)
            .set_power(power2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.duration(), duration2);
        assert_eq!(entry.power(), &expected_decimal2);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry.set_custom_data(None);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_set_power_decimal() {
        let duration = 3600;
        let power_f64 = 11000.0;
        let power_decimal = dec!(12345.6789);

        let mut entry = EVPowerScheduleEntryType::new(duration, power_f64);
        entry.set_power_decimal(power_decimal);

        assert_eq!(entry.power(), &power_decimal);
    }

    #[test]
    fn test_decimal_precision() {
        // Test with a high precision decimal value
        let duration = 3600;
        let power_decimal = dec!(12345.6789012345);

        let mut entry = EVPowerScheduleEntryType::new(duration, 0.0);
        entry.set_power_decimal(power_decimal);

        assert_eq!(entry.power(), &power_decimal);

        // Verify precision is maintained through serialization/deserialization
        let serialized = serde_json::to_string(&entry).unwrap();
        let deserialized: EVPowerScheduleEntryType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.power(), &power_decimal);
    }
}
