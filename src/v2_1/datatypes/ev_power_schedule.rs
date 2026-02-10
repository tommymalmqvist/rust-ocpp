use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, ev_power_schedule_entry::EVPowerScheduleEntryType};

/// Power schedule of EV energy offer.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVPowerScheduleType {
    /// Starting point in time of the EVEnergyOffer.
    pub time_anchor: DateTime<Utc>,

    /// List of power schedule entries.
    #[validate(length(min = 1, max = 1024))]
    #[validate(nested)]
    pub ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EVPowerScheduleType {
    /// Creates a new `EVPowerScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point in time of the EVEnergyOffer
    /// * `ev_power_schedule_entries` - List of power schedule entries
    ///
    /// # Returns
    ///
    /// A new instance of `EVPowerScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
    ) -> Self {
        Self {
            time_anchor,
            ev_power_schedule_entries,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this power schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the time anchor.
    ///
    /// # Returns
    ///
    /// A reference to the starting point in time of the EVEnergyOffer
    pub fn time_anchor(&self) -> &DateTime<Utc> {
        &self.time_anchor
    }

    /// Sets the time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point in time of the EVEnergyOffer
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: DateTime<Utc>) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the power schedule entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of power schedule entries
    pub fn ev_power_schedule_entries(&self) -> &Vec<EVPowerScheduleEntryType> {
        &self.ev_power_schedule_entries
    }

    /// Sets the power schedule entries.
    ///
    /// # Arguments
    ///
    /// * `ev_power_schedule_entries` - List of power schedule entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_power_schedule_entries(
        &mut self,
        ev_power_schedule_entries: Vec<EVPowerScheduleEntryType>,
    ) -> &mut Self {
        self.ev_power_schedule_entries = ev_power_schedule_entries;
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
    /// * `custom_data` - Custom data for this power schedule, or None to clear
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
    use serde_json::json;
    use validator::Validate;

    #[test]
    fn test_new_ev_power_schedule() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];

        let schedule = EVPowerScheduleType::new(time_anchor.clone(), entries.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let schedule = EVPowerScheduleType::new(time_anchor.clone(), entries.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries);
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();
        let entries1 = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];

        let time_anchor2 = Utc::now();
        let entries2 = vec![
            EVPowerScheduleEntryType::new(1800, 22000.0),
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(5400, 5500.0),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut schedule = EVPowerScheduleType::new(time_anchor1.clone(), entries1.clone());

        schedule
            .set_time_anchor(time_anchor2.clone())
            .set_ev_power_schedule_entries(entries2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), &time_anchor2);
        assert_eq!(schedule.ev_power_schedule_entries(), &entries2);
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_custom_data(None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_validation_basic() {
        // Valid case with minimum requirements
        let time_anchor = Utc::now();
        let entries = vec![EVPowerScheduleEntryType::new(3600, 11000.0)];
        let schedule = EVPowerScheduleType::new(time_anchor, entries);

        assert!(
            schedule.validate().is_ok(),
            "Valid schedule should pass validation"
        );

        // Valid case with multiple entries
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
            EVPowerScheduleEntryType::new(10800, 5000.0),
        ];
        let schedule = EVPowerScheduleType::new(time_anchor, entries);

        assert!(
            schedule.validate().is_ok(),
            "Schedule with multiple entries should pass validation"
        );
    }

    #[test]
    fn test_validation_errors() {
        // Test with empty entries vector (should fail validation)
        let time_anchor = Utc::now();
        let empty_entries: Vec<EVPowerScheduleEntryType> = vec![];
        let invalid_schedule = EVPowerScheduleType::new(time_anchor, empty_entries);

        let validation_result = invalid_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with empty entries should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the ev_power_schedule_entries field for length validation
        assert!(
            field_errors.contains_key("ev_power_schedule_entries"),
            "Validation errors should contain ev_power_schedule_entries field"
        );

        let entries_errors = &field_errors["ev_power_schedule_entries"];
        assert!(
            !entries_errors.is_empty(),
            "ev_power_schedule_entries field should have validation errors"
        );
        assert_eq!(
            entries_errors[0].code, "length",
            "ev_power_schedule_entries field should have a length error"
        );
    }

    #[test]
    fn test_nested_validation() {
        // Test nested validation for EVPowerScheduleEntryType
        let time_anchor = Utc::now();

        // Create an entry with invalid custom data (vendor_id too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let entry =
            EVPowerScheduleEntryType::new(3600, 11000.0).with_custom_data(invalid_custom_data);

        let schedule = EVPowerScheduleType::new(time_anchor, vec![entry]);

        // Validation should fail due to invalid nested custom_data
        let validation_result = schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with invalid nested custom_data should fail validation"
        );
    }

    #[test]
    fn test_serialization_deserialization() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let schedule =
            EVPowerScheduleType::new(time_anchor.clone(), entries).with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&schedule).unwrap();

        // Deserialize back
        let deserialized: EVPowerScheduleType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(schedule, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_json_structure() {
        let time_anchor = Utc::now();
        let entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let schedule =
            EVPowerScheduleType::new(time_anchor.clone(), entries).with_custom_data(custom_data);

        // Serialize to JSON Value
        let json_value = serde_json::to_value(&schedule).unwrap();

        // Check JSON structure
        assert!(json_value.is_object());
        assert!(json_value.get("timeAnchor").is_some());
        assert!(json_value.get("evPowerScheduleEntries").is_some());
        assert!(json_value.get("customData").is_some());

        // Check entries array
        let entries_json = json_value.get("evPowerScheduleEntries").unwrap();
        assert!(entries_json.is_array());
        assert_eq!(entries_json.as_array().unwrap().len(), 2);

        // Check first entry structure
        let first_entry = &entries_json.as_array().unwrap()[0];
        assert!(first_entry.get("duration").is_some());
        assert!(first_entry.get("power").is_some());

        // Check custom data
        let custom_data_json = json_value.get("customData").unwrap();
        assert_eq!(custom_data_json.get("vendorId").unwrap(), "VendorX");
        assert_eq!(custom_data_json.get("version").unwrap(), "1.0");
    }

    #[test]
    fn test_deserialization_from_json() {
        // Create a JSON string representing an EVPowerScheduleType
        let json_str = r#"{
            "timeAnchor": "2023-01-01T12:00:00Z",
            "evPowerScheduleEntries": [
                {
                    "duration": 3600,
                    "power": 11000
                },
                {
                    "duration": 7200,
                    "power": 7500
                }
            ],
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        // Deserialize from JSON string
        let schedule: EVPowerScheduleType = serde_json::from_str(json_str).unwrap();

        // Verify deserialized values
        assert_eq!(
            schedule.time_anchor().to_rfc3339(),
            "2023-01-01T12:00:00+00:00"
        );
        assert_eq!(schedule.ev_power_schedule_entries().len(), 2);
        assert_eq!(schedule.ev_power_schedule_entries()[0].duration(), 3600);
        assert_eq!(schedule.ev_power_schedule_entries()[1].duration(), 7200);
        assert_eq!(schedule.custom_data().unwrap().vendor_id(), "TestVendor");
    }

    #[test]
    fn test_max_entries_validation() {
        // Create a schedule with maximum allowed entries (1024)
        let time_anchor = Utc::now();
        let max_entries = (0..1024)
            .map(|i| EVPowerScheduleEntryType::new(i * 60, 1000.0))
            .collect();

        let max_schedule = EVPowerScheduleType::new(time_anchor.clone(), max_entries);

        // Should pass validation with exactly 1024 entries
        assert!(
            max_schedule.validate().is_ok(),
            "Schedule with 1024 entries should pass validation"
        );

        // Create a schedule with too many entries (1025)
        let time_anchor = Utc::now();
        let too_many_entries = (0..1025)
            .map(|i| EVPowerScheduleEntryType::new(i * 60, 1000.0))
            .collect();

        let invalid_schedule = EVPowerScheduleType::new(time_anchor.clone(), too_many_entries);

        // Should fail validation with more than 1024 entries
        let validation_result = invalid_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with 1025 entries should fail validation"
        );
    }
}
