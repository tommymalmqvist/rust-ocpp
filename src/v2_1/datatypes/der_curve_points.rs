use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Points defining a DER curve.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurvePointsType {
    /// The data value of the X-axis (independent) variable, depending on the curve type.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub x: Decimal,

    /// The data value of the Y-axis (dependent) variable, depending on the DERUnitEnumType of the curve.
    /// If y is power factor, then a positive value means DER is absorbing reactive power (under-excited),
    /// a negative value when DER is injecting reactive power (over-excited).
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub y: Decimal,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DERCurvePointsType {
    /// Creates a new `DERCurvePointsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `x` - The data value of the X-axis (independent) variable
    /// * `y` - The data value of the Y-axis (dependent) variable
    ///
    /// # Returns
    ///
    /// A new instance of `DERCurvePointsType` with optional fields set to `None`
    pub fn new(x: Decimal, y: Decimal) -> Self {
        Self {
            x,
            y,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this curve point
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the X-axis value.
    ///
    /// # Returns
    ///
    /// The X-axis value
    pub fn x(&self) -> Decimal {
        self.x
    }

    /// Sets the X-axis value.
    ///
    /// # Arguments
    ///
    /// * `x` - The data value of the X-axis (independent) variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_x(&mut self, x: Decimal) -> &mut Self {
        self.x = x;
        self
    }

    /// Gets the Y-axis value.
    ///
    /// # Returns
    ///
    /// The Y-axis value
    pub fn y(&self) -> Decimal {
        self.y
    }

    /// Sets the Y-axis value.
    ///
    /// # Arguments
    ///
    /// * `y` - The data value of the Y-axis (dependent) variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_y(&mut self, y: Decimal) -> &mut Self {
        self.y = y;
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
    /// * `custom_data` - Custom data for this curve point, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

impl Default for DERCurvePointsType {
    fn default() -> Self {
        Self {
            x: Decimal::ZERO,
            y: Decimal::ZERO,
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_der_curve_points() {
        use rust_decimal::prelude::*;

        let x = Decimal::from_str("10.5").unwrap();
        let y = Decimal::from_str("20.3").unwrap();
        let point = DERCurvePointsType::new(x, y);

        assert_eq!(point.x(), x);
        assert_eq!(point.y(), y);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        use rust_decimal::prelude::*;

        let custom_data = CustomDataType::new("VendorX".to_string());
        let x = Decimal::from_str("10.5").unwrap();
        let y = Decimal::from_str("20.3").unwrap();
        let point = DERCurvePointsType {
            x,
            y,
            custom_data: Some(custom_data.clone()),
        };

        assert_eq!(point.x(), x);
        assert_eq!(point.y(), y);
        assert_eq!(point.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        use rust_decimal::prelude::*;

        let custom_data = CustomDataType::new("VendorX".to_string());
        let x1 = Decimal::from_str("10.5").unwrap();
        let y1 = Decimal::from_str("20.3").unwrap();
        let x2 = Decimal::from_str("15.7").unwrap();
        let y2 = Decimal::from_str("25.9").unwrap();

        let mut point = DERCurvePointsType {
            x: x1,
            y: y1,
            custom_data: None,
        };

        point
            .set_x(x2)
            .set_y(y2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(point.x(), x2);
        assert_eq!(point.y(), y2);
        assert_eq!(point.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        point.set_custom_data(None);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_default() {
        let point = DERCurvePointsType::default();

        assert_eq!(point.x(), Decimal::ZERO);
        assert_eq!(point.y(), Decimal::ZERO);
        assert_eq!(point.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        use rust_decimal::prelude::*;

        // 创建有效的DERCurvePointsType实例
        let x = Decimal::from_str("10.5").unwrap();
        let y = Decimal::from_str("20.3").unwrap();
        let valid_point = DERCurvePointsType {
            x,
            y,
            custom_data: None,
        };

        // 验证有效实例应该通过
        assert!(valid_point.validate().is_ok());

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let point_with_invalid_custom_data = DERCurvePointsType {
            x,
            y,
            custom_data: Some(invalid_custom_data),
        };

        // 验证应该失败，因为custom_data无效
        let validation_result = point_with_invalid_custom_data.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("custom_data"));
    }

    #[test]
    fn test_serialization() {
        use rust_decimal::prelude::*;
        use serde_json::{json, Value};

        // 创建测试数据
        let x = Decimal::from_str("10.5").unwrap();
        let y = Decimal::from_str("-20.3").unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        // 创建DERCurvePointsType实例
        let point = DERCurvePointsType {
            x,
            y,
            custom_data: Some(custom_data),
        };

        // 序列化为JSON
        let serialized = serde_json::to_string(&point).unwrap();

        // 反序列化并验证
        let deserialized: DERCurvePointsType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(point, deserialized);
        assert_eq!(deserialized.x, x);
        assert_eq!(deserialized.y, y);

        // 验证JSON结构
        let json_value: Value = serde_json::from_str(&serialized).unwrap();
        assert!(json_value.is_object());
        assert!(json_value.get("x").is_some());
        assert!(json_value.get("y").is_some());
        assert!(json_value.get("customData").is_some());

        // 测试没有自定义数据的情况
        let point_without_custom_data = DERCurvePointsType {
            x,
            y,
            custom_data: None,
        };

        let serialized = serde_json::to_string(&point_without_custom_data).unwrap();
        let json_value: Value = serde_json::from_str(&serialized).unwrap();

        assert!(json_value.is_object());
        assert!(json_value.get("x").is_some());
        assert!(json_value.get("y").is_some());
        assert!(json_value.get("customData").is_none());
    }

    #[test]
    fn test_decimal_precision() {
        use rust_decimal::prelude::*;

        // 测试高精度小数
        let x = Decimal::from_str("123456789.123456789").unwrap();
        let y = Decimal::from_str("-987654321.987654321").unwrap();

        let point = DERCurvePointsType {
            x,
            y,
            custom_data: None,
        };

        // 序列化为JSON
        let serialized = serde_json::to_string(&point).unwrap();

        // 反序列化并验证精度保持不变
        let deserialized: DERCurvePointsType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized.x, x);
        assert_eq!(deserialized.y, y);
    }
}
