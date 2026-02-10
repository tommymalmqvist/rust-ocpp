use super::custom_data::CustomDataType;
use chrono::{DateTime, Utc};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

/// Fixed power factor settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct FixedPFType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Power factor, cos(phi), as value between 0..1.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub displacement: Decimal,

    /// True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited).
    pub excitation: bool,

    /// Time when this setting becomes active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Duration of the setting in seconds.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub duration: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl FixedPFType {
    /// Creates a new `FixedPFType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `displacement` - Power factor, cos(phi), as value between 0..1
    /// * `excitation` - True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited)
    ///
    /// # Returns
    ///
    /// A new instance of `FixedPFType` with optional fields set to `None`
    pub fn new(priority: i32, displacement: f64, excitation: bool) -> Self {
        Self {
            priority,
            displacement: Decimal::try_from(displacement).unwrap_or_default(),
            excitation,
            start_time: None,
            duration: None,
            custom_data: None,
        }
    }

    /// Creates a new `FixedPFType` with all fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `displacement` - Power factor, cos(phi), as value between 0..1
    /// * `excitation` - True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited)
    /// * `start_time` - Time when this setting becomes active
    /// * `duration` - Duration of the setting in seconds
    /// * `custom_data` - Custom data for these fixed power factor settings
    ///
    /// # Returns
    ///
    /// A new instance of `FixedPFType` with all fields set
    pub fn new_with_all_fields(
        priority: i32,
        displacement: f64,
        excitation: bool,
        start_time: Option<DateTime<Utc>>,
        duration: Option<f64>,
        custom_data: Option<CustomDataType>,
    ) -> Self {
        Self {
            priority,
            displacement: Decimal::try_from(displacement).unwrap_or_default(),
            excitation,
            start_time,
            duration: duration.map(|d| Decimal::try_from(d).unwrap_or_default()),
            custom_data,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these fixed power factor settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time when this setting becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_time(mut self, start_time: DateTime<Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the setting in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = Some(Decimal::try_from(duration).unwrap_or_default());
        self
    }

    /// Gets the priority.
    ///
    /// # Returns
    ///
    /// The priority of setting (0=highest)
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the displacement (power factor).
    ///
    /// # Returns
    ///
    /// The power factor, cos(phi), as a Decimal value
    pub fn displacement(&self) -> &Decimal {
        &self.displacement
    }

    /// Gets the displacement (power factor) as f64.
    ///
    /// # Returns
    ///
    /// The power factor, cos(phi), as an f64 value, or 0.0 if conversion fails
    pub fn displacement_as_f64(&self) -> f64 {
        self.displacement.to_f64().unwrap_or(0.0)
    }

    /// Sets the displacement (power factor).
    ///
    /// # Arguments
    ///
    /// * `displacement` - Power factor, cos(phi), as value between 0..1
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_displacement(&mut self, displacement: f64) -> &mut Self {
        self.displacement = Decimal::try_from(displacement).unwrap_or_default();
        self
    }

    /// Sets the displacement (power factor) from a Decimal.
    ///
    /// # Arguments
    ///
    /// * `displacement` - Power factor, cos(phi), as a Decimal value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_displacement_decimal(&mut self, displacement: Decimal) -> &mut Self {
        self.displacement = displacement;
        self
    }

    /// Gets the excitation.
    ///
    /// # Returns
    ///
    /// True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited)
    pub fn excitation(&self) -> bool {
        self.excitation
    }

    /// Sets the excitation.
    ///
    /// # Arguments
    ///
    /// * `excitation` - True when absorbing reactive power (under-excited), false when injecting reactive power (over-excited)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_excitation(&mut self, excitation: bool) -> &mut Self {
        self.excitation = excitation;
        self
    }

    /// Gets the start time.
    ///
    /// # Returns
    ///
    /// An optional reference to the time when this setting becomes active
    pub fn start_time(&self) -> Option<&DateTime<Utc>> {
        self.start_time.as_ref()
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time when this setting becomes active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_time(&mut self, start_time: Option<DateTime<Utc>>) -> &mut Self {
        self.start_time = start_time;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// An optional reference to the duration of the setting in seconds
    pub fn duration(&self) -> Option<&Decimal> {
        self.duration.as_ref()
    }

    /// Gets the duration as f64.
    ///
    /// # Returns
    ///
    /// The duration of the setting in seconds as an f64 value, or None if not set or conversion fails
    pub fn duration_as_f64(&self) -> Option<f64> {
        self.duration.as_ref().and_then(|d| d.to_f64())
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the setting in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<f64>) -> &mut Self {
        self.duration = duration.map(|d| Decimal::try_from(d).unwrap_or_default());
        self
    }

    /// Sets the duration from a Decimal.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the setting in seconds as a Decimal, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration_decimal(&mut self, duration: Option<Decimal>) -> &mut Self {
        self.duration = duration;
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
    /// * `custom_data` - Custom data for these fixed power factor settings, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

/// Implementation of the Display trait for FixedPFType
impl fmt::Display for FixedPFType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FixedPFType {{ priority: {}, displacement: {}, excitation: {} }}",
            self.priority, self.displacement, self.excitation
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use serde_json::json;

    #[test]
    fn test_new_fixed_pf() {
        let priority = 1;
        let displacement = 0.95;
        let excitation = true;

        let fixed_pf = FixedPFType::new(priority, displacement, excitation);

        assert_eq!(fixed_pf.priority(), priority);
        assert_eq!(fixed_pf.displacement_as_f64(), displacement);
        assert_eq!(fixed_pf.excitation(), excitation);
        assert_eq!(fixed_pf.start_time(), None);
        assert_eq!(fixed_pf.duration(), None);
        assert_eq!(fixed_pf.custom_data(), None);
    }

    #[test]
    fn test_new_with_all_fields() {
        let priority = 1;
        let displacement = 0.95;
        let excitation = true;
        let start_time = Utc::now();
        let duration = 3600.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_pf = FixedPFType::new_with_all_fields(
            priority,
            displacement,
            excitation,
            Some(start_time.clone()),
            Some(duration),
            Some(custom_data.clone()),
        );

        assert_eq!(fixed_pf.priority(), priority);
        assert_eq!(fixed_pf.displacement_as_f64(), displacement);
        assert_eq!(fixed_pf.excitation(), excitation);
        assert_eq!(fixed_pf.start_time(), Some(&start_time));
        assert_eq!(fixed_pf.duration_as_f64(), Some(duration));
        assert_eq!(fixed_pf.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_with_methods() {
        let priority = 1;
        let displacement = 0.95;
        let excitation = true;
        let start_time = Utc::now();
        let duration = 3600.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_pf = FixedPFType::new(priority, displacement, excitation)
            .with_start_time(start_time.clone())
            .with_duration(duration)
            .with_custom_data(custom_data.clone());

        assert_eq!(fixed_pf.priority(), priority);
        assert_eq!(fixed_pf.displacement_as_f64(), displacement);
        assert_eq!(fixed_pf.excitation(), excitation);
        assert_eq!(fixed_pf.start_time(), Some(&start_time));
        assert_eq!(fixed_pf.duration_as_f64(), Some(duration));
        assert_eq!(fixed_pf.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let displacement1 = 0.95;
        let excitation1 = true;
        let priority2 = 2;
        let displacement2 = 0.85;
        let excitation2 = false;
        let start_time = Utc::now();
        let duration = 3600.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut fixed_pf = FixedPFType::new(priority1, displacement1, excitation1);

        fixed_pf
            .set_priority(priority2)
            .set_displacement(displacement2)
            .set_excitation(excitation2)
            .set_start_time(Some(start_time.clone()))
            .set_duration(Some(duration))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(fixed_pf.priority(), priority2);
        assert_eq!(fixed_pf.displacement_as_f64(), displacement2);
        assert_eq!(fixed_pf.excitation(), excitation2);
        assert_eq!(fixed_pf.start_time(), Some(&start_time));
        assert_eq!(fixed_pf.duration_as_f64(), Some(duration));
        assert_eq!(fixed_pf.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        fixed_pf
            .set_start_time(None)
            .set_duration(None)
            .set_custom_data(None);
        assert_eq!(fixed_pf.start_time(), None);
        assert_eq!(fixed_pf.duration(), None);
        assert_eq!(fixed_pf.custom_data(), None);
    }

    #[test]
    fn test_decimal_setters() {
        let displacement_decimal = dec!(0.95);
        let duration_decimal = dec!(3600);

        let mut fixed_pf = FixedPFType::new(1, 0.0, true);
        fixed_pf
            .set_displacement_decimal(displacement_decimal)
            .set_duration_decimal(Some(duration_decimal));

        assert_eq!(fixed_pf.displacement(), &displacement_decimal);
        assert_eq!(fixed_pf.duration(), Some(&duration_decimal));
    }

    #[test]
    fn test_default() {
        // Test the Default trait implementation
        let default_fixed_pf = FixedPFType::default();

        // Verify default values
        assert_eq!(default_fixed_pf.priority(), 0);
        assert_eq!(default_fixed_pf.displacement_as_f64(), 0.0);
        assert_eq!(default_fixed_pf.excitation(), false);
        assert_eq!(default_fixed_pf.start_time(), None);
        assert_eq!(default_fixed_pf.duration(), None);
        assert_eq!(default_fixed_pf.custom_data(), None);
    }

    #[test]
    fn test_display() {
        // Test the Display trait implementation
        let fixed_pf = FixedPFType::new(1, 0.95, true);

        let display_string = format!("{}", fixed_pf);

        // Verify the display string contains all the important information
        assert!(display_string.contains("priority: 1"));
        assert!(display_string.contains("displacement: 0.95"));
        assert!(display_string.contains("excitation: true"));
    }

    #[test]
    fn test_validation() {
        // Valid FixedPFType with minimum requirements
        let valid_fixed_pf = FixedPFType::new(1, 0.95, true);
        assert!(
            valid_fixed_pf.validate().is_ok(),
            "Valid FixedPFType should pass validation"
        );

        // Invalid priority (negative)
        let invalid_fixed_pf = FixedPFType {
            priority: -1,
            displacement: dec!(0.95),
            excitation: true,
            start_time: None,
            duration: None,
            custom_data: None,
        };
        assert!(
            invalid_fixed_pf.validate().is_err(),
            "FixedPFType with negative priority should fail validation"
        );

        // Invalid custom data
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);
        let invalid_fixed_pf =
            FixedPFType::new(1, 0.95, true).with_custom_data(invalid_custom_data);
        assert!(
            invalid_fixed_pf.validate().is_err(),
            "FixedPFType with invalid custom data should fail validation"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let priority = 1;
        let displacement = 0.95;
        let excitation = true;
        let start_time = Utc::now();
        let duration = 3600.0;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let fixed_pf = FixedPFType::new_with_all_fields(
            priority,
            displacement,
            excitation,
            Some(start_time.clone()),
            Some(duration),
            Some(custom_data.clone()),
        );

        // Serialize to JSON
        let serialized = serde_json::to_string(&fixed_pf).unwrap();

        // Deserialize back
        let deserialized: FixedPFType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(fixed_pf, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_json_structure() {
        let priority = 1;
        let displacement = 0.95;
        let excitation = true;
        let start_time = Utc::now();
        let duration = 3600.0;
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let fixed_pf = FixedPFType::new_with_all_fields(
            priority,
            displacement,
            excitation,
            Some(start_time.clone()),
            Some(duration),
            Some(custom_data.clone()),
        );

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&fixed_pf).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        assert!(json_value.get("priority").is_some());
        assert!(json_value.get("displacement").is_some());
        assert!(json_value.get("excitation").is_some());
        assert!(json_value.get("startTime").is_some());
        assert!(json_value.get("duration").is_some());
        assert!(json_value.get("customData").is_some());

        // Check field values
        assert_eq!(json_value["priority"], 1);
        assert_eq!(json_value["displacement"], 0.95);
        assert_eq!(json_value["excitation"], true);
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_edge_cases() {
        // Test with extreme displacement values
        let high_displacement = FixedPFType::new(1, 1.0, true);
        let low_displacement = FixedPFType::new(1, 0.0, false);

        assert_eq!(high_displacement.displacement_as_f64(), 1.0);
        assert_eq!(low_displacement.displacement_as_f64(), 0.0);

        // Test with very large duration
        let large_duration = 86400.0 * 365.0; // 1 year in seconds
        let fixed_pf = FixedPFType::new(1, 0.95, true).with_duration(large_duration);

        assert_eq!(fixed_pf.duration_as_f64(), Some(large_duration));
    }
}
