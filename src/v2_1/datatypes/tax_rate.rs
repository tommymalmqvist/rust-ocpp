use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Tax percentage
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRateType {
    /// Required. Type of this tax, e.g. "Federal", "State", for information on receipt.
    #[validate(length(max = 20))]
    #[serde(rename = "type")]
    pub type_: String,

    /// Required. Tax percentage
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub tax: Decimal,

    /// Optional. Stack level for this type of tax. Default value, when absent, is 0.
    /// stack = 0: tax on net price;
    /// stack = 1: tax added on top of stack 0;
    /// stack = 2: tax added on top of stack 1, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub stack: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TaxRateType {
    /// Creates a new `TaxRateType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tax` - Tax percentage
    /// * `type_` - Type of this tax
    ///
    /// # Returns
    ///
    /// A new instance of `TaxRateType` with optional fields set to `None`
    pub fn new(tax: Decimal, type_: String) -> Self {
        Self {
            type_,
            tax,
            stack: None,
            custom_data: None,
        }
    }

    /// Sets the stack level for this type of tax.
    ///
    /// # Arguments
    ///
    /// * `stack` - Stack level for this type of tax
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_stack(mut self, stack: i32) -> Self {
        self.stack = Some(stack);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tax rate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the type of this tax.
    ///
    /// # Returns
    ///
    /// The type of this tax
    pub fn type_(&self) -> &str {
        &self.type_
    }

    /// Sets the type of this tax.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of this tax
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: String) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the tax percentage.
    ///
    /// # Returns
    ///
    /// The tax percentage
    pub fn tax(&self) -> Decimal {
        self.tax
    }

    /// Sets the tax percentage.
    ///
    /// # Arguments
    ///
    /// * `tax` - Tax percentage
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax(&mut self, tax: Decimal) -> &mut Self {
        self.tax = tax;
        self
    }

    /// Gets the stack level for this type of tax.
    ///
    /// # Returns
    ///
    /// An optional stack level for this type of tax
    pub fn stack(&self) -> Option<i32> {
        self.stack
    }

    /// Sets the stack level for this type of tax.
    ///
    /// # Arguments
    ///
    /// * `stack` - Stack level for this type of tax, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_stack(&mut self, stack: Option<i32>) -> &mut Self {
        self.stack = stack;
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
    /// * `custom_data` - Custom data for this tax rate, or None to clear
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
    fn test_new_tax_rate() {
        let tax = Decimal::new(210, 1); // 21.0
        let type_ = "VAT".to_string();

        let tax_rate = TaxRateType::new(tax, type_.clone());

        assert_eq!(tax_rate.tax(), tax);
        assert_eq!(tax_rate.type_(), type_);
        assert_eq!(tax_rate.stack(), None);
        assert_eq!(tax_rate.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let tax = Decimal::new(210, 1); // 21.0
        let type_ = "VAT".to_string();
        let stack = 1;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tax_rate = TaxRateType::new(tax, type_.clone())
            .with_stack(stack)
            .with_custom_data(custom_data.clone());

        assert_eq!(tax_rate.tax(), tax);
        assert_eq!(tax_rate.type_(), type_);
        assert_eq!(tax_rate.stack(), Some(stack));
        assert_eq!(tax_rate.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let tax1 = Decimal::new(210, 1); // 21.0
        let type1 = "VAT".to_string();
        let tax2 = Decimal::new(150, 1); // 15.0
        let type2 = "GST".to_string();
        let stack = 1;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tax_rate = TaxRateType::new(tax1, type1);

        tax_rate
            .set_tax(tax2)
            .set_type(type2.clone())
            .set_stack(Some(stack))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tax_rate.tax(), tax2);
        assert_eq!(tax_rate.type_(), type2);
        assert_eq!(tax_rate.stack(), Some(stack));
        assert_eq!(tax_rate.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tax_rate.set_stack(None).set_custom_data(None);
        assert_eq!(tax_rate.stack(), None);
        assert_eq!(tax_rate.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid tax rate
        let tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string());
        assert!(tax_rate.validate().is_ok());

        // Test with invalid type (too long)
        let mut invalid_tax_rate = TaxRateType::new(
            Decimal::new(210, 1),
            "ThisTaxTypeNameIsTooLongAndExceedsTheMaximumLength".to_string(),
        );
        assert!(invalid_tax_rate.validate().is_err());

        // Test with invalid stack (negative value)
        invalid_tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string()).with_stack(-1);
        assert!(invalid_tax_rate.validate().is_err());
    }
}
