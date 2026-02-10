use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::{attribute::AttributeEnumType, mutability::MutabilityEnumType};

/// Attribute data of a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableAttributeType {
    /// Required. Type of attribute: Actual, Target, MinSet, MaxSet, etc.
    #[serde(rename = "type")]
    pub type_: AttributeEnumType,

    /// Value of the attribute. May only be omitted when mutability is set to 'WriteOnly'.
    ///
    /// The Configuration Variable <<configkey-reporting-value-size,ReportingValueSize>> can be used to limit GetVariableResult.attributeValue, VariableAttribute.value and EventData.actualValue. The max size of these values will always remain equal.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2500))]
    pub value: Option<String>,

    /// Defines the mutability of this attribute.
    pub mutability: MutabilityEnumType,

    /// If true, value will be persistent across system reboots or power down. Default when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,

    /// If true, value that will never be changed by the Charging Station at runtime. Default when omitted is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<bool>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VariableAttributeType {
    /// Creates a new `VariableAttributeType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of attribute
    /// * `mutability` - Defines the mutability of this attribute
    ///
    /// # Returns
    ///
    /// A new instance of `VariableAttributeType` with optional fields set to `None`
    pub fn new(type_: AttributeEnumType, mutability: MutabilityEnumType) -> Self {
        Self {
            type_,
            value: None,
            mutability,
            persistent: None,
            constant: None,
            custom_data: None,
        }
    }

    /// Creates a new `VariableAttributeType` with required fields and value.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of attribute
    /// * `value` - Value of the attribute
    /// * `mutability` - Defines the mutability of this attribute
    ///
    /// # Returns
    ///
    /// A new instance of `VariableAttributeType` with optional fields set to `None`
    pub fn new_with_value(
        type_: AttributeEnumType,
        value: String,
        mutability: MutabilityEnumType,
    ) -> Self {
        Self {
            type_,
            value: Some(value),
            mutability,
            persistent: None,
            constant: None,
            custom_data: None,
        }
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value of the attribute
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the persistent flag.
    ///
    /// # Arguments
    ///
    /// * `persistent` - Boolean indicating if this variable is persistent between sessions
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = Some(persistent);
        self
    }

    /// Sets the constant flag.
    ///
    /// # Arguments
    ///
    /// * `constant` - Boolean indicating if this variable is constant
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_constant(mut self, constant: bool) -> Self {
        self.constant = Some(constant);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this variable attribute
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the attribute type.
    ///
    /// # Returns
    ///
    /// The type of attribute
    pub fn type_(&self) -> &AttributeEnumType {
        &self.type_
    }

    /// Sets the attribute type.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of attribute
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: AttributeEnumType) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the attribute value.
    ///
    /// # Returns
    ///
    /// An optional reference to the value of the attribute
    pub fn value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    /// Sets the attribute value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value of the attribute, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: Option<String>) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the mutability.
    ///
    /// # Returns
    ///
    /// The mutability of this attribute
    pub fn mutability(&self) -> &MutabilityEnumType {
        &self.mutability
    }

    /// Sets the mutability.
    ///
    /// # Arguments
    ///
    /// * `mutability` - Defines the mutability of this attribute
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_mutability(&mut self, mutability: MutabilityEnumType) -> &mut Self {
        self.mutability = mutability;
        self
    }

    /// Gets the persistent flag.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating if this variable is persistent between sessions
    pub fn persistent(&self) -> Option<bool> {
        self.persistent
    }

    /// Sets the persistent flag.
    ///
    /// # Arguments
    ///
    /// * `persistent` - Boolean indicating if this variable is persistent between sessions, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_persistent(&mut self, persistent: Option<bool>) -> &mut Self {
        self.persistent = persistent;
        self
    }

    /// Gets the constant flag.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating if this variable is constant
    pub fn constant(&self) -> Option<bool> {
        self.constant
    }

    /// Sets the constant flag.
    ///
    /// # Arguments
    ///
    /// * `constant` - Boolean indicating if this variable is constant, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_constant(&mut self, constant: Option<bool>) -> &mut Self {
        self.constant = constant;
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
    /// * `custom_data` - Custom data for this variable attribute, or None to clear
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
    fn test_new_variable_attribute() {
        let attr_type = AttributeEnumType::Actual;
        let mutability = MutabilityEnumType::ReadOnly;

        let attribute = VariableAttributeType::new(attr_type.clone(), mutability.clone());

        assert_eq!(attribute.type_(), &attr_type);
        assert_eq!(attribute.value(), None);
        assert_eq!(attribute.mutability(), &mutability);
        assert_eq!(attribute.persistent(), None);
        assert_eq!(attribute.constant(), None);
        assert_eq!(attribute.custom_data(), None);
    }

    #[test]
    fn test_new_with_value() {
        let attr_type = AttributeEnumType::Actual;
        let value = "42".to_string();
        let mutability = MutabilityEnumType::ReadOnly;

        let attribute = VariableAttributeType::new_with_value(
            attr_type.clone(),
            value.clone(),
            mutability.clone(),
        );

        assert_eq!(attribute.type_(), &attr_type);
        assert_eq!(attribute.value(), Some(&value));
        assert_eq!(attribute.mutability(), &mutability);
        assert_eq!(attribute.persistent(), None);
        assert_eq!(attribute.constant(), None);
        assert_eq!(attribute.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let attr_type = AttributeEnumType::Actual;
        let value = "42".to_string();
        let mutability = MutabilityEnumType::ReadOnly;
        let persistent = true;
        let constant = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let attribute = VariableAttributeType::new(attr_type.clone(), mutability.clone())
            .with_value(value.clone())
            .with_persistent(persistent)
            .with_constant(constant)
            .with_custom_data(custom_data.clone());

        assert_eq!(attribute.type_(), &attr_type);
        assert_eq!(attribute.value(), Some(&value));
        assert_eq!(attribute.mutability(), &mutability);
        assert_eq!(attribute.persistent(), Some(persistent));
        assert_eq!(attribute.constant(), Some(constant));
        assert_eq!(attribute.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let attr_type1 = AttributeEnumType::Actual;
        let attr_type2 = AttributeEnumType::Target;
        let value1 = "42".to_string();
        let value2 = "55".to_string();
        let mutability1 = MutabilityEnumType::ReadOnly;
        let mutability2 = MutabilityEnumType::ReadWrite;
        let persistent = true;
        let constant = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut attribute = VariableAttributeType::new_with_value(
            attr_type1.clone(),
            value1.clone(),
            mutability1.clone(),
        );

        attribute
            .set_type(attr_type2.clone())
            .set_value(Some(value2.clone()))
            .set_mutability(mutability2.clone())
            .set_persistent(Some(persistent))
            .set_constant(Some(constant))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(attribute.type_(), &attr_type2);
        assert_eq!(attribute.value(), Some(&value2));
        assert_eq!(attribute.mutability(), &mutability2);
        assert_eq!(attribute.persistent(), Some(persistent));
        assert_eq!(attribute.constant(), Some(constant));
        assert_eq!(attribute.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        attribute
            .set_value(None)
            .set_persistent(None)
            .set_constant(None)
            .set_custom_data(None);

        assert_eq!(attribute.value(), None);
        assert_eq!(attribute.persistent(), None);
        assert_eq!(attribute.constant(), None);
        assert_eq!(attribute.custom_data(), None);
    }
}
