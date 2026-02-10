use super::custom_data::CustomDataType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// Log parameters for GetLog request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LogParametersType {
    /// Required. The Id of this request.
    #[validate(length(max = 2000))]
    pub remote_location: String,

    /// Required. The oldest log entry date/time to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_timestamp: Option<DateTime<Utc>>,

    /// Optional. The latest log entry date/time to include in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_timestamp: Option<DateTime<Utc>>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

/// Validates that the latest timestamp is after the oldest timestamp if both are provided.
///
/// # Arguments
///
/// * `oldest` - The oldest timestamp
/// * `latest` - The latest timestamp
///
/// # Returns
///
/// Returns Ok(()) if the timestamps are valid, otherwise returns Err
fn validate_timestamps(
    oldest: &Option<DateTime<Utc>>,
    latest: &Option<DateTime<Utc>>,
) -> Result<(), ValidationError> {
    if let (Some(oldest), Some(latest)) = (oldest, latest) {
        if latest <= oldest {
            let mut error = ValidationError::new("latest_timestamp_before_oldest");
            error.message = Some("Latest timestamp must be after oldest timestamp".into());
            return Err(error);
        }
    }
    Ok(())
}

impl LogParametersType {
    /// Creates a new `LogParametersType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `remote_location` - The Id of this request
    ///
    /// # Returns
    ///
    /// A new instance of `LogParametersType` with optional fields set to `None`
    pub fn new(remote_location: String) -> Self {
        Self {
            remote_location,
            oldest_timestamp: None,
            latest_timestamp: None,
            custom_data: None,
        }
    }

    /// Sets the oldest timestamp.
    ///
    /// # Arguments
    ///
    /// * `oldest_timestamp` - The oldest log entry date/time to include in the response
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_oldest_timestamp(mut self, oldest_timestamp: DateTime<Utc>) -> Self {
        self.oldest_timestamp = Some(oldest_timestamp);
        self
    }

    /// Sets the latest timestamp.
    ///
    /// # Arguments
    ///
    /// * `latest_timestamp` - The latest log entry date/time to include in the response
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_latest_timestamp(mut self, latest_timestamp: DateTime<Utc>) -> Self {
        self.latest_timestamp = Some(latest_timestamp);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this log parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the remote location.
    ///
    /// # Returns
    ///
    /// The Id of this request
    pub fn remote_location(&self) -> &str {
        &self.remote_location
    }

    /// Sets the remote location.
    ///
    /// # Arguments
    ///
    /// * `remote_location` - The Id of this request
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_remote_location(&mut self, remote_location: String) -> &mut Self {
        self.remote_location = remote_location;
        self
    }

    /// Gets the oldest timestamp.
    ///
    /// # Returns
    ///
    /// An optional reference to the oldest log entry date/time to include in the response
    pub fn oldest_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.oldest_timestamp.as_ref()
    }

    /// Sets the oldest timestamp.
    ///
    /// # Arguments
    ///
    /// * `oldest_timestamp` - The oldest log entry date/time to include in the response, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_oldest_timestamp(&mut self, oldest_timestamp: Option<DateTime<Utc>>) -> &mut Self {
        self.oldest_timestamp = oldest_timestamp;
        self
    }

    /// Gets the latest timestamp.
    ///
    /// # Returns
    ///
    /// An optional reference to the latest log entry date/time to include in the response
    pub fn latest_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.latest_timestamp.as_ref()
    }

    /// Sets the latest timestamp.
    ///
    /// # Arguments
    ///
    /// * `latest_timestamp` - The latest log entry date/time to include in the response, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_latest_timestamp(&mut self, latest_timestamp: Option<DateTime<Utc>>) -> &mut Self {
        self.latest_timestamp = latest_timestamp;
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
    /// * `custom_data` - Custom data for this log parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Validates this instance according to the OCPP 2.1 specification.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the instance is valid, otherwise an error
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();

        // Validate using the derive(Validate) implementation
        if let Err(e) = <Self as Validate>::validate(self) {
            errors.0.extend(e.0);
        }

        // Validate timestamps
        if let Err(e) = validate_timestamps(&self.oldest_timestamp, &self.latest_timestamp) {
            errors.add("timestamps", e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::json;

    #[test]
    fn test_new_log_parameters() {
        let remote_location = "https://example.com/logs".to_string();

        let log_params = LogParametersType::new(remote_location.clone());

        assert_eq!(log_params.remote_location(), remote_location);
        assert_eq!(log_params.oldest_timestamp(), None);
        assert_eq!(log_params.latest_timestamp(), None);
        assert_eq!(log_params.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let latest_timestamp = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let log_params = LogParametersType::new(remote_location.clone())
            .with_oldest_timestamp(oldest_timestamp.clone())
            .with_latest_timestamp(latest_timestamp.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(log_params.remote_location(), remote_location);
        assert_eq!(log_params.oldest_timestamp(), Some(&oldest_timestamp));
        assert_eq!(log_params.latest_timestamp(), Some(&latest_timestamp));
        assert_eq!(log_params.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let remote_location1 = "https://example.com/logs".to_string();
        let remote_location2 = "https://logs.example.org".to_string();
        let _oldest_timestamp1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let oldest_timestamp2 = Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();
        let latest_timestamp = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut log_params = LogParametersType::new(remote_location1.clone());

        log_params
            .set_remote_location(remote_location2.clone())
            .set_oldest_timestamp(Some(oldest_timestamp2.clone()))
            .set_latest_timestamp(Some(latest_timestamp.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(log_params.remote_location(), remote_location2);
        assert_eq!(log_params.oldest_timestamp(), Some(&oldest_timestamp2));
        assert_eq!(log_params.latest_timestamp(), Some(&latest_timestamp));
        assert_eq!(log_params.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        log_params
            .set_oldest_timestamp(None)
            .set_latest_timestamp(None)
            .set_custom_data(None);

        assert_eq!(log_params.oldest_timestamp(), None);
        assert_eq!(log_params.latest_timestamp(), None);
        assert_eq!(log_params.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid values
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let latest_timestamp = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();

        let log_params = LogParametersType::new(remote_location.clone())
            .with_oldest_timestamp(oldest_timestamp)
            .with_latest_timestamp(latest_timestamp);

        assert!(log_params.validate().is_ok());

        // Test with invalid remote_location (too long)
        let long_remote_location = "a".repeat(2001);
        let invalid_log_params = LogParametersType::new(long_remote_location);
        assert!(invalid_log_params.validate().is_err());

        // Test with invalid timestamps (latest before oldest)
        let invalid_latest_timestamp = Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap();
        let invalid_timestamp_params = LogParametersType::new(remote_location.clone())
            .with_oldest_timestamp(oldest_timestamp)
            .with_latest_timestamp(invalid_latest_timestamp);

        assert!(invalid_timestamp_params.validate().is_err());

        // Test with only oldest timestamp
        let only_oldest =
            LogParametersType::new(remote_location.clone()).with_oldest_timestamp(oldest_timestamp);
        assert!(only_oldest.validate().is_ok());

        // Test with only latest timestamp
        let only_latest =
            LogParametersType::new(remote_location.clone()).with_latest_timestamp(latest_timestamp);
        assert!(only_latest.validate().is_ok());

        // Test with same timestamps (should fail)
        let same_timestamps = LogParametersType::new(remote_location)
            .with_oldest_timestamp(oldest_timestamp)
            .with_latest_timestamp(oldest_timestamp); // Same as oldest
        assert!(same_timestamps.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let latest_timestamp = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"))
            .with_property("features".to_string(), json!(["feature1", "feature2"]));

        let log_params = LogParametersType::new(remote_location.clone())
            .with_oldest_timestamp(oldest_timestamp.clone())
            .with_latest_timestamp(latest_timestamp.clone())
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = serde_json::to_string(&log_params).unwrap();

        // Deserialize back
        let deserialized: LogParametersType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.remote_location(), log_params.remote_location());
        assert_eq!(
            deserialized.oldest_timestamp(),
            log_params.oldest_timestamp()
        );
        assert_eq!(
            deserialized.latest_timestamp(),
            log_params.latest_timestamp()
        );
        assert_eq!(
            deserialized.custom_data().is_some(),
            log_params.custom_data().is_some()
        );
        if let Some(custom_data) = deserialized.custom_data() {
            assert_eq!(custom_data.vendor_id, "VendorX");
            assert!(custom_data.additional_properties.contains_key("version"));
            assert!(custom_data.additional_properties.contains_key("features"));
        }
    }

    #[test]
    fn test_json_structure() {
        let remote_location = "https://example.com/logs".to_string();
        let oldest_timestamp = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let latest_timestamp = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let log_params = LogParametersType::new(remote_location)
            .with_oldest_timestamp(oldest_timestamp)
            .with_latest_timestamp(latest_timestamp)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let json_value = serde_json::to_value(&log_params).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        let obj = json_value.as_object().unwrap();

        assert!(obj.contains_key("remoteLocation"));
        assert!(obj.contains_key("oldestTimestamp"));
        assert!(obj.contains_key("latestTimestamp"));
        assert!(obj.contains_key("customData"));

        // Check custom data structure
        let custom_data = &obj["customData"];
        assert!(custom_data.is_object());
        let custom_obj = custom_data.as_object().unwrap();

        assert!(custom_obj.contains_key("vendorId"));
        assert!(custom_obj.contains_key("version"));
        assert_eq!(custom_obj["vendorId"], "VendorX");
        assert_eq!(custom_obj["version"], "1.0");
    }
}
