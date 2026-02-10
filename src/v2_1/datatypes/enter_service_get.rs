use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, enter_service::EnterServiceType};

/// Type for getting EnterService DER control function parameters.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EnterServiceGetType {
    /// The EnterService parameters.
    #[validate(nested)]
    pub enter_service: EnterServiceType,

    /// Id of setting.
    #[validate(length(max = 36))]
    pub id: String,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EnterServiceGetType {
    /// Creates a new `EnterServiceGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `enter_service` - The EnterService parameters
    /// * `id` - Id of setting
    ///
    /// # Returns
    ///
    /// A new instance of `EnterServiceGetType` with optional fields set to `None`
    pub fn new(enter_service: EnterServiceType, id: String) -> Self {
        Self {
            enter_service,
            id,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this enter service get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the EnterService parameters.
    ///
    /// # Returns
    ///
    /// A reference to the EnterService parameters
    pub fn enter_service(&self) -> &EnterServiceType {
        &self.enter_service
    }

    /// Sets the EnterService parameters.
    ///
    /// # Arguments
    ///
    /// * `enter_service` - The EnterService parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_enter_service(&mut self, enter_service: EnterServiceType) -> &mut Self {
        self.enter_service = enter_service;
        self
    }

    /// Gets the ID of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the ID of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
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
    /// * `custom_data` - Custom data for this enter service get, or None to clear
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
    fn test_new_enter_service_get() {
        use rust_decimal::prelude::*;

        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let enter_service = EnterServiceType::new(
            1,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );
        let id = "setting1".to_string();

        let enter_service_get = EnterServiceGetType::new(enter_service.clone(), id.clone());

        assert_eq!(enter_service_get.enter_service(), &enter_service);
        assert_eq!(enter_service_get.id(), id);
        assert_eq!(enter_service_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        use rust_decimal::prelude::*;

        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let enter_service = EnterServiceType::new(
            1,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );
        let id = "setting1".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let enter_service_get = EnterServiceGetType::new(enter_service.clone(), id.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(enter_service_get.enter_service(), &enter_service);
        assert_eq!(enter_service_get.id(), id);
        assert_eq!(enter_service_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        use rust_decimal::prelude::*;

        let high_voltage1 = Decimal::from_str("240.0").unwrap();
        let low_voltage1 = Decimal::from_str("220.0").unwrap();
        let high_freq1 = Decimal::from_str("60.5").unwrap();
        let low_freq1 = Decimal::from_str("59.5").unwrap();
        let delay1 = Decimal::from_str("5.0").unwrap();
        let random_delay1 = Decimal::from_str("2.0").unwrap();
        let ramp_rate1 = Decimal::from_str("10.0").unwrap();

        let high_voltage2 = Decimal::from_str("245.0").unwrap();
        let low_voltage2 = Decimal::from_str("215.0").unwrap();
        let high_freq2 = Decimal::from_str("61.0").unwrap();
        let low_freq2 = Decimal::from_str("59.0").unwrap();
        let delay2 = Decimal::from_str("6.0").unwrap();
        let random_delay2 = Decimal::from_str("3.0").unwrap();
        let ramp_rate2 = Decimal::from_str("12.0").unwrap();

        let enter_service1 = EnterServiceType::new(
            1,
            high_voltage1,
            low_voltage1,
            high_freq1,
            low_freq1,
            delay1,
            random_delay1,
            ramp_rate1,
        );
        let enter_service2 = EnterServiceType::new(
            2,
            high_voltage2,
            low_voltage2,
            high_freq2,
            low_freq2,
            delay2,
            random_delay2,
            ramp_rate2,
        );
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut enter_service_get = EnterServiceGetType::new(enter_service1.clone(), id1.clone());

        enter_service_get
            .set_enter_service(enter_service2.clone())
            .set_id(id2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(enter_service_get.enter_service(), &enter_service2);
        assert_eq!(enter_service_get.id(), id2);
        assert_eq!(enter_service_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        enter_service_get.set_custom_data(None);
        assert_eq!(enter_service_get.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        use rust_decimal::prelude::*;

        // 创建有效的EnterServiceGetType实例
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let enter_service = EnterServiceType::new(
            1,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );
        let id = "valid_id".to_string();

        let valid_enter_service_get = EnterServiceGetType::new(enter_service.clone(), id.clone());

        // 验证有效实例应该通过
        assert!(valid_enter_service_get.validate().is_ok());

        // 测试ID长度超过限制的情况
        let long_id = "a".repeat(37); // 创建一个37字符长的ID，超过了36的限制
        let invalid_id_enter_service_get = EnterServiceGetType::new(enter_service.clone(), long_id);

        // 验证应该失败，因为ID太长
        let validation_result = invalid_id_enter_service_get.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("id"));
        assert!(error_message.contains("length"));

        // 测试嵌套验证 - enter_service中的priority为负数
        let invalid_enter_service = EnterServiceType::new(
            -1,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        );
        let enter_service_get_with_invalid_enter_service =
            EnterServiceGetType::new(invalid_enter_service, id.clone());

        // 验证应该失败，因为enter_service中的priority为负数
        let validation_result = enter_service_get_with_invalid_enter_service.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("enter_service"));
        assert!(error_message.contains("priority"));

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let enter_service_get_with_invalid_custom_data = EnterServiceGetType {
            enter_service: enter_service.clone(),
            id: id.clone(),
            custom_data: Some(invalid_custom_data),
        };

        // 验证应该失败，因为custom_data无效
        let validation_result = enter_service_get_with_invalid_custom_data.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("custom_data"));
    }

    #[test]
    fn test_serialization() {
        use rust_decimal::prelude::*;
        use serde_json::{json, Value};

        // 创建测试数据
        let high_voltage = Decimal::from_str("240.0").unwrap();
        let low_voltage = Decimal::from_str("220.0").unwrap();
        let high_freq = Decimal::from_str("60.5").unwrap();
        let low_freq = Decimal::from_str("59.5").unwrap();
        let delay = Decimal::from_str("5.0").unwrap();
        let random_delay = Decimal::from_str("2.0").unwrap();
        let ramp_rate = Decimal::from_str("10.0").unwrap();

        let enter_service = EnterServiceType::new(
            1,
            high_voltage,
            low_voltage,
            high_freq,
            low_freq,
            delay,
            random_delay,
            ramp_rate,
        )
        .with_custom_data(CustomDataType::new("VendorX".to_string()));
        let id = "setting1".to_string();
        let custom_data = CustomDataType::new("VendorY".to_string())
            .with_property("version".to_string(), json!("1.0"));

        // 创建完整的EnterServiceGetType实例
        let enter_service_get = EnterServiceGetType {
            enter_service,
            id,
            custom_data: Some(custom_data),
        };

        // 序列化为JSON
        let serialized = serde_json::to_string(&enter_service_get).unwrap();

        // 反序列化并验证
        let deserialized: EnterServiceGetType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(enter_service_get, deserialized);

        // 验证JSON结构
        let json_value: Value = serde_json::from_str(&serialized).unwrap();
        assert!(json_value.is_object());
        assert!(json_value.get("enterService").is_some());
        assert!(json_value.get("id").is_some());
        assert!(json_value.get("customData").is_some());
    }
}
