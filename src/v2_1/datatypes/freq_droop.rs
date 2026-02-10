use super::custom_data::CustomDataType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Frequency droop settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Over-frequency start of droop
    #[serde(with = "rust_decimal::serde::arbitrary_precision", rename = "overFreq")]
    pub over_freq: Decimal,

    /// Under-frequency start of droop
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision",
        rename = "underFreq"
    )]
    pub under_freq: Decimal,

    /// Over-frequency droop per unit, oFDroop
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision",
        rename = "overDroop"
    )]
    pub over_droop: Decimal,

    /// Under-frequency droop per unit, uFDroop
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision",
        rename = "underDroop"
    )]
    pub under_droop: Decimal,

    /// Response time in seconds.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub response_time: Decimal,

    /// Time when this setting becomes active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Duration in seconds that this setting is active
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

impl FreqDroopType {
    /// Creates a new `FreqDroopType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `over_freq` - Over-frequency start of droop
    /// * `under_freq` - Under-frequency start of droop
    /// * `over_droop` - Over-frequency droop per unit, oFDroop
    /// * `under_droop` - Under-frequency droop per unit, uFDroop
    /// * `response_time` - Response time in seconds
    ///
    /// # Returns
    ///
    /// A new instance of `FreqDroopType` with optional fields set to `None`
    pub fn new(
        priority: i32,
        over_freq: Decimal,
        under_freq: Decimal,
        over_droop: Decimal,
        under_droop: Decimal,
        response_time: Decimal,
    ) -> Self {
        Self {
            priority,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
            start_time: None,
            duration: None,
            custom_data: None,
        }
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
    /// * `duration` - Duration in seconds that this setting is active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_duration(mut self, duration: Decimal) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these frequency droop settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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

    /// Gets the over-frequency start of droop.
    ///
    /// # Returns
    ///
    /// The over-frequency start of droop
    pub fn over_freq(&self) -> Decimal {
        self.over_freq
    }

    /// Sets the over-frequency start of droop.
    ///
    /// # Arguments
    ///
    /// * `over_freq` - Over-frequency start of droop
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_over_freq(&mut self, over_freq: Decimal) -> &mut Self {
        self.over_freq = over_freq;
        self
    }

    /// Gets the under-frequency start of droop.
    ///
    /// # Returns
    ///
    /// The under-frequency start of droop
    pub fn under_freq(&self) -> Decimal {
        self.under_freq
    }

    /// Sets the under-frequency start of droop.
    ///
    /// # Arguments
    ///
    /// * `under_freq` - Under-frequency start of droop
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_under_freq(&mut self, under_freq: Decimal) -> &mut Self {
        self.under_freq = under_freq;
        self
    }

    /// Gets the over-frequency droop per unit.
    ///
    /// # Returns
    ///
    /// The over-frequency droop per unit, oFDroop
    pub fn over_droop(&self) -> Decimal {
        self.over_droop
    }

    /// Sets the over-frequency droop per unit.
    ///
    /// # Arguments
    ///
    /// * `over_droop` - Over-frequency droop per unit, oFDroop
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_over_droop(&mut self, over_droop: Decimal) -> &mut Self {
        self.over_droop = over_droop;
        self
    }

    /// Gets the under-frequency droop per unit.
    ///
    /// # Returns
    ///
    /// The under-frequency droop per unit, uFDroop
    pub fn under_droop(&self) -> Decimal {
        self.under_droop
    }

    /// Sets the under-frequency droop per unit.
    ///
    /// # Arguments
    ///
    /// * `under_droop` - Under-frequency droop per unit, uFDroop
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_under_droop(&mut self, under_droop: Decimal) -> &mut Self {
        self.under_droop = under_droop;
        self
    }

    /// Gets the response time.
    ///
    /// # Returns
    ///
    /// The response time in seconds
    pub fn response_time(&self) -> Decimal {
        self.response_time
    }

    /// Sets the response time.
    ///
    /// # Arguments
    ///
    /// * `response_time` - Response time in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_response_time(&mut self, response_time: Decimal) -> &mut Self {
        self.response_time = response_time;
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
    /// An optional reference to the duration in seconds that this setting is active
    pub fn duration(&self) -> Option<Decimal> {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration in seconds that this setting is active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: Option<Decimal>) -> &mut Self {
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
    /// * `custom_data` - Custom data for these frequency droop settings, or None to clear
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
    use chrono::TimeZone;
    use rust_decimal::prelude::FromStr;

    #[test]
    fn test_new_freq_droop() {
        let priority = 1;
        let over_freq = Decimal::from_str("50.5").unwrap();
        let under_freq = Decimal::from_str("49.5").unwrap();
        let over_droop = Decimal::from_str("0.04").unwrap();
        let under_droop = Decimal::from_str("0.04").unwrap();
        let response_time = Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            priority,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );

        assert_eq!(freq_droop.priority(), priority);
        assert_eq!(freq_droop.over_freq(), over_freq);
        assert_eq!(freq_droop.under_freq(), under_freq);
        assert_eq!(freq_droop.over_droop(), over_droop);
        assert_eq!(freq_droop.under_droop(), under_droop);
        assert_eq!(freq_droop.response_time(), response_time);
        assert_eq!(freq_droop.start_time(), None);
        assert_eq!(freq_droop.duration(), None);
        assert_eq!(freq_droop.custom_data(), None);
    }

    #[test]
    fn test_with_optional_fields() {
        let priority = 1;
        let over_freq = Decimal::from_str("50.5").unwrap();
        let under_freq = Decimal::from_str("49.5").unwrap();
        let over_droop = Decimal::from_str("0.04").unwrap();
        let under_droop = Decimal::from_str("0.04").unwrap();
        let response_time = Decimal::from_str("2.0").unwrap();
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let duration = Decimal::from_str("3600").unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let freq_droop = FreqDroopType::new(
            priority,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        )
        .with_start_time(start_time)
        .with_duration(duration)
        .with_custom_data(custom_data.clone());

        assert_eq!(freq_droop.priority(), priority);
        assert_eq!(freq_droop.over_freq(), over_freq);
        assert_eq!(freq_droop.under_freq(), under_freq);
        assert_eq!(freq_droop.over_droop(), over_droop);
        assert_eq!(freq_droop.under_droop(), under_droop);
        assert_eq!(freq_droop.response_time(), response_time);
        assert_eq!(freq_droop.start_time(), Some(&start_time));
        assert_eq!(freq_droop.duration(), Some(duration));
        assert_eq!(freq_droop.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let over_freq1 = Decimal::from_str("50.5").unwrap();
        let under_freq1 = Decimal::from_str("49.5").unwrap();
        let over_droop1 = Decimal::from_str("0.04").unwrap();
        let under_droop1 = Decimal::from_str("0.04").unwrap();
        let response_time1 = Decimal::from_str("2.0").unwrap();

        let priority2 = 2;
        let over_freq2 = Decimal::from_str("50.6").unwrap();
        let under_freq2 = Decimal::from_str("49.4").unwrap();
        let over_droop2 = Decimal::from_str("0.05").unwrap();
        let under_droop2 = Decimal::from_str("0.05").unwrap();
        let response_time2 = Decimal::from_str("3.0").unwrap();
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let duration = Decimal::from_str("3600").unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut freq_droop = FreqDroopType::new(
            priority1,
            over_freq1,
            under_freq1,
            over_droop1,
            under_droop1,
            response_time1,
        );

        freq_droop
            .set_priority(priority2)
            .set_over_freq(over_freq2)
            .set_under_freq(under_freq2)
            .set_over_droop(over_droop2)
            .set_under_droop(under_droop2)
            .set_response_time(response_time2)
            .set_start_time(Some(start_time))
            .set_duration(Some(duration))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(freq_droop.priority(), priority2);
        assert_eq!(freq_droop.over_freq(), over_freq2);
        assert_eq!(freq_droop.under_freq(), under_freq2);
        assert_eq!(freq_droop.over_droop(), over_droop2);
        assert_eq!(freq_droop.under_droop(), under_droop2);
        assert_eq!(freq_droop.response_time(), response_time2);
        assert_eq!(freq_droop.start_time(), Some(&start_time));
        assert_eq!(freq_droop.duration(), Some(duration));
        assert_eq!(freq_droop.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        freq_droop
            .set_start_time(None)
            .set_duration(None)
            .set_custom_data(None);
        assert_eq!(freq_droop.start_time(), None);
        assert_eq!(freq_droop.duration(), None);
        assert_eq!(freq_droop.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        let priority = 1;
        let over_freq = Decimal::from_str("50.5").unwrap();
        let under_freq = Decimal::from_str("49.5").unwrap();
        let over_droop = Decimal::from_str("0.04").unwrap();
        let under_droop = Decimal::from_str("0.04").unwrap();
        let response_time = Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            priority,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );

        // 验证有效实例应该通过
        assert!(freq_droop.validate().is_ok());

        // 测试无效的优先级（负数）
        let mut invalid_freq_droop = freq_droop.clone();
        invalid_freq_droop.priority = -1;
        assert!(invalid_freq_droop.validate().is_err());

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let mut freq_droop_with_invalid_custom_data = freq_droop.clone();
        freq_droop_with_invalid_custom_data.custom_data = Some(invalid_custom_data);
        assert!(freq_droop_with_invalid_custom_data.validate().is_err());
    }
}
