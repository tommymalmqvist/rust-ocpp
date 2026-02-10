use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{
        AbsolutePriceScheduleType, ChargingSchedulePeriodType, CustomDataType, LimitAtSoCType,
        PriceLevelScheduleType, SalesTariffType,
    },
    enumerations::ChargingRateUnitEnumType,
};

/// Charging schedule structure defines a list of charging periods, as used in: NotifyEVChargingScheduleRequest and ChargingProfileType.
/// When used in a NotifyEVChargingScheduleRequest only duration and chargingSchedulePeriod are relevant and chargingRateUnit must be 'W'.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    /// Identifies the ChargingSchedule.
    pub id: i32,

    /// Starting point of an absolute schedule or recurring schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,

    /// Duration of the charging schedule in seconds.
    /// If the duration is left empty, the last period will continue indefinitely or until end of the transaction
    /// in case startSchedule is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,

    /// The unit of measure in which limits and setpoints are expressed.
    pub charging_rate_unit: ChargingRateUnitEnumType,

    /// Minimum charging rate supported by the EV. The unit of measure is defined by the chargingRateUnit.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charging_rate: Option<Decimal>,

    /// *(2.1)* Power tolerance when following EVPowerProfile.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub power_tolerance: Option<Decimal>,

    /// *(2.1)* Power tolerance when following EVPowerProfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = "0"))]
    pub signature_id: Option<i32>,

    /// *(2.1)* Base64 encoded hash (SHA256 for ISO 15118-2, SHA512 for ISO 15118-20) of the EXI price schedule element. Used in signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 88))]
    pub digest_value: Option<String>,

    /// *(2.1)* Defaults to false. When true, disregard time zone offset in dateTime fields of  _ChargingScheduleType_ and use unqualified local time at Charging Station instead.\r\n This allows the same `Absolute` or `Recurring` charging profile to be used in both summer and winter time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_local_time: Option<bool>,

    /// *(2.1)* Defaults to 0. When _randomizedDelay_ not equals zero, then the start of each &lt;&lt;cmn_chargingscheduleperiodtype,ChargingSchedulePeriodType&gt;&gt; is delayed by a randomly chosen number of seconds between 0 and _randomizedDelay_.  Only allowed for TxProfile and TxDefaultProfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = "0"))]
    pub randomized_delay: Option<i32>,

    /// Sales tariff for charging associated with this schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub sales_tariff: Option<SalesTariffType>,

    /// List of charging periods describing the amount of power or current that can be delivered per time interval.
    #[validate(length(min = 1, max = 1024))]
    #[validate(nested)]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,

    /// The ISO 15118-20 absolute price schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub absolute_price_schedule: Option<AbsolutePriceScheduleType>,

    /// (2.1) The ISO 15118-20 price level schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub price_level_schedule: Option<PriceLevelScheduleType>,

    /// 2.1) When present and SoC of EV is greater than or equal to soc, then charging limit or setpoint will be capped to the value of limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub limit_at_so_c: Option<LimitAtSoCType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingScheduleType {
    /// Creates a new `ChargingScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifies the ChargingSchedule
    /// * `charging_rate_unit` - The unit of measure in which limits and setpoints are expressed
    /// * `charging_schedule_period` - List of charging periods
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingScheduleType` with optional fields set to `None`
    pub fn new(
        id: i32,
        charging_rate_unit: ChargingRateUnitEnumType,
        charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    ) -> Self {
        Self {
            id,
            charging_rate_unit,
            charging_schedule_period,
            custom_data: None,
            start_schedule: None,
            duration: None,
            min_charging_rate: None,
            power_tolerance: None,
            signature_id: None,
            digest_value: None,
            use_local_time: None,
            randomized_delay: None,
            sales_tariff: None,
            absolute_price_schedule: None,
            price_level_schedule: None,
            limit_at_so_c: None,
        }
    }

    /// Sets the start schedule.
    ///
    /// # Arguments
    ///
    /// * `start_schedule` - Starting point of an absolute schedule or recurring schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_schedule(mut self, start_schedule: DateTime<Utc>) -> Self {
        self.start_schedule = Some(start_schedule);
        self
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the charging schedule in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The ID of the charging schedule
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Identifies the ChargingSchedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the start schedule.
    ///
    /// # Returns
    ///
    /// An optional reference to the start schedule
    pub fn start_schedule(&self) -> Option<&DateTime<Utc>> {
        self.start_schedule.as_ref()
    }

    /// Sets the start schedule.
    ///
    /// # Arguments
    ///
    /// * `start_schedule` - Starting point of an absolute schedule or recurring schedule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_schedule(&mut self, start_schedule: Option<DateTime<Utc>>) -> &mut Self {
        self.start_schedule = start_schedule;
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// An optional duration of the charging schedule in seconds
    pub fn duration(&self) -> Option<i32> {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the charging schedule in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<i32>) -> &mut Self {
        self.duration = duration;
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
    pub fn charging_schedule_period(&self) -> &Vec<ChargingSchedulePeriodType> {
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
    /// * `custom_data` - Custom data for this charging schedule, or None to clear
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
    use crate::v2_1::datatypes::rational_number::RationalNumberType;
    use rust_decimal_macros::dec;
    use serde_json::{from_str, to_string};
    use validator::Validate;

    fn create_test_period() -> ChargingSchedulePeriodType {
        ChargingSchedulePeriodType::new_from_f64(0, 16.0)
    }

    #[test]
    fn test_new_charging_schedule() {
        let period = create_test_period();
        let schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period.clone()]);

        assert_eq!(schedule.id(), 1);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::A);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period.limit);
        assert_eq!(schedule.start_schedule(), None);
        assert_eq!(schedule.duration(), None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let period = create_test_period();
        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc::now();

        let schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()])
                .with_start_schedule(start_time)
                .with_duration(3600)
                .with_custom_data(custom_data.clone());

        assert_eq!(schedule.id(), 1);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::W);
        assert_eq!(schedule.charging_schedule_period().len(), 1);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period.limit);
        assert_eq!(schedule.start_schedule(), Some(&start_time));
        assert_eq!(schedule.duration(), Some(3600));
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let period1 = create_test_period();
        let period2 = ChargingSchedulePeriodType::new_from_f64(3600, 32.0);

        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc::now();

        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::A, vec![period1.clone()]);

        schedule
            .set_id(2)
            .set_charging_rate_unit(ChargingRateUnitEnumType::W)
            .set_charging_schedule_period(vec![period1.clone(), period2.clone()])
            .set_start_schedule(Some(start_time))
            .set_duration(Some(7200))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.id(), 2);
        assert_eq!(schedule.charging_rate_unit(), &ChargingRateUnitEnumType::W);
        assert_eq!(schedule.charging_schedule_period().len(), 2);
        assert_eq!(
            schedule.charging_schedule_period()[0].start_period,
            period1.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[0].limit, period1.limit);
        assert_eq!(
            schedule.charging_schedule_period()[1].start_period,
            period2.start_period
        );
        assert_eq!(schedule.charging_schedule_period()[1].limit, period2.limit);
        assert_eq!(schedule.start_schedule(), Some(&start_time));
        assert_eq!(schedule.duration(), Some(7200));
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule
            .set_start_schedule(None)
            .set_duration(None)
            .set_custom_data(None);

        assert_eq!(schedule.start_schedule(), None);
        assert_eq!(schedule.duration(), None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_serialization_deserialization() {
        let mut period = create_test_period();
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        let custom_data = CustomDataType::new("VendorX".to_string());
        let start_time = Utc::now();

        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()])
                .with_start_schedule(start_time)
                .with_duration(3600)
                .with_custom_data(custom_data.clone());

        schedule.min_charging_rate = Some(dec!(5.0));
        schedule.power_tolerance = Some(dec!(5.0));

        // Serialize to JSON
        let serialized = to_string(&schedule).unwrap();

        // Verify JSON contains expected fields
        assert!(serialized.contains(r#""id":1"#));
        assert!(serialized.contains(r#""chargingRateUnit":"W""#));
        assert!(serialized.contains(r#""duration":3600"#));
        assert!(serialized.contains(r#""vendorId":"VendorX""#));
        assert!(serialized.contains(r#""minChargingRate":5.0"#));
        assert!(serialized.contains(r#""powerTolerance":5.0"#));

        // Print the serialized JSON for debugging
        println!("Serialized JSON: {}", serialized);

        // Create a JSON string for deserialization
        let json = format!(
            r#"{{
            "id": 1,
            "chargingRateUnit": "W",
            "duration": 3600,
            "startSchedule": "{}",
            "minChargingRate": 5.0,
            "powerTolerance": 5.0,
            "chargingSchedulePeriod": [{{
                "startPeriod": 0,
                "limit": 16.0,
                "limit_L2": 16.0,
                "limit_L3": 16.0,
                "dischargeLimit": -10.0,
                "dischargeLimit_L2": -10.0,
                "dischargeLimit_L3": -10.0,
                "setpoint": 20.0,
                "setpoint_L2": 21.0,
                "setpoint_L3": 22.0,
                "setpointReactive": 5.0,
                "setpointReactive_L2": 6.0,
                "setpointReactive_L3": 7.0,
                "v2xBaseline": 50.0
            }}],
            "customData": {{
                "vendorId": "VendorX"
            }}
        }}"#,
            start_time.to_rfc3339()
        );

        // Deserialize back
        let deserialized: ChargingScheduleType = from_str(&json).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(schedule.id(), deserialized.id());
        assert_eq!(
            schedule.charging_rate_unit(),
            deserialized.charging_rate_unit()
        );
        assert_eq!(schedule.duration(), deserialized.duration());
        assert_eq!(schedule.min_charging_rate, deserialized.min_charging_rate);
        assert_eq!(schedule.power_tolerance, deserialized.power_tolerance);
        assert_eq!(
            schedule.custom_data().unwrap().vendor_id(),
            deserialized.custom_data().unwrap().vendor_id()
        );
        assert_eq!(
            schedule.charging_schedule_period().len(),
            deserialized.charging_schedule_period().len()
        );
    }

    #[test]
    fn test_ocpp_2_1_specific_fields() {
        let mut period = create_test_period();
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        // Create a schedule with OCPP 2.1 specific fields
        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()]);

        // Set OCPP 2.1 specific fields
        schedule.power_tolerance = Some(dec!(5.0));
        schedule.signature_id = Some(42);
        schedule.digest_value = Some("base64_encoded_hash_value".to_string());
        schedule.use_local_time = Some(true);
        schedule.randomized_delay = Some(30);

        // Verify fields are set correctly
        assert_eq!(schedule.power_tolerance, Some(dec!(5.0)));
        assert_eq!(schedule.signature_id, Some(42));
        assert_eq!(
            schedule.digest_value,
            Some("base64_encoded_hash_value".to_string())
        );
        assert_eq!(schedule.use_local_time, Some(true));
        assert_eq!(schedule.randomized_delay, Some(30));

        // Serialize to JSON
        let serialized = to_string(&schedule).unwrap();

        // Verify JSON contains OCPP 2.1 specific fields
        assert!(serialized.contains(r#""powerTolerance":5.0"#));
        assert!(serialized.contains(r#""signatureId":42"#));
        assert!(serialized.contains(r#""digestValue":"base64_encoded_hash_value""#));
        assert!(serialized.contains(r#""useLocalTime":true"#));
        assert!(serialized.contains(r#""randomizedDelay":30"#));

        // Create a JSON string for deserialization
        let json = r#"{
            "id": 1,
            "chargingRateUnit": "W",
            "chargingSchedulePeriod": [{
                "startPeriod": 0,
                "limit": 16.0,
                "limit_L2": 16.0,
                "limit_L3": 16.0,
                "dischargeLimit": -10.0,
                "dischargeLimit_L2": -10.0,
                "dischargeLimit_L3": -10.0,
                "setpoint": 20.0,
                "setpoint_L2": 21.0,
                "setpoint_L3": 22.0,
                "setpointReactive": 5.0,
                "setpointReactive_L2": 6.0,
                "setpointReactive_L3": 7.0,
                "v2xBaseline": 50.0
            }],
            "minChargingRate": 5.0,
            "powerTolerance": 5.0,
            "signatureId": 42,
            "digestValue": "base64_encoded_hash_value",
            "useLocalTime": true,
            "randomizedDelay": 30
        }"#;

        // Deserialize back
        let deserialized: ChargingScheduleType = from_str(json).unwrap();

        // Verify OCPP 2.1 specific fields are preserved
        assert_eq!(deserialized.power_tolerance, Some(dec!(5.0)));
        assert_eq!(deserialized.signature_id, Some(42));
        assert_eq!(
            deserialized.digest_value,
            Some("base64_encoded_hash_value".to_string())
        );
        assert_eq!(deserialized.use_local_time, Some(true));
        assert_eq!(deserialized.randomized_delay, Some(30));
    }

    #[test]
    fn test_validation() {
        let period = create_test_period();

        // Valid schedule
        let valid_schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()]);
        assert!(
            valid_schedule.validate().is_ok(),
            "Valid schedule should pass validation"
        );

        // Test signature_id validation (negative value)
        let mut invalid_schedule = valid_schedule.clone();
        invalid_schedule.signature_id = Some(-1); // Invalid: must be >= 0
        assert!(
            invalid_schedule.validate().is_err(),
            "Schedule with negative signature_id should fail validation"
        );

        // Test digest_value validation (too long)
        let mut invalid_schedule = valid_schedule.clone();
        invalid_schedule.digest_value = Some("a".repeat(89)); // Invalid: exceeds max length of 88
        assert!(
            invalid_schedule.validate().is_err(),
            "Schedule with too long digest_value should fail validation"
        );

        // Test randomized_delay validation (negative value)
        let mut invalid_schedule = valid_schedule.clone();
        invalid_schedule.randomized_delay = Some(-10); // Invalid: must be >= 0
        assert!(
            invalid_schedule.validate().is_err(),
            "Schedule with negative randomized_delay should fail validation"
        );

        // Test charging_schedule_period validation (empty array)
        let mut invalid_schedule = valid_schedule.clone();
        invalid_schedule.charging_schedule_period = vec![]; // Invalid: must have at least 1 period
        assert!(
            invalid_schedule.validate().is_err(),
            "Schedule with empty charging_schedule_period should fail validation"
        );

        // Test charging_schedule_period validation (too many periods)
        let mut invalid_schedule = valid_schedule.clone();
        invalid_schedule.charging_schedule_period = vec![period.clone(); 1025]; // Invalid: exceeds max of 1024
        assert!(
            invalid_schedule.validate().is_err(),
            "Schedule with too many periods should fail validation"
        );
    }

    #[test]
    fn test_iso_15118_20_fields() {
        let mut period = create_test_period();
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()]);

        // Create time anchor for schedules
        let time_anchor = Utc::now();

        // Create mock price rule stack for AbsolutePriceScheduleType
        use crate::v2_1::datatypes::{
            PriceLevelScheduleEntryType, PriceRuleStackType, PriceRuleType,
        };

        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        let price_rule_stack = PriceRuleStackType::new(3600, vec![price_rule]);

        // Create AbsolutePriceScheduleType with required fields
        let absolute_price_schedule = AbsolutePriceScheduleType::new(
            time_anchor,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack],
        );

        // Create PriceLevelScheduleType with required fields
        let price_level_entries = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];

        let price_level_schedule = PriceLevelScheduleType::new(
            time_anchor,
            1, // Placeholder for price_schedule_id
            3, // Placeholder for number_of_price_levels
            price_level_entries,
        );

        // Create LimitAtSoCType with required fields
        let limit_at_soc = LimitAtSoCType::new(
            80,           // soc
            dec!(7500.0), // limit
        );

        // Set ISO 15118-20 related fields
        schedule.absolute_price_schedule = Some(absolute_price_schedule.clone());
        schedule.price_level_schedule = Some(price_level_schedule.clone());
        schedule.limit_at_so_c = Some(limit_at_soc.clone());

        // Verify fields are set correctly
        assert!(schedule.absolute_price_schedule.is_some());
        assert!(schedule.price_level_schedule.is_some());
        assert!(schedule.limit_at_so_c.is_some());

        // Verify specific field values
        assert_eq!(
            schedule
                .absolute_price_schedule
                .as_ref()
                .unwrap()
                .price_schedule_id,
            123
        );
        assert_eq!(
            schedule.absolute_price_schedule.as_ref().unwrap().currency,
            "USD"
        );
        assert_eq!(
            schedule
                .price_level_schedule
                .as_ref()
                .unwrap()
                .price_level_schedule_entries
                .len(),
            2
        );
        assert_eq!(schedule.limit_at_so_c.as_ref().unwrap().soc, 80);
        assert_eq!(schedule.limit_at_so_c.as_ref().unwrap().limit, dec!(7500.0));

        // Serialize to JSON
        let serialized = to_string(&schedule).unwrap();

        // Verify JSON contains ISO 15118-20 related fields
        assert!(serialized.contains(r#""absolutePriceSchedule"#));
        assert!(serialized.contains(r#""priceLevelSchedule"#));
        assert!(serialized.contains(r#""limitAtSoC"#));

        // Create a JSON string for deserialization with all required fields
        let json = format!(
            r#"{{
            "id": 1,
            "chargingRateUnit": "W",
            "chargingSchedulePeriod": [{{
                "startPeriod": 0,
                "limit": 16.0,
                "limit_L2": 16.0,
                "limit_L3": 16.0,
                "dischargeLimit": -10.0,
                "dischargeLimit_L2": -10.0,
                "dischargeLimit_L3": -10.0,
                "setpoint": 20.0,
                "setpoint_L2": 21.0,
                "setpoint_L3": 22.0,
                "setpointReactive": 5.0,
                "setpointReactive_L2": 6.0,
                "setpointReactive_L3": 7.0,
                "v2xBaseline": 50.0
            }}],
            "absolutePriceSchedule": {{
                "timeAnchor": "{}",
                "priceScheduleID": 123,
                "currency": "USD",
                "language": "en",
                "priceAlgorithm": "urn:algorithm:energy-fee:1.0",
                "priceRuleStacks": [{{
                    "duration": 3600,
                    "priceRules": [{{
                        "energyFee": {{"exponent": 2, "value": 25}},
                        "powerRangeStart": {{"exponent": 0, "value": 0}}
                    }}]
                }}]
            }},
            "priceLevelSchedule": {{
                "timeAnchor": "{}",
                "priceScheduleId": 1,
                "numberOfPriceLevels": 3,
                "priceLevelScheduleEntries": [
                    {{ "duration": 3600, "priceLevel": 1 }},
                    {{ "duration": 7200, "priceLevel": 2 }}
                ]
            }},
            "limitAtSoC": {{
                "soc": 80,
                "limit": 7500.0
            }},
            "minChargingRate": 5.0,
            "powerTolerance": 5.0
        }}"#,
            time_anchor.to_rfc3339(),
            time_anchor.to_rfc3339()
        );

        // Deserialize back
        let deserialized: ChargingScheduleType = from_str(&json).unwrap();

        // Verify ISO 15118-20 related fields are preserved
        assert!(deserialized.absolute_price_schedule.is_some());
        assert!(deserialized.price_level_schedule.is_some());
        assert!(deserialized.limit_at_so_c.is_some());

        // Verify specific field values are preserved
        assert_eq!(
            deserialized
                .absolute_price_schedule
                .as_ref()
                .unwrap()
                .price_schedule_id,
            123
        );
        assert_eq!(
            deserialized
                .absolute_price_schedule
                .as_ref()
                .unwrap()
                .currency,
            "USD"
        );
        assert_eq!(
            deserialized
                .price_level_schedule
                .as_ref()
                .unwrap()
                .price_level_schedule_entries
                .len(),
            2
        );
        assert_eq!(deserialized.limit_at_so_c.as_ref().unwrap().soc, 80);
        assert_eq!(
            deserialized.limit_at_so_c.as_ref().unwrap().limit,
            dec!(7500.0)
        );
    }

    #[test]
    fn test_min_charging_rate() {
        let mut period = create_test_period();
        period.limit_l2 = Some(dec!(16.0));
        period.limit_l3 = Some(dec!(16.0));
        period.discharge_limit = Some(dec!(-10.0));
        period.discharge_limit_l2 = Some(dec!(-10.0));
        period.discharge_limit_l3 = Some(dec!(-10.0));
        period.setpoint = Some(dec!(20.0));
        period.setpoint_l2 = Some(dec!(21.0));
        period.setpoint_l3 = Some(dec!(22.0));
        period.setpoint_reactive = Some(dec!(5.0));
        period.setpoint_reactive_l2 = Some(dec!(6.0));
        period.setpoint_reactive_l3 = Some(dec!(7.0));
        period.v2x_baseline = Some(dec!(50.0));

        let mut schedule =
            ChargingScheduleType::new(1, ChargingRateUnitEnumType::W, vec![period.clone()]);

        // Set min_charging_rate
        schedule.min_charging_rate = Some(dec!(5.0));

        // Verify field is set correctly
        assert_eq!(schedule.min_charging_rate, Some(dec!(5.0)));

        // Serialize to JSON
        let serialized = to_string(&schedule).unwrap();

        // Verify JSON contains min_charging_rate
        assert!(serialized.contains(r#""minChargingRate":5.0"#));

        // Create a JSON string for deserialization
        let json = r#"{
            "id": 1,
            "chargingRateUnit": "W",
            "chargingSchedulePeriod": [{
                "startPeriod": 0,
                "limit": 16.0,
                "limit_L2": 16.0,
                "limit_L3": 16.0,
                "dischargeLimit": -10.0,
                "dischargeLimit_L2": -10.0,
                "dischargeLimit_L3": -10.0,
                "setpoint": 20.0,
                "setpoint_L2": 21.0,
                "setpoint_L3": 22.0,
                "setpointReactive": 5.0,
                "setpointReactive_L2": 6.0,
                "setpointReactive_L3": 7.0,
                "v2xBaseline": 50.0
            }],
            "minChargingRate": 5.0,
            "powerTolerance": 5.0
        }"#;

        // Deserialize back
        let deserialized: ChargingScheduleType = from_str(json).unwrap();

        // Verify min_charging_rate is preserved
        assert_eq!(deserialized.min_charging_rate, Some(dec!(5.0)));
    }
}
