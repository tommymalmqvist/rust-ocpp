use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Entry in the PriceLevelSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleEntryType {
    /// Required. Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Required. Relative price level of this schedule entry.
    /// Small values represent a cheaper price level, large values represent a more expensive price level.
    #[validate(range(min = 0, max = 9))]
    pub price_level: i8,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PriceLevelScheduleEntryType {
    /// Creates a new `PriceLevelScheduleEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `price_level` - Relative price level of this schedule entry (-9 to 9)
    ///
    /// # Returns
    ///
    /// A new instance of `PriceLevelScheduleEntryType` with optional fields set to `None`
    pub fn new(duration: i32, price_level: i8) -> Self {
        Self {
            duration,
            price_level,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price level schedule entry
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

    /// Gets the price level.
    ///
    /// # Returns
    ///
    /// The relative price level of this schedule entry (-9 to 9)
    pub fn price_level(&self) -> i8 {
        self.price_level
    }

    /// Sets the price level.
    ///
    /// # Arguments
    ///
    /// * `price_level` - Relative price level of this schedule entry (-9 to 9)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_level(&mut self, price_level: i8) -> &mut Self {
        self.price_level = price_level;
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
    /// * `custom_data` - Custom data for this price level schedule entry, or None to clear
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

    #[test]
    fn test_new_price_level_schedule_entry() {
        let duration = 3600;
        let price_level = 2;

        let entry = PriceLevelScheduleEntryType::new(duration, price_level);

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.price_level(), price_level);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let price_level = 2;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let entry = PriceLevelScheduleEntryType::new(duration, price_level)
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.price_level(), price_level);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let price_level1 = 2;
        let duration2 = 7200;
        let price_level2 = 5;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut entry = PriceLevelScheduleEntryType::new(duration1, price_level1);

        entry
            .set_duration(duration2)
            .set_price_level(price_level2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.duration(), duration2);
        assert_eq!(entry.price_level(), price_level2);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry.set_custom_data(None);
        assert_eq!(entry.custom_data(), None);
    }
}
