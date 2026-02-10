use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{charging_schedule_period::ChargingSchedulePeriodType, custom_data::CustomDataType};
use crate::v2_1::enumerations::ChargingRateUnitEnumType;

/// Composite Schedule structure defines a list of charging periods.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompositeScheduleType {
    /// The ID of the EVSE for which the schedule is requested.
    /// When evseid=0, the Charging Station calculated the expected consumption for the grid connection.
    #[validate(range(min = 0))]
    pub evse_id: i32,

    /// Duration of the schedule in seconds.
    pub duration: i32,

    /// Date and time at which the schedule becomes active.
    /// All time measurements within the schedule are relative to this timestamp.
    pub schedule_start: DateTime<Utc>,

    /// The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,

    /// List of charging periods describing the amount of power or current that can be delivered per time interval.
    #[validate(length(min = 1), nested)]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CompositeScheduleType {
    /// Creates a new `CompositeScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - The ID of the EVSE for which the schedule is requested
    /// * `duration` - Duration of the schedule in seconds
    /// * `schedule_start` - Date and time at which the schedule becomes active
    /// * `charging_rate_unit` - The unit of measure in which limits and setpoints are expressed
    /// * `charging_schedule_period` - List of charging periods
    ///
    /// # Returns
    ///
    /// A new instance of `CompositeScheduleType` with optional fields set to `None`
    pub fn new(
        evse_id: i32,
        duration: i32,
        schedule_start: DateTime<Utc>,
        charging_rate_unit: ChargingRateUnitEnumType,
        charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    ) -> Self {
        Self {
            evse_id,
            duration,
            schedule_start,
            charging_rate_unit,
            charging_schedule_period,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this composite schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EVSE ID.
    ///
    /// # Returns
    ///
    /// The ID of the EVSE for which the schedule is requested
    pub fn evse_id(&self) -> i32 {
        self.evse_id
    }

    /// Sets the EVSE ID.
    ///
    /// # Arguments
    ///
    /// * `evse_id` - The ID of the EVSE for which the schedule is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_id(&mut self, evse_id: i32) -> &mut Self {
        self.evse_id = evse_id;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// Duration of the schedule in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the schedule start time.
    ///
    /// # Returns
    ///
    /// A reference to the date and time at which the schedule becomes active
    pub fn schedule_start(&self) -> &DateTime<Utc> {
        &self.schedule_start
    }

    /// Sets the schedule start time.
    ///
    /// # Arguments
    ///
    /// * `schedule_start` - Date and time at which the schedule becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_schedule_start(&mut self, schedule_start: DateTime<Utc>) -> &mut Self {
        self.schedule_start = schedule_start;
        self
    }

    /// Gets the charging rate unit.
    ///
    /// # Returns
    ///
    /// The unit of measure in which limits and setpoints are expressed
    pub fn charging_rate_unit(&self) -> &ChargingRateUnitEnumType {
        &self.charging_rate_unit
    }

    /// Sets the charging rate unit.
    ///
    /// # Arguments
    ///
    /// * `charging_rate_unit` - The unit of measure in which limits and setpoints are expressed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_rate_unit(
        &mut self,
        charging_rate_unit: ChargingRateUnitEnumType,
    ) -> &mut Self {
        self.charging_rate_unit = charging_rate_unit;
        self
    }

    /// Gets the charging schedule periods.
    ///
    /// # Returns
    ///
    /// A reference to the list of charging periods
    pub fn charging_schedule_period(&self) -> &[ChargingSchedulePeriodType] {
        &self.charging_schedule_period
    }

    /// Sets the charging schedule periods.
    ///
    /// # Arguments
    ///
    /// * `charging_schedule_period` - List of charging periods
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_schedule_period(
        &mut self,
        charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    ) -> &mut Self {
        self.charging_schedule_period = charging_schedule_period;
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
    /// * `custom_data` - Custom data for this composite schedule, or None to clear
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
    use validator::Validate;

    #[test]
    fn test_new_composite_schedule() {
        let start_time = Utc::now();
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);

        let schedule = CompositeScheduleType::new(
            1,
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period.clone()],
        );

        assert_eq!(schedule.evse_id(), 1);
        assert_eq!(schedule.duration(), 3600);
        assert_eq!(schedule.schedule_start(), &start_time);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::A);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(schedule.charging_schedule_period()[0].start_period(), 0);
        assert_eq!(schedule.charging_schedule_period()[0].limit(), &dec!(16.0));
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let start_time = Utc::now();
        let period = ChargingSchedulePeriodType::new(0, dec!(16.0));
        let custom_data = CustomDataType::new("VendorX".to_string());

        let schedule = CompositeScheduleType::new(
            1,
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period.clone()],
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(schedule.evse_id(), 1);
        assert_eq!(schedule.duration(), 3600);
        assert_eq!(schedule.schedule_start(), &start_time);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::A);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let start_time = Utc::now();
        let new_start_time = Utc::now();
        let period1 = ChargingSchedulePeriodType::new_from_f64(0, 16.0);
        let period2 = ChargingSchedulePeriodType::new_from_f64(3600, 32.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut schedule = CompositeScheduleType::new(
            1,
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period1.clone()],
        );

        schedule
            .set_evse_id(2)
            .set_duration(7200)
            .set_schedule_start(new_start_time)
            .set_charging_rate_unit(ChargingRateUnitEnumType::W)
            .set_charging_schedule_period(vec![period1.clone(), period2.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.evse_id(), 2);
        assert_eq!(schedule.duration(), 7200);
        assert_eq!(schedule.schedule_start(), &new_start_time);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::W);
        assert_eq!(schedule.charging_schedule_period().len(), 2);
        assert_eq!(schedule.charging_schedule_period()[0].start_period(), 0);
        assert_eq!(schedule.charging_schedule_period()[1].start_period(), 3600);
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_custom_data(None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_valid_validation() {
        let start_time = Utc::now();
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let schedule = CompositeScheduleType::new(
            1,
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period.clone()],
        )
        .with_custom_data(custom_data);

        // Valid instance should pass validation
        assert!(schedule.validate().is_ok());
    }

    #[test]
    fn test_invalid_evse_id_validation() {
        let start_time = Utc::now();
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);

        // Create a schedule with negative EVSE ID (invalid)
        let schedule = CompositeScheduleType::new(
            -1, // Invalid: should be >= 0
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period.clone()],
        );

        // Should fail validation due to negative EVSE ID
        let validation_result = schedule.validate();
        assert!(validation_result.is_err());

        // Check that the error is related to evse_id field
        let errors = validation_result.unwrap_err();
        assert!(errors.field_errors().contains_key("evse_id"));
    }

    #[test]
    fn test_empty_charging_schedule_period_validation() {
        let start_time = Utc::now();

        // Create a schedule with empty charging schedule period (invalid)
        let schedule = CompositeScheduleType {
            evse_id: 1,
            duration: 3600,
            schedule_start: start_time,
            charging_rate_unit: ChargingRateUnitEnumType::A,
            charging_schedule_period: vec![], // Invalid: should have at least one period
            custom_data: None,
        };

        // Should fail validation due to empty charging_schedule_period
        let validation_result = schedule.validate();
        assert!(validation_result.is_err());

        // Check that the error is related to charging_schedule_period field
        let errors = validation_result.unwrap_err();
        assert!(errors
            .field_errors()
            .contains_key("charging_schedule_period"));
    }

    #[test]
    fn test_nested_invalid_custom_data_validation() {
        let start_time = Utc::now();
        let period = ChargingSchedulePeriodType::new_from_f64(0, 16.0);

        // Create custom data with invalid vendor_id (longer than 255 chars)
        let too_long_vendor_id = "X".repeat(256);
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let schedule = CompositeScheduleType::new(
            1,
            3600,
            start_time,
            ChargingRateUnitEnumType::A,
            vec![period.clone()],
        )
        .with_custom_data(invalid_custom_data);

        // Should fail validation due to nested custom_data validation
        let validation_result = schedule.validate();

        // The test passes if validation fails (we know the structure is invalid)
        assert!(
            validation_result.is_err(),
            "Validation should fail with invalid nested data"
        );
    }
}
