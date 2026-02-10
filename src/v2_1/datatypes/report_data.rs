use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    component::ComponentType, custom_data::CustomDataType, variable::VariableType,
    variable_attribute::VariableAttributeType,
    variable_characteristics::VariableCharacteristicsType,
};

/// Class to report components, variables and variable attributes and characteristics.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ReportDataType {
    /// Required. Component for which a report of Variable is requested.
    #[validate(nested)]
    pub component: ComponentType,

    /// Required. Variable for which a report is requested.
    #[validate(nested)]
    pub variable: VariableType,

    /// Required. List of variable attribute types and values.
    #[validate(length(min = 1, max = 4), nested)]
    pub variable_attribute: Vec<VariableAttributeType>,

    /// Optional. Fixed read-only parameters of the variable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub variable_characteristics: Option<VariableCharacteristicsType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ReportDataType {
    /// Creates a new `ReportDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    /// * `variable` - Variable for which a report is requested
    /// * `variable_attribute` - List of variable attribute types and values
    ///
    /// # Returns
    ///
    /// A new instance of `ReportDataType` with optional fields set to `None`
    pub fn new(
        component: ComponentType,
        variable: VariableType,
        variable_attribute: Vec<VariableAttributeType>,
    ) -> Self {
        Self {
            component,
            variable,
            variable_attribute,
            variable_characteristics: None,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this report data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the variable characteristics.
    ///
    /// # Arguments
    ///
    /// * `variable_characteristics` - Fixed read-only parameters of the variable
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_variable_characteristics(
        mut self,
        variable_characteristics: VariableCharacteristicsType,
    ) -> Self {
        self.variable_characteristics = Some(variable_characteristics);
        self
    }

    /// Gets the component.
    ///
    /// # Returns
    ///
    /// A reference to the component
    pub fn component(&self) -> &ComponentType {
        &self.component
    }

    /// Sets the component.
    ///
    /// # Arguments
    ///
    /// * `component` - Component for which a report of Variable is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_component(&mut self, component: ComponentType) -> &mut Self {
        self.component = component;
        self
    }

    /// Gets the variable.
    ///
    /// # Returns
    ///
    /// A reference to the variable
    pub fn variable(&self) -> &VariableType {
        &self.variable
    }

    /// Sets the variable.
    ///
    /// # Arguments
    ///
    /// * `variable` - Variable for which a report is requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable(&mut self, variable: VariableType) -> &mut Self {
        self.variable = variable;
        self
    }

    /// Gets the variable attributes.
    ///
    /// # Returns
    ///
    /// A reference to the list of variable attributes
    pub fn variable_attribute(&self) -> &[VariableAttributeType] {
        &self.variable_attribute
    }

    /// Sets the variable attributes.
    ///
    /// # Arguments
    ///
    /// * `variable_attribute` - List of variable attribute types and values
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_attribute(
        &mut self,
        variable_attribute: Vec<VariableAttributeType>,
    ) -> &mut Self {
        self.variable_attribute = variable_attribute;
        self
    }

    /// Gets the variable characteristics.
    ///
    /// # Returns
    ///
    /// An optional reference to the variable characteristics
    pub fn variable_characteristics(&self) -> Option<&VariableCharacteristicsType> {
        self.variable_characteristics.as_ref()
    }

    /// Sets the variable characteristics.
    ///
    /// # Arguments
    ///
    /// * `variable_characteristics` - Fixed read-only parameters of the variable, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_variable_characteristics(
        &mut self,
        variable_characteristics: Option<VariableCharacteristicsType>,
    ) -> &mut Self {
        self.variable_characteristics = variable_characteristics;
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
    /// * `custom_data` - Custom data for this report data, or None to clear
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
    use crate::v2_1::enumerations::data_enum::DataEnumType;
    use crate::v2_1::enumerations::{attribute::AttributeEnumType, mutability::MutabilityEnumType};

    #[test]
    fn test_new_report_data() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let attribute = VariableAttributeType::new_with_value(
            AttributeEnumType::MaxSet,
            "100".to_string(),
            MutabilityEnumType::ReadOnly,
        );
        let variable_attributes = vec![attribute];

        let report_data = ReportDataType::new(
            component.clone(),
            variable.clone(),
            variable_attributes.clone(),
        );

        assert_eq!(report_data.component(), &component);
        assert_eq!(report_data.variable(), &variable);
        assert_eq!(
            report_data.variable_attribute(),
            variable_attributes.as_slice()
        );
        assert_eq!(report_data.custom_data(), None);
        assert_eq!(report_data.variable_characteristics(), None);
    }

    #[test]
    fn test_with_methods() {
        let component = ComponentType::new("Connector".to_string());
        let variable =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());
        let attribute = VariableAttributeType::new_with_value(
            AttributeEnumType::MaxSet,
            "100".to_string(),
            MutabilityEnumType::ReadOnly,
        );
        let variable_attributes = vec![attribute];
        let variable_characteristics =
            VariableCharacteristicsType::new(DataEnumType::Integer, true)
                .with_unit("Ampere".to_string())
                .with_min_limit(0.0)
                .with_max_limit(100.0);

        let report_data = ReportDataType::new(
            component.clone(),
            variable.clone(),
            variable_attributes.clone(),
        )
        .with_custom_data(custom_data.clone())
        .with_variable_characteristics(variable_characteristics.clone());

        assert_eq!(report_data.component(), &component);
        assert_eq!(report_data.variable(), &variable);
        assert_eq!(
            report_data.variable_attribute(),
            variable_attributes.as_slice()
        );
        assert_eq!(report_data.custom_data(), Some(&custom_data));
        assert_eq!(
            report_data.variable_characteristics(),
            Some(&variable_characteristics)
        );
    }

    #[test]
    fn test_setter_methods() {
        let component1 = ComponentType::new("Connector".to_string());
        let variable1 =
            VariableType::new_with_instance("CurrentLimit".to_string(), "Main".to_string());
        let attribute1 = VariableAttributeType::new_with_value(
            AttributeEnumType::MaxSet,
            "100".to_string(),
            MutabilityEnumType::ReadOnly,
        );
        let variable_attributes1 = vec![attribute1];

        let component2 = ComponentType::new("Meter".to_string());
        let variable2 =
            VariableType::new_with_instance("VoltageLimit".to_string(), "Secondary".to_string());
        let attribute2 = VariableAttributeType::new_with_value(
            AttributeEnumType::MinSet,
            "50".to_string(),
            MutabilityEnumType::ReadWrite,
        );
        let variable_attributes2 = vec![attribute2];

        let custom_data = CustomDataType::new("VendorX".to_string());
        let variable_characteristics =
            VariableCharacteristicsType::new(DataEnumType::Integer, true)
                .with_unit("Volt".to_string())
                .with_min_limit(0.0)
                .with_max_limit(500.0);

        let mut report_data = ReportDataType::new(component1, variable1, variable_attributes1);

        report_data
            .set_component(component2.clone())
            .set_variable(variable2.clone())
            .set_variable_attribute(variable_attributes2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_variable_characteristics(Some(variable_characteristics.clone()));

        assert_eq!(report_data.component(), &component2);
        assert_eq!(report_data.variable(), &variable2);
        assert_eq!(
            report_data.variable_attribute(),
            variable_attributes2.as_slice()
        );
        assert_eq!(report_data.custom_data(), Some(&custom_data));
        assert_eq!(
            report_data.variable_characteristics(),
            Some(&variable_characteristics)
        );

        // Test clearing optional fields
        report_data
            .set_custom_data(None)
            .set_variable_characteristics(None);

        assert_eq!(report_data.custom_data(), None);
        assert_eq!(report_data.variable_characteristics(), None);
    }
}
