use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::data_enum::DataEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Fixed read-only parameters of a variable.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct VariableCharacteristicsType {
    /// Unit of the variable. When the transmitted value has a unit, this field SHALL be included.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 16))]
    pub unit: Option<String>,

    /// Required. Data type of this variable.
    pub data_type: DataEnumType,

    /// Minimum possible value of this variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit: Option<f64>,

    /// Maximum possible value of this variable. When the datatype of this Variable is String,
    /// OptionList, SequenceList or MemberList, this field defines the maximum length of the (CSV) string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit: Option<f64>,

    /// (2.1) Maximum number of elements from _valuesList_ that are supported as _attributeValue_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_elements: Option<i32>,

    /// Mandatory when _dataType_ = OptionList, MemberList or SequenceList. In that case _valuesList_
    /// specifies the allowed values for the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 1000))]
    pub values_list: Option<String>,

    /// Required. Flag indicating if this variable supports monitoring.
    pub supports_monitoring: bool,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl VariableCharacteristicsType {
    /// Creates a new `VariableCharacteristicsType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `data_type` - Data type of this variable
    /// * `supports_monitoring` - Flag indicating if this variable supports monitoring
    ///
    /// # Returns
    ///
    /// A new `VariableCharacteristicsType` instance with optional fields set to `None`
    pub fn new(data_type: DataEnumType, supports_monitoring: bool) -> Self {
        Self {
            unit: None,
            data_type,
            min_limit: None,
            max_limit: None,
            max_elements: None,
            values_list: None,
            supports_monitoring,
            custom_data: None,
        }
    }

    /// Sets the unit field.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_unit(mut self, unit: String) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Sets the min_limit field.
    ///
    /// # Arguments
    ///
    /// * `min_limit` - Minimum possible value of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_min_limit(mut self, min_limit: f64) -> Self {
        self.min_limit = Some(min_limit);
        self
    }

    /// Sets the max_limit field.
    ///
    /// # Arguments
    ///
    /// * `max_limit` - Maximum possible value of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_max_limit(mut self, max_limit: f64) -> Self {
        self.max_limit = Some(max_limit);
        self
    }

    /// Sets the max_elements field.
    ///
    /// # Arguments
    ///
    /// * `max_elements` - Maximum number of elements from valuesList that are supported
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_max_elements(mut self, max_elements: i32) -> Self {
        self.max_elements = Some(max_elements);
        self
    }

    /// Sets the values_list field.
    ///
    /// # Arguments
    ///
    /// * `values_list` - Allowed values for OptionList, MemberList or SequenceList types
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_values_list(mut self, values_list: String) -> Self {
        self.values_list = Some(values_list);
        self
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the unit.
    ///
    /// # Returns
    ///
    /// An optional reference to the unit of the variable
    pub fn unit(&self) -> Option<&String> {
        self.unit.as_ref()
    }

    /// Sets the unit.
    ///
    /// # Arguments
    ///
    /// * `unit` - Unit of the variable, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_unit(&mut self, unit: Option<String>) -> &mut Self {
        self.unit = unit;
        self
    }

    /// Gets the data type.
    ///
    /// # Returns
    ///
    /// The data type of this variable
    pub fn data_type(&self) -> &DataEnumType {
        &self.data_type
    }

    /// Sets the data type.
    ///
    /// # Arguments
    ///
    /// * `data_type` - Data type of this variable
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_data_type(&mut self, data_type: DataEnumType) -> &mut Self {
        self.data_type = data_type;
        self
    }

    /// Gets the minimum limit.
    ///
    /// # Returns
    ///
    /// An optional minimum possible value of this variable
    pub fn min_limit(&self) -> Option<f64> {
        self.min_limit
    }

    /// Sets the minimum limit.
    ///
    /// # Arguments
    ///
    /// * `min_limit` - Minimum possible value of this variable, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_min_limit(&mut self, min_limit: Option<f64>) -> &mut Self {
        self.min_limit = min_limit;
        self
    }

    /// Gets the maximum limit.
    ///
    /// # Returns
    ///
    /// An optional maximum possible value of this variable
    pub fn max_limit(&self) -> Option<f64> {
        self.max_limit
    }

    /// Sets the maximum limit.
    ///
    /// # Arguments
    ///
    /// * `max_limit` - Maximum possible value of this variable, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_max_limit(&mut self, max_limit: Option<f64>) -> &mut Self {
        self.max_limit = max_limit;
        self
    }

    /// Gets the maximum elements.
    ///
    /// # Returns
    ///
    /// An optional maximum number of elements from valuesList that are supported
    pub fn max_elements(&self) -> Option<i32> {
        self.max_elements
    }

    /// Sets the maximum elements.
    ///
    /// # Arguments
    ///
    /// * `max_elements` - Maximum number of elements from valuesList, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_max_elements(&mut self, max_elements: Option<i32>) -> &mut Self {
        self.max_elements = max_elements;
        self
    }

    /// Gets the values list.
    ///
    /// # Returns
    ///
    /// An optional reference to the allowed values for special list types
    pub fn values_list(&self) -> Option<&String> {
        self.values_list.as_ref()
    }

    /// Sets the values list.
    ///
    /// # Arguments
    ///
    /// * `values_list` - Allowed values for special list types, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_values_list(&mut self, values_list: Option<String>) -> &mut Self {
        self.values_list = values_list;
        self
    }

    /// Gets the supports monitoring flag.
    ///
    /// # Returns
    ///
    /// Flag indicating if this variable supports monitoring
    pub fn supports_monitoring(&self) -> bool {
        self.supports_monitoring
    }

    /// Sets the supports monitoring flag.
    ///
    /// # Arguments
    ///
    /// * `supports_monitoring` - Flag indicating if this variable supports monitoring
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_supports_monitoring(&mut self, supports_monitoring: bool) -> &mut Self {
        self.supports_monitoring = supports_monitoring;
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
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `VariableCharacteristicsType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_characteristics_new() {
        let data_type = DataEnumType::Decimal;
        let supports_monitoring = true;

        let characteristics =
            VariableCharacteristicsType::new(data_type.clone(), supports_monitoring);

        assert_eq!(characteristics.unit(), None);
        assert_eq!(characteristics.data_type(), &data_type);
        assert_eq!(characteristics.min_limit(), None);
        assert_eq!(characteristics.max_limit(), None);
        assert_eq!(characteristics.max_elements(), None);
        assert_eq!(characteristics.values_list(), None);
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring);
        assert_eq!(characteristics.custom_data(), None);
    }

    #[test]
    fn test_variable_characteristics_with_methods() {
        let data_type = DataEnumType::OptionList;
        let supports_monitoring = true;
        let unit = "kWh".to_string();
        let min_limit = 0.0;
        let max_limit = 100.0;
        let max_elements = 5;
        let values_list = "a,b,c,d,e".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let characteristics =
            VariableCharacteristicsType::new(data_type.clone(), supports_monitoring)
                .with_unit(unit.clone())
                .with_min_limit(min_limit)
                .with_max_limit(max_limit)
                .with_max_elements(max_elements)
                .with_values_list(values_list.clone())
                .with_custom_data(custom_data.clone());

        assert_eq!(characteristics.unit(), Some(&unit));
        assert_eq!(characteristics.data_type(), &data_type);
        assert_eq!(characteristics.min_limit(), Some(min_limit));
        assert_eq!(characteristics.max_limit(), Some(max_limit));
        assert_eq!(characteristics.max_elements(), Some(max_elements));
        assert_eq!(characteristics.values_list(), Some(&values_list));
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring);
        assert_eq!(characteristics.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_variable_characteristics_setters() {
        let data_type1 = DataEnumType::Decimal;
        let data_type2 = DataEnumType::Integer;
        let supports_monitoring1 = true;
        let supports_monitoring2 = false;
        let unit = "A".to_string();
        let min_limit = -10.0;
        let max_limit = 200.0;
        let max_elements = 10;
        let values_list = "x,y,z".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut characteristics =
            VariableCharacteristicsType::new(data_type1, supports_monitoring1);

        characteristics
            .set_data_type(data_type2.clone())
            .set_supports_monitoring(supports_monitoring2)
            .set_unit(Some(unit.clone()))
            .set_min_limit(Some(min_limit))
            .set_max_limit(Some(max_limit))
            .set_max_elements(Some(max_elements))
            .set_values_list(Some(values_list.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(characteristics.unit(), Some(&unit));
        assert_eq!(characteristics.data_type(), &data_type2);
        assert_eq!(characteristics.min_limit(), Some(min_limit));
        assert_eq!(characteristics.max_limit(), Some(max_limit));
        assert_eq!(characteristics.max_elements(), Some(max_elements));
        assert_eq!(characteristics.values_list(), Some(&values_list));
        assert_eq!(characteristics.supports_monitoring(), supports_monitoring2);
        assert_eq!(characteristics.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        characteristics
            .set_unit(None)
            .set_min_limit(None)
            .set_max_limit(None)
            .set_max_elements(None)
            .set_values_list(None)
            .set_custom_data(None);

        assert_eq!(characteristics.unit(), None);
        assert_eq!(characteristics.min_limit(), None);
        assert_eq!(characteristics.max_limit(), None);
        assert_eq!(characteristics.max_elements(), None);
        assert_eq!(characteristics.values_list(), None);
        assert_eq!(characteristics.custom_data(), None);
    }
}
