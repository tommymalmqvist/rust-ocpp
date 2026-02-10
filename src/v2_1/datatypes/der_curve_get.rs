use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, der_curve::DERCurveType};
use crate::v2_1::enumerations::der_control::DERControlEnumType;

/// DER curve get type for retrieving DER curve information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DERCurveGetType {
    /// The DER curve.
    #[validate(nested)]
    pub curve: DERCurveType,

    /// Id of DER curve.
    #[validate(length(max = 36))]
    pub id: String,

    /// Type of DER curve.
    pub curve_type: DERControlEnumType,

    /// True if this is a default curve.
    pub is_default: bool,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl DERCurveGetType {
    /// Creates a new `DERCurveGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `curve` - The DER curve
    /// * `id` - Id of DER curve
    /// * `curve_type` - Type of DER curve
    /// * `is_default` - True if this is a default curve
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// A new instance of `DERCurveGetType` with optional fields set to `None`
    pub fn new(
        curve: DERCurveType,
        id: String,
        curve_type: DERControlEnumType,
        is_default: bool,
        is_superseded: bool,
    ) -> Self {
        Self {
            curve,
            id,
            curve_type,
            is_default,
            is_superseded,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this curve get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the DER curve.
    ///
    /// # Returns
    ///
    /// A reference to the DER curve
    pub fn curve(&self) -> &DERCurveType {
        &self.curve
    }

    /// Sets the DER curve.
    ///
    /// # Arguments
    ///
    /// * `curve` - The DER curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_curve(&mut self, curve: DERCurveType) -> &mut Self {
        self.curve = curve;
        self
    }

    /// Gets the ID of the DER curve.
    ///
    /// # Returns
    ///
    /// A reference to the ID of the DER curve
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the DER curve.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of DER curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the type of the DER curve.
    ///
    /// # Returns
    ///
    /// The type of the DER curve
    pub fn curve_type(&self) -> DERControlEnumType {
        self.curve_type.clone()
    }

    /// Sets the type of the DER curve.
    ///
    /// # Arguments
    ///
    /// * `curve_type` - Type of DER curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_curve_type(&mut self, curve_type: DERControlEnumType) -> &mut Self {
        self.curve_type = curve_type;
        self
    }

    /// Gets whether this is a default curve.
    ///
    /// # Returns
    ///
    /// True if this is a default curve
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Sets whether this is a default curve.
    ///
    /// # Arguments
    ///
    /// * `is_default` - True if this is a default curve
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = is_default;
        self
    }

    /// Gets whether this setting is superseded.
    ///
    /// # Returns
    ///
    /// True if this setting is superseded by a higher priority setting
    pub fn is_superseded(&self) -> bool {
        self.is_superseded
    }

    /// Sets whether this setting is superseded.
    ///
    /// # Arguments
    ///
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_superseded(&mut self, is_superseded: bool) -> &mut Self {
        self.is_superseded = is_superseded;
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
    /// * `custom_data` - Custom data for this curve get, or None to clear
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
    use crate::v2_1::datatypes::der_curve_points::DERCurvePointsType;
    use crate::v2_1::enumerations::der_unit::DERUnitEnumType;

    #[test]
    fn test_new_der_curve_get() {
        let curve_points = vec![DERCurvePointsType::default()];
        let curve = DERCurveType::new(curve_points, 1, DERUnitEnumType::PctMaxW);
        let id = "curve1".to_string();
        let curve_type = DERControlEnumType::FreqDroop;
        let is_default = true;
        let is_superseded = false;

        let curve_get = DERCurveGetType::new(
            curve.clone(),
            id.clone(),
            curve_type.clone(),
            is_default,
            is_superseded,
        );

        assert_eq!(curve_get.curve(), &curve);
        assert_eq!(curve_get.id(), id);
        assert_eq!(curve_get.curve_type(), curve_type);
        assert_eq!(curve_get.is_default(), is_default);
        assert_eq!(curve_get.is_superseded(), is_superseded);
        assert_eq!(curve_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let curve_points = vec![DERCurvePointsType::default()];
        let curve = DERCurveType::new(curve_points, 1, DERUnitEnumType::PctMaxW);
        let id = "curve1".to_string();
        let curve_type = DERControlEnumType::FreqDroop;
        let is_default = true;
        let is_superseded = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let curve_get = DERCurveGetType::new(
            curve.clone(),
            id.clone(),
            curve_type.clone(),
            is_default,
            is_superseded,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(curve_get.curve(), &curve);
        assert_eq!(curve_get.id(), id);
        assert_eq!(curve_get.curve_type(), curve_type);
        assert_eq!(curve_get.is_default(), is_default);
        assert_eq!(curve_get.is_superseded(), is_superseded);
        assert_eq!(curve_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        use rust_decimal::prelude::*;

        let curve_points1 = vec![DERCurvePointsType::default()];
        let curve1 = DERCurveType::new(curve_points1, 1, DERUnitEnumType::PctMaxW);
        let curve_points2 = vec![
            DERCurvePointsType::new(
                Decimal::from_str("1.0").unwrap(),
                Decimal::from_str("2.0").unwrap(),
            ),
            DERCurvePointsType::new(
                Decimal::from_str("3.0").unwrap(),
                Decimal::from_str("4.0").unwrap(),
            ),
        ];
        let curve2 = DERCurveType::new(curve_points2, 2, DERUnitEnumType::PctMaxVar);
        let id1 = "curve1".to_string();
        let id2 = "curve2".to_string();
        let curve_type1 = DERControlEnumType::FreqDroop;
        let curve_type2 = DERControlEnumType::FixedVar;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut curve_get = DERCurveGetType::new(
            curve1.clone(),
            id1.clone(),
            curve_type1.clone(),
            true,
            false,
        );

        curve_get
            .set_curve(curve2.clone())
            .set_id(id2.clone())
            .set_curve_type(curve_type2.clone())
            .set_is_default(false)
            .set_is_superseded(true)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(curve_get.curve(), &curve2);
        assert_eq!(curve_get.id(), id2);
        assert_eq!(curve_get.curve_type(), curve_type2);
        assert_eq!(curve_get.is_default(), false);
        assert_eq!(curve_get.is_superseded(), true);
        assert_eq!(curve_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        curve_get.set_custom_data(None);
        assert_eq!(curve_get.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        use rust_decimal::prelude::*;

        // 创建有效的DERCurveGetType实例
        let curve_points = vec![DERCurvePointsType::new(
            Decimal::from_str("1.0").unwrap(),
            Decimal::from_str("2.0").unwrap(),
        )];
        let curve = DERCurveType::new(curve_points, 1, DERUnitEnumType::PctMaxW);
        let id = "valid_id".to_string();
        let curve_type = DERControlEnumType::FreqDroop;
        let is_default = true;
        let is_superseded = false;

        let valid_curve_get = DERCurveGetType::new(
            curve.clone(),
            id.clone(),
            curve_type.clone(),
            is_default,
            is_superseded,
        );

        // 验证有效实例应该通过
        assert!(valid_curve_get.validate().is_ok());

        // 测试ID长度超过限制的情况
        let long_id = "a".repeat(37); // 创建一个37字符长的ID，超过了36的限制
        let invalid_id_curve_get = DERCurveGetType::new(
            curve.clone(),
            long_id,
            curve_type.clone(),
            is_default,
            is_superseded,
        );

        // 验证应该失败，因为ID太长
        let validation_result = invalid_id_curve_get.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("id"));
        assert!(error_message.contains("length"));

        // 测试curve_data为空的情况
        let empty_curve_points: Vec<DERCurvePointsType> = vec![];
        let invalid_curve = DERCurveType::new(empty_curve_points, 1, DERUnitEnumType::PctMaxW);
        let invalid_curve_get = DERCurveGetType::new(
            invalid_curve,
            id.clone(),
            curve_type.clone(),
            is_default,
            is_superseded,
        );

        // 验证应该失败，因为curve_data为空
        let validation_result = invalid_curve_get.validate();
        assert!(validation_result.is_err());
        let error_message = validation_result.unwrap_err().to_string();
        assert!(error_message.contains("curve"));
        assert!(error_message.contains("curve_data"));
    }
}
