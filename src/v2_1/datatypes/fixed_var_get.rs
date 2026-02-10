use super::super::helpers::validator::validate_identifier_string;
use super::{custom_data::CustomDataType, fixed_var::FixedVarType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Fixed VAr get type for retrieving fixed VAr settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FixedVarGetType {
    /// Id of the setting.
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub id: String,

    /// True if setting is a default control.
    pub is_default: bool,

    /// True if this setting is superseded by a lower priority setting
    pub is_superseded: bool,

    /// The fixed VAr settings.
    #[validate(nested)]
    pub fixed_var: FixedVarType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl FixedVarGetType {
    /// Creates a new `FixedVarGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `fixed_var` - The fixed VAr settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// * `is_default` - True if setting is a default control
    /// # Returns
    ///
    /// A new instance of `FixedVarGetType` with optional fields set to `None`
    pub fn new(fixed_var: FixedVarType, id: String, is_superseded: bool, is_default: bool) -> Self {
        Self {
            fixed_var,
            id,
            is_superseded,
            is_default,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this fixed VAr get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the fixed VAr settings.
    ///
    /// # Returns
    ///
    /// A reference to the fixed VAr settings
    pub fn fixed_var(&self) -> &FixedVarType {
        &self.fixed_var
    }

    /// Sets the fixed VAr settings.
    ///
    /// # Arguments
    ///
    /// * `fixed_var` - The fixed VAr settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed_var(&mut self, fixed_var: FixedVarType) -> &mut Self {
        self.fixed_var = fixed_var;
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
    /// * `id` - Id of the setting
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

    /// Gets whether this setting is a default control.
    ///
    /// # Returns
    ///
    /// True if setting is a default control
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Sets whether this setting is a default control.
    ///
    /// # Arguments
    ///
    /// * `is_default` - True if setting is a default control
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = is_default;
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
    /// * `custom_data` - Custom data for this fixed VAr get, or None to clear
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
    fn test_new_fixed_var_get() {
        let fixed_var = FixedVarType::new(1, 100.0);
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;

        let fixed_var_get =
            FixedVarGetType::new(fixed_var.clone(), id.clone(), is_superseded, is_default);

        assert_eq!(fixed_var_get.fixed_var(), &fixed_var);
        assert_eq!(fixed_var_get.id(), id);
        assert_eq!(fixed_var_get.is_superseded(), is_superseded);
        assert_eq!(fixed_var_get.is_default(), is_default);
        assert_eq!(fixed_var_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let fixed_var = FixedVarType::new(1, 100.0);
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let fixed_var_get =
            FixedVarGetType::new(fixed_var.clone(), id.clone(), is_superseded, is_default)
                .with_custom_data(custom_data.clone());

        assert_eq!(fixed_var_get.fixed_var(), &fixed_var);
        assert_eq!(fixed_var_get.id(), id);
        assert_eq!(fixed_var_get.is_superseded(), is_superseded);
        assert_eq!(fixed_var_get.is_default(), is_default);
        assert_eq!(fixed_var_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let fixed_var1 = FixedVarType::new(1, 100.0);
        let fixed_var2 = FixedVarType::new(2, -50.0);
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let is_default1 = true;
        let is_default2 = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut fixed_var_get =
            FixedVarGetType::new(fixed_var1.clone(), id1.clone(), is_superseded1, is_default1);

        fixed_var_get
            .set_fixed_var(fixed_var2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_is_default(is_default2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(fixed_var_get.fixed_var(), &fixed_var2);
        assert_eq!(fixed_var_get.id(), id2);
        assert_eq!(fixed_var_get.is_superseded(), is_superseded2);
        assert_eq!(fixed_var_get.is_default(), is_default2);
        assert_eq!(fixed_var_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        fixed_var_get.set_custom_data(None);
        assert_eq!(fixed_var_get.custom_data(), None);
    }
}
