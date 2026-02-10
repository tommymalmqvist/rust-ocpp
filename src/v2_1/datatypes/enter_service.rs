use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Parameters for the EnterService DER control function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// Enter service voltage high
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub high_voltage: Decimal,

    /// Enter service voltage low
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub low_voltage: Decimal,

    /// Enter service frequency high
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub high_freq: Decimal,

    /// Enter service frequency low
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub low_freq: Decimal,

    /// Enter service delay
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub delay: Option<Decimal>,

    /// Enter service randomized delay
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub random_delay: Option<Decimal>,

    /// Enter service ramp rate in seconds
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub ramp_rate: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EnterServiceType {
    /// Creates a new `EnterServiceType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `high_voltage` - Enter service voltage high
    /// * `low_voltage` - Enter service voltage low
    /// * `high_freq` - Enter service frequency high
    /// * `low_freq` - Enter service frequency low
    /// * `delay` - Enter service delay
    /// * `random_delay` - Enter service randomized delay
    /// * `ramp_rate` - Enter service ramp rate in seconds
    ///
    /// # Returns
    ///
    /// A new instance of `EnterServiceType` with optional fields set to `None`
    pub fn new(
        priority: i32,
        high_voltage: Decimal,
        low_voltage: Decimal,
        high_freq: Decimal,
        low_freq: Decimal,
        delay: Decimal,
        random_delay: Decimal,
        ramp_rate: Decimal,
    ) -> Self {
        Self {
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay: Some(delay),
            random_delay: Some(random_delay),
            ramp_rate: Some(ramp_rate),
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these enter service parameters
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

    /// Gets the high voltage.
    ///
    /// # Returns
    ///
    /// The enter service voltage high
    pub fn high_voltage(&self) -> Decimal {
        self.high_voltage
    }

    /// Sets the high voltage.
    ///
    /// # Arguments
    ///
    /// * `high_voltage` - Enter service voltage high
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_high_voltage(&mut self, high_voltage: Decimal) -> &mut Self {
        self.high_voltage = high_voltage;
        self
    }

    /// Gets the low voltage.
    ///
    /// # Returns
    ///
    /// The enter service voltage low
    pub fn low_voltage(&self) -> Decimal {
        self.low_voltage
    }

    /// Sets the low voltage.
    ///
    /// # Arguments
    ///
    /// * `low_voltage` - Enter service voltage low
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_low_voltage(&mut self, low_voltage: Decimal) -> &mut Self {
        self.low_voltage = low_voltage;
        self
    }

    /// Gets the high frequency.
    ///
    /// # Returns
    ///
    /// The enter service frequency high
    pub fn high_freq(&self) -> Decimal {
        self.high_freq
    }

    /// Sets the high frequency.
    ///
    /// # Arguments
    ///
    /// * `high_freq` - Enter service frequency high
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_high_freq(&mut self, high_freq: Decimal) -> &mut Self {
        self.high_freq = high_freq;
        self
    }

    /// Gets the low frequency.
    ///
    /// # Returns
    ///
    /// The enter service frequency low
    pub fn low_freq(&self) -> Decimal {
        self.low_freq
    }

    /// Sets the low frequency.
    ///
    /// # Arguments
    ///
    /// * `low_freq` - Enter service frequency low
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_low_freq(&mut self, low_freq: Decimal) -> &mut Self {
        self.low_freq = low_freq;
        self
    }

    /// Gets the delay.
    ///
    /// # Returns
    ///
    /// The enter service delay
    pub fn delay(&self) -> Option<Decimal> {
        self.delay
    }

    /// Sets the delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - Enter service delay
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_delay(&mut self, delay: Option<Decimal>) -> &mut Self {
        self.delay = delay;
        self
    }

    /// Gets the random delay.
    ///
    /// # Returns
    ///
    /// The enter service randomized delay
    pub fn random_delay(&self) -> Option<Decimal> {
        self.random_delay
    }

    /// Sets the random delay.
    ///
    /// # Arguments
    ///
    /// * `random_delay` - Enter service randomized delay
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_random_delay(&mut self, random_delay: Option<Decimal>) -> &mut Self {
        self.random_delay = random_delay;
        self
    }

    /// Gets the ramp rate.
    ///
    /// # Returns
    ///
    /// The enter service ramp rate in seconds
    pub fn ramp_rate(&self) -> Option<Decimal> {
        self.ramp_rate
    }

    /// Sets the ramp rate.
    ///
    /// # Arguments
    ///
    /// * `ramp_rate` - Enter service ramp rate in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ramp_rate(&mut self, ramp_rate: Option<Decimal>) -> &mut Self {
        self.ramp_rate = ramp_rate;
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
    /// * `custom_data` - Custom data for these enter service parameters, or None to clear
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
    use rust_decimal::prelude::*;

    #[test]
    fn test_new_enter_service() {
        let priority = 1;
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let enter_service = EnterServiceType::new(
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );

        assert_eq!(enter_service.priority(), priority);
        assert_eq!(enter_service.high_voltage(), high_voltage);
        assert_eq!(enter_service.low_voltage(), low_voltage);
        assert_eq!(enter_service.high_freq(), high_freq);
        assert_eq!(enter_service.low_freq(), low_freq);
        assert_eq!(enter_service.delay(), Some(delay));
        assert_eq!(enter_service.random_delay(), Some(random_delay));
        assert_eq!(enter_service.ramp_rate(), Some(ramp_rate));
        assert_eq!(enter_service.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let enter_service = EnterServiceType::new(
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(enter_service.priority(), priority);
        assert_eq!(enter_service.high_voltage(), high_voltage);
        assert_eq!(enter_service.low_voltage(), low_voltage);
        assert_eq!(enter_service.high_freq(), high_freq);
        assert_eq!(enter_service.low_freq(), low_freq);
        assert_eq!(enter_service.delay(), Some(delay));
        assert_eq!(enter_service.random_delay(), Some(random_delay));
        assert_eq!(enter_service.ramp_rate(), Some(ramp_rate));
        assert_eq!(enter_service.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let high_voltage1 = Decimal::from_str("240.0").unwrap();
        let low_voltage1 = Decimal::from_str("220.0").unwrap();
        let high_freq1 = Decimal::from_str("60.5").unwrap();
        let low_freq1 = Decimal::from_str("59.5").unwrap();
        let delay1 = Decimal::from_str("5.0").unwrap();
        let random_delay1 = Decimal::from_str("2.0").unwrap();
        let ramp_rate1 = Decimal::from_str("10.0").unwrap();

        let priority2 = 2;
        let high_voltage2 = Decimal::from_str("245.0").unwrap();
        let low_voltage2 = Decimal::from_str("215.0").unwrap();
        let high_freq2 = Decimal::from_str("61.0").unwrap();
        let low_freq2 = Decimal::from_str("59.0").unwrap();
        let delay2 = Decimal::from_str("6.0").unwrap();
        let random_delay2 = Decimal::from_str("3.0").unwrap();
        let ramp_rate2 = Decimal::from_str("12.0").unwrap();

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut enter_service = EnterServiceType::new(
            priority1,
            high_voltage1,
            low_voltage1,
            high_freq1,
            low_freq1,
            delay1,
            random_delay1,
            ramp_rate1,
        );

        enter_service
            .set_priority(priority2)
            .set_high_voltage(high_voltage2)
            .set_low_voltage(low_voltage2)
            .set_high_freq(high_freq2)
            .set_low_freq(low_freq2)
            .set_delay(Some(delay2))
            .set_random_delay(Some(random_delay2))
            .set_ramp_rate(Some(ramp_rate2))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(enter_service.priority(), priority2);
        assert_eq!(enter_service.high_voltage(), high_voltage2);
        assert_eq!(enter_service.low_voltage(), low_voltage2);
        assert_eq!(enter_service.high_freq(), high_freq2);
        assert_eq!(enter_service.low_freq(), low_freq2);
        assert_eq!(enter_service.delay(), Some(delay2));
        assert_eq!(enter_service.random_delay(), Some(random_delay2));
        assert_eq!(enter_service.ramp_rate(), Some(ramp_rate2));
        assert_eq!(enter_service.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        enter_service.set_custom_data(None);
        assert_eq!(enter_service.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        // 创建有效的EnterServiceType实例
        let priority = 1;
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let valid_enter_service = EnterServiceType::new(
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );

        // 验证有效实例应该通过
        assert!(valid_enter_service.validate().is_ok());

        // 测试priority为负数的情况
        let negative_priority = -1;
        let invalid_priority_enter_service = EnterServiceType::new(
            negative_priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );

        // 验证应该失败，因为priority为负数
        let validation_result = invalid_priority_enter_service.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("priority"));
        assert!(error_message.contains("range"));

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let enter_service_with_invalid_custom_data = EnterServiceType {
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay: Some(delay),
            random_delay: Some(random_delay),
            ramp_rate: Some(ramp_rate),
            custom_data: Some(invalid_custom_data),
        };

        // 验证应该失败，因为custom_data无效
        let validation_result = enter_service_with_invalid_custom_data.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("custom_data"));
    }

    #[test]
    fn test_serialization() {
        use serde_json::{json, Value};

        // 创建测试数据
        let priority = 1;
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        // 创建完整的EnterServiceType实例
        let enter_service = EnterServiceType {
            priority,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay: Some(delay),
            random_delay: Some(random_delay),
            ramp_rate: Some(ramp_rate),
            custom_data: Some(custom_data),
        };

        // 序列化为JSON
        let serialized = serde_json::to_string(&enter_service).unwrap();

        // 反序列化并验证
        let deserialized: EnterServiceType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(enter_service, deserialized);

        // 验证JSON结构
        let json_value: Value = serde_json::from_str(&serialized).unwrap();
        assert!(json_value.is_object());
        assert!(json_value.get("priority").is_some());
        assert!(json_value.get("highVoltage").is_some());
        assert!(json_value.get("lowVoltage").is_some());
        assert!(json_value.get("highFreq").is_some());
        assert!(json_value.get("lowFreq").is_some());
        assert!(json_value.get("delay").is_some());
        assert!(json_value.get("randomDelay").is_some());
        assert!(json_value.get("rampRate").is_some());
        assert!(json_value.get("customData").is_some());

        // 验证Decimal类型的精度保持
        assert!(json_value.get("highVoltage").is_some());

        // 验证Optional<Decimal>字段存在
        assert!(json_value.get("delay").is_some());
        assert!(json_value.get("randomDelay").is_some());
        assert!(json_value.get("rampRate").is_some());
    }

    #[test]
    fn test_decimal_precision() {
        // 测试高精度小数
        let priority = 1;
        let high_voltage = Decimal::from_str("240.123456789").unwrap();
        let low_voltage = Decimal::from_str("220.987654321").unwrap();
        let high_freq = Decimal::from_str("60.555555555").unwrap();
        let low_freq = Decimal::from_str("59.444444444").unwrap();
        let delay = Decimal::from_str("5.111111111").unwrap();
        let random_delay = Decimal::from_str("2.222222222").unwrap();
        let ramp_rate = Decimal::from_str("10.333333333").unwrap();

        let enter_service = EnterServiceType::new(
            priority,
            high_voltage.clone(),
            low_voltage.clone(),
            high_freq.clone(),
            low_freq.clone(),
            delay.clone(),
            random_delay.clone(),
            ramp_rate.clone(),
        );

        // 序列化为JSON
        let serialized = serde_json::to_string(&enter_service).unwrap();

        // 反序列化并验证精度保持不变
        let deserialized: EnterServiceType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.high_voltage(), high_voltage);
        assert_eq!(deserialized.low_voltage(), low_voltage);
        assert_eq!(deserialized.high_freq(), high_freq);
        assert_eq!(deserialized.low_freq(), low_freq);
        assert_eq!(deserialized.delay(), Some(delay));
        assert_eq!(deserialized.random_delay(), Some(random_delay));
        assert_eq!(deserialized.ramp_rate(), Some(ramp_rate));
    }
}
