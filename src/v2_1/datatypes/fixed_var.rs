use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::der_unit::DERUnitEnumType;
use chrono::{DateTime, Utc};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Fixed VAr settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FixedVarType {
    /// Priority of setting (0=highest)
    #[validate(range(min = 0))]
    pub priority: i32,

    /// The value specifies a target var output
    /// interpreted as a signed percentage (-100 to 100). A
    /// negative value refers to charging, whereas a positive one
    /// refers to discharging. The value type is determined by the
    /// unit field.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub setpoint: Decimal,

    /// Unit of the setpoint.
    pub unit: DERUnitEnumType,

    /// Time when this setting becomes active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// Duration in seconds that this setting is active.
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

impl FixedVarType {
    /// Creates a new `FixedVarType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority of setting (0=highest)
    /// * `var` - Fixed VAr value in VAr
    ///
    /// # Returns
    ///
    /// A new instance of `FixedVarType` with optional fields set to `None`
    pub fn new(priority: i32, var: f64) -> Self {
        Self {
            priority,
            setpoint: Decimal::from_f64(var).unwrap_or_default(),
            unit: DERUnitEnumType::PctMaxVar,
            start_time: None,
            duration: None,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for these fixed VAr settings
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

    /// Gets the VAr value.
    ///
    /// # Returns
    ///
    /// The fixed VAr value in VAr
    pub fn var(&self) -> f64 {
        self.setpoint.to_f64().unwrap_or_default()
    }

    /// Sets the VAr value.
    ///
    /// # Arguments
    ///
    /// * `var` - Fixed VAr value in VAr
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_var(&mut self, var: f64) -> &mut Self {
        self.setpoint = Decimal::from_f64(var).unwrap_or_default();
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
    /// * `custom_data` - Custom data for these fixed VAr settings, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

/// Trait for managing Fixed VAr settings.
pub trait FixedVarSettings {
    /// Gets the priority of the setting.
    fn get_priority(&self) -> i32;

    /// Gets the setpoint value.
    fn get_setpoint(&self) -> f64;
}

impl FixedVarSettings for FixedVarType {
    fn get_priority(&self) -> i32 {
        self.priority()
    }

    fn get_setpoint(&self) -> f64 {
        self.var()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fixed_var() {
        let priority = 1;
        let var = 100.0;

        let fixed_var = FixedVarType::new(priority, var);

        assert_eq!(fixed_var.priority(), priority);
        assert_eq!(fixed_var.var(), var);
        assert_eq!(fixed_var.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let priority = 1;
        let var = 100.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_var = FixedVarType::new(priority, var).with_custom_data(custom_data.clone());

        assert_eq!(fixed_var.priority(), priority);
        assert_eq!(fixed_var.var(), var);
        assert_eq!(fixed_var.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let priority1 = 1;
        let var1 = 100.0;
        let priority2 = 2;
        let var2 = -50.0;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut fixed_var = FixedVarType::new(priority1, var1);

        fixed_var
            .set_priority(priority2)
            .set_var(var2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(fixed_var.priority(), priority2);
        assert_eq!(fixed_var.var(), var2);
        assert_eq!(fixed_var.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        fixed_var.set_custom_data(None);
        assert_eq!(fixed_var.custom_data(), None);
    }

    #[test]
    fn test_fixed_var_settings_trait() {
        let priority = 1;
        let var_value = 100.0;
        let fixed_var = FixedVarType::new(priority, var_value);
        assert_eq!(fixed_var.get_priority(), priority);
        assert_eq!(fixed_var.get_setpoint(), var_value);
    }
}
