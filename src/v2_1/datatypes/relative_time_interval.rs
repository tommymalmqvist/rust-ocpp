use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Time interval relative to a fixed point in time defined in the message that contains this type.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RelativeTimeIntervalType {
    /// Required. Start of the interval, in seconds from NOW.
    pub start: i32,

    /// Required. Duration of the interval, in seconds.
    pub duration: i32,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl RelativeTimeIntervalType {
    /// Creates a new `RelativeTimeIntervalType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `start` - Start of the interval, in seconds from NOW
    /// * `duration` - Duration of the interval, in seconds
    ///
    /// # Returns
    ///
    /// A new `RelativeTimeIntervalType` instance with optional fields set to `None`
    pub fn new(start: i32, duration: i32) -> Self {
        Self {
            custom_data: None,
            start,
            duration,
        }
    }

    /// Creates a new `RelativeTimeIntervalType` with default values.
    ///
    /// # Returns
    ///
    /// A new `RelativeTimeIntervalType` instance with start=0, duration=0, and custom_data=None
    pub fn new_default() -> Self {
        Self {
            custom_data: None,
            start: 0,
            duration: 0,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `RelativeTimeIntervalType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `RelativeTimeIntervalType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the start of the interval.
    ///
    /// # Returns
    ///
    /// The start of the interval, in seconds from NOW
    pub fn start(&self) -> i32 {
        self.start
    }

    /// Sets the start of the interval.
    ///
    /// # Arguments
    ///
    /// * `start` - Start of the interval, in seconds from NOW
    ///
    /// # Returns
    ///
    /// The modified `RelativeTimeIntervalType` instance
    pub fn set_start(&mut self, start: i32) -> &mut Self {
        self.start = start;
        self
    }

    /// Gets the duration of the interval.
    ///
    /// # Returns
    ///
    /// The duration of the interval, in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration of the interval.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the interval, in seconds
    ///
    /// # Returns
    ///
    /// The modified `RelativeTimeIntervalType` instance
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_time_interval_new() {
        let start = 10;
        let duration = 60;

        let interval = RelativeTimeIntervalType::new(start, duration);

        assert_eq!(interval.start(), start);
        assert_eq!(interval.duration(), duration);
        assert_eq!(interval.custom_data(), None);
    }

    #[test]
    fn test_relative_time_interval_new_default() {
        let interval = RelativeTimeIntervalType::new_default();

        assert_eq!(interval.start(), 0);
        assert_eq!(interval.duration(), 0);
        assert_eq!(interval.custom_data(), None);
    }

    #[test]
    fn test_relative_time_interval_with_methods() {
        let start = 10;
        let duration = 60;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let interval =
            RelativeTimeIntervalType::new(start, duration).with_custom_data(custom_data.clone());

        assert_eq!(interval.start(), start);
        assert_eq!(interval.duration(), duration);
        assert_eq!(interval.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_relative_time_interval_setters() {
        let start1 = 10;
        let start2 = 20;
        let duration1 = 60;
        let duration2 = 120;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut interval = RelativeTimeIntervalType::new(start1, duration1);

        interval
            .set_start(start2)
            .set_duration(duration2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(interval.start(), start2);
        assert_eq!(interval.duration(), duration2);
        assert_eq!(interval.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        interval.set_custom_data(None);
        assert_eq!(interval.custom_data(), None);
    }
}
