use super::{custom_data::CustomDataType, limit_max_discharge::LimitMaxDischargeType};
use crate::v2_1::helpers::validator::validate_identifier_string;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Limit max discharge get type for retrieving limit max discharge settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LimitMaxDischargeGetType {
    /// The limit max discharge settings.
    #[validate(nested)]
    pub limit_max_discharge: LimitMaxDischargeType,

    /// Id of the setting.
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,

    /// True if this is the default setting.
    pub is_default: bool,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl LimitMaxDischargeGetType {
    /// Creates a new `LimitMaxDischargeGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `limit_max_discharge` - The limit max discharge settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    /// * `is_default` - True if this is the default setting
    ///
    /// # Returns
    ///
    /// A new instance of `LimitMaxDischargeGetType` with optional fields set to `None`
    pub fn new(
        limit_max_discharge: LimitMaxDischargeType,
        id: String,
        is_superseded: bool,
        is_default: bool,
    ) -> Self {
        Self {
            custom_data: None,
            limit_max_discharge,
            id,
            is_superseded,
            is_default,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this LimitMaxDischargeGet
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this LimitMaxDischargeGet, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the limit max discharge settings.
    ///
    /// # Returns
    ///
    /// Reference to the limit max discharge settings
    pub fn limit_max_discharge(&self) -> &LimitMaxDischargeType {
        &self.limit_max_discharge
    }

    /// Sets the limit max discharge settings.
    ///
    /// # Arguments
    ///
    /// * `limit_max_discharge` - The limit max discharge settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_limit_max_discharge(
        &mut self,
        limit_max_discharge: LimitMaxDischargeType,
    ) -> &mut Self {
        self.limit_max_discharge = limit_max_discharge;
        self
    }

    /// Gets the ID of the setting.
    ///
    /// # Returns
    ///
    /// The ID of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
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

    /// Gets whether this is the default setting.
    ///
    /// # Returns
    ///
    /// True if this is the default setting
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Sets whether this is the default setting.
    ///
    /// # Arguments
    ///
    /// * `is_default` - True if this is the default setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = is_default;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::*;

    #[test]
    fn test_new_limit_max_discharge_get() {
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let limit_max_discharge = LimitMaxDischargeType::new(1, pct_max_discharge_power);
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;

        let limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge.clone(),
            id.clone(),
            is_superseded,
            is_default,
        );

        assert_eq!(limit_get.limit_max_discharge(), &limit_max_discharge);
        assert_eq!(limit_get.id(), id);
        assert_eq!(limit_get.is_superseded(), is_superseded);
        assert_eq!(limit_get.is_default(), is_default);
        assert_eq!(limit_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let limit_max_discharge = LimitMaxDischargeType::new(1, pct_max_discharge_power);
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge.clone(),
            id.clone(),
            is_superseded,
            is_default,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(limit_get.limit_max_discharge(), &limit_max_discharge);
        assert_eq!(limit_get.id(), id);
        assert_eq!(limit_get.is_superseded(), is_superseded);
        assert_eq!(limit_get.is_default(), is_default);
        assert_eq!(limit_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let pct_max_discharge_power1 = Decimal::from_str("80.0").unwrap();
        let pct_max_discharge_power2 = Decimal::from_str("90.0").unwrap();
        let limit_max_discharge1 = LimitMaxDischargeType::new(1, pct_max_discharge_power1);
        let limit_max_discharge2 = LimitMaxDischargeType::new(2, pct_max_discharge_power2);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let is_default1 = true;
        let is_default2 = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge1.clone(),
            id1.clone(),
            is_superseded1,
            is_default1,
        );

        limit_get
            .set_limit_max_discharge(limit_max_discharge2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_is_default(is_default2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(limit_get.limit_max_discharge(), &limit_max_discharge2);
        assert_eq!(limit_get.id(), id2);
        assert_eq!(limit_get.is_superseded(), is_superseded2);
        assert_eq!(limit_get.is_default(), is_default2);
        assert_eq!(limit_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        limit_get.set_custom_data(None);
        assert_eq!(limit_get.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        // Valid values
        let pct_max_discharge_power = Decimal::from_str("80.0").unwrap();
        let limit_max_discharge = LimitMaxDischargeType::new(1, pct_max_discharge_power.clone());
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;

        let limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge.clone(),
            id.clone(),
            is_superseded,
            is_default,
        );
        assert!(limit_get.validate().is_ok());

        // Test with invalid id (too long)
        let limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge.clone(),
            "A".repeat(37),
            is_superseded,
            is_default,
        );
        assert!(limit_get.validate().is_err());

        // Test with invalid id (invalid characters)
        let limit_get = LimitMaxDischargeGetType::new(
            limit_max_discharge.clone(),
            "invalid-id!".to_string(),
            is_superseded,
            is_default,
        );
        assert!(limit_get.validate().is_err());

        // Test with invalid LimitMaxDischargeType (negative priority)
        let invalid_limit_max_discharge = LimitMaxDischargeType::new(-1, pct_max_discharge_power);
        let limit_get = LimitMaxDischargeGetType::new(
            invalid_limit_max_discharge,
            id.clone(),
            is_superseded,
            is_default,
        );
        assert!(limit_get.validate().is_err());
    }
}
