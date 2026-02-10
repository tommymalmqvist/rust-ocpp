use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Parameters for periodic event stream configuration.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicEventStreamParamsType {
    /// Required. Time in seconds after which stream data is sent.
    #[validate(range(min = 0, max = 86400))]
    pub interval: i32,

    /// Required. Number of items to be sent together in stream.
    #[validate(range(min = 0))]
    pub values: i32,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PeriodicEventStreamParamsType {
    /// Creates a new `PeriodicEventStreamParamsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `interval` - Time in seconds after which stream data is sent
    /// * `values` - Number of items to be sent together in stream
    ///
    /// # Returns
    ///
    /// A new instance of `PeriodicEventStreamParamsType` with optional fields set to `None`
    pub fn new(interval: i32, values: i32) -> Self {
        Self {
            interval,
            values,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the interval.
    ///
    /// # Returns
    ///
    /// Time in seconds after which stream data is sent
    pub fn interval(&self) -> i32 {
        self.interval
    }

    /// Sets the interval.
    ///
    /// # Arguments
    ///
    /// * `interval` - Time in seconds after which stream data is sent
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.interval = interval;
        self
    }

    /// Gets the number of values.
    ///
    /// # Returns
    ///
    /// Number of items to be sent together in stream
    pub fn values(&self) -> i32 {
        self.values
    }

    /// Sets the number of values.
    ///
    /// # Arguments
    ///
    /// * `values` - Number of items to be sent together in stream
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_values(&mut self, values: i32) -> &mut Self {
        self.values = values;
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
    /// * `custom_data` - Custom data for these parameters, or None to clear
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
    fn test_new_periodic_event_stream_params() {
        let params = PeriodicEventStreamParamsType::new(60, 10);

        assert_eq!(params.interval(), 60);
        assert_eq!(params.values(), 10);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let params =
            PeriodicEventStreamParamsType::new(60, 10).with_custom_data(custom_data.clone());

        assert_eq!(params.interval(), 60);
        assert_eq!(params.values(), 10);
        assert_eq!(params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut params = PeriodicEventStreamParamsType::new(60, 10);

        params
            .set_interval(120)
            .set_values(20)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(params.interval(), 120);
        assert_eq!(params.values(), 20);
        assert_eq!(params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        params.set_custom_data(None);
        assert_eq!(params.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Test valid case
        let params = PeriodicEventStreamParamsType::new(60, 10);
        assert!(
            params.validate().is_ok(),
            "Valid params should pass validation"
        );

        // Test interval below minimum
        let mut params = PeriodicEventStreamParamsType::new(-1, 10);
        assert!(
            params.validate().is_err(),
            "Interval below minimum should fail validation"
        );

        // Test values below minimum
        params = PeriodicEventStreamParamsType::new(60, -1);
        assert!(
            params.validate().is_err(),
            "Values below minimum should fail validation"
        );

        // Test interval above maximum
        params = PeriodicEventStreamParamsType::new(86401, 10);
        assert!(
            params.validate().is_err(),
            "Interval above maximum should fail validation"
        );
    }
}
