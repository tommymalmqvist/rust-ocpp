use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, rational_number::RationalNumberType};

/// Part of ISO 15118-20 price schedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxRuleType {
    /// Required. Id for the tax rule.
    #[validate(range(min = 0))]
    pub tax_rule_id: i32,

    /// Optional. Human readable string to identify the tax rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 100))]
    pub tax_rule_name: Option<String>,

    /// Optional. Indicates whether the tax is included in any price or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_included_in_price: Option<bool>,

    /// Required. Indicates whether this tax applies to Energy Fees.
    pub applies_to_energy_fee: bool,

    /// Required. Indicates whether this tax applies to Parking Fees.
    pub applies_to_parking_fee: bool,

    /// Required. Indicates whether this tax applies to Overstay Fees.
    pub applies_to_overstay_fee: bool,

    /// Required. Indicates whether this tax applies to Minimum/Maximum Cost.
    pub applies_to_minimum_maximum_cost: bool,

    /// Required. The tax rate.
    #[validate(nested)]
    pub tax_rate: RationalNumberType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TaxRuleType {
    /// Creates a new `TaxRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tax_rule_id` - Id for the tax rule
    /// * `applies_to_energy_fee` - Indicates whether this tax applies to Energy Fees
    /// * `applies_to_parking_fee` - Indicates whether this tax applies to Parking Fees
    /// * `applies_to_overstay_fee` - Indicates whether this tax applies to Overstay Fees
    /// * `applies_to_minimum_maximum_cost` - Indicates whether this tax applies to Minimum/Maximum Cost
    /// * `tax_rate` - The tax rate
    ///
    /// # Returns
    ///
    /// A new instance of `TaxRuleType` with optional fields set to `None`
    pub fn new(
        tax_rule_id: i32,
        applies_to_energy_fee: bool,
        applies_to_parking_fee: bool,
        applies_to_overstay_fee: bool,
        applies_to_minimum_maximum_cost: bool,
        tax_rate: RationalNumberType,
    ) -> Self {
        Self {
            tax_rule_id,
            tax_rule_name: None,
            tax_included_in_price: None,
            applies_to_energy_fee,
            applies_to_parking_fee,
            applies_to_overstay_fee,
            applies_to_minimum_maximum_cost,
            tax_rate,
            custom_data: None,
        }
    }

    /// Sets the tax rule name.
    ///
    /// # Arguments
    ///
    /// * `tax_rule_name` - Human readable string to identify the tax rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tax_rule_name(mut self, tax_rule_name: String) -> Self {
        self.tax_rule_name = Some(tax_rule_name);
        self
    }

    /// Sets whether the tax is included in any price or not.
    ///
    /// # Arguments
    ///
    /// * `tax_included_in_price` - Indicates whether the tax is included in any price or not
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tax_included_in_price(mut self, tax_included_in_price: bool) -> Self {
        self.tax_included_in_price = Some(tax_included_in_price);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tax rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the tax rule ID.
    ///
    /// # Returns
    ///
    /// The ID for the tax rule
    pub fn tax_rule_id(&self) -> i32 {
        self.tax_rule_id
    }

    /// Sets the tax rule ID.
    ///
    /// # Arguments
    ///
    /// * `tax_rule_id` - ID for the tax rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rule_id(&mut self, tax_rule_id: i32) -> &mut Self {
        self.tax_rule_id = tax_rule_id;
        self
    }

    /// Gets the tax rule name.
    ///
    /// # Returns
    ///
    /// An optional reference to the human readable string to identify the tax rule
    pub fn tax_rule_name(&self) -> Option<&str> {
        self.tax_rule_name.as_deref()
    }

    /// Sets the tax rule name.
    ///
    /// # Arguments
    ///
    /// * `tax_rule_name` - Human readable string to identify the tax rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rule_name(&mut self, tax_rule_name: Option<String>) -> &mut Self {
        self.tax_rule_name = tax_rule_name;
        self
    }

    /// Gets whether the tax is included in any price or not.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating whether the tax is included in any price or not
    pub fn tax_included_in_price(&self) -> Option<bool> {
        self.tax_included_in_price
    }

    /// Sets whether the tax is included in any price or not.
    ///
    /// # Arguments
    ///
    /// * `tax_included_in_price` - Indicates whether the tax is included in any price or not, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_included_in_price(&mut self, tax_included_in_price: Option<bool>) -> &mut Self {
        self.tax_included_in_price = tax_included_in_price;
        self
    }

    /// Gets whether this tax applies to Energy Fees.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether this tax applies to Energy Fees
    pub fn applies_to_energy_fee(&self) -> bool {
        self.applies_to_energy_fee
    }

    /// Sets whether this tax applies to Energy Fees.
    ///
    /// # Arguments
    ///
    /// * `applies_to_energy_fee` - Indicates whether this tax applies to Energy Fees
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_applies_to_energy_fee(&mut self, applies_to_energy_fee: bool) -> &mut Self {
        self.applies_to_energy_fee = applies_to_energy_fee;
        self
    }

    /// Gets whether this tax applies to Parking Fees.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether this tax applies to Parking Fees
    pub fn applies_to_parking_fee(&self) -> bool {
        self.applies_to_parking_fee
    }

    /// Sets whether this tax applies to Parking Fees.
    ///
    /// # Arguments
    ///
    /// * `applies_to_parking_fee` - Indicates whether this tax applies to Parking Fees
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_applies_to_parking_fee(&mut self, applies_to_parking_fee: bool) -> &mut Self {
        self.applies_to_parking_fee = applies_to_parking_fee;
        self
    }

    /// Gets whether this tax applies to Overstay Fees.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether this tax applies to Overstay Fees
    pub fn applies_to_overstay_fee(&self) -> bool {
        self.applies_to_overstay_fee
    }

    /// Sets whether this tax applies to Overstay Fees.
    ///
    /// # Arguments
    ///
    /// * `applies_to_overstay_fee` - Indicates whether this tax applies to Overstay Fees
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_applies_to_overstay_fee(&mut self, applies_to_overstay_fee: bool) -> &mut Self {
        self.applies_to_overstay_fee = applies_to_overstay_fee;
        self
    }

    /// Gets whether this tax applies to Minimum/Maximum Cost.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether this tax applies to Minimum/Maximum Cost
    pub fn applies_to_minimum_maximum_cost(&self) -> bool {
        self.applies_to_minimum_maximum_cost
    }

    /// Sets whether this tax applies to Minimum/Maximum Cost.
    ///
    /// # Arguments
    ///
    /// * `applies_to_minimum_maximum_cost` - Indicates whether this tax applies to Minimum/Maximum Cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_applies_to_minimum_maximum_cost(
        &mut self,
        applies_to_minimum_maximum_cost: bool,
    ) -> &mut Self {
        self.applies_to_minimum_maximum_cost = applies_to_minimum_maximum_cost;
        self
    }

    /// Gets the tax rate.
    ///
    /// # Returns
    ///
    /// A reference to the tax rate
    pub fn tax_rate(&self) -> &RationalNumberType {
        &self.tax_rate
    }

    /// Sets the tax rate.
    ///
    /// # Arguments
    ///
    /// * `tax_rate` - The tax rate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rate(&mut self, tax_rate: RationalNumberType) -> &mut Self {
        self.tax_rate = tax_rate;
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
    /// * `custom_data` - Custom data for this tax rule, or None to clear
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
    fn test_new_tax_rule() {
        let tax_rule_id = 1;
        let applies_to_energy_fee = true;
        let applies_to_parking_fee = false;
        let applies_to_overstay_fee = true;
        let applies_to_minimum_maximum_cost = false;
        let tax_rate = RationalNumberType::new(-2, 2100); // 21.00%

        let tax_rule = TaxRuleType::new(
            tax_rule_id,
            applies_to_energy_fee,
            applies_to_parking_fee,
            applies_to_overstay_fee,
            applies_to_minimum_maximum_cost,
            tax_rate.clone(),
        );

        assert_eq!(tax_rule.tax_rule_id(), tax_rule_id);
        assert_eq!(tax_rule.tax_rule_name(), None);
        assert_eq!(tax_rule.tax_included_in_price(), None);
        assert_eq!(tax_rule.applies_to_energy_fee(), applies_to_energy_fee);
        assert_eq!(tax_rule.applies_to_parking_fee(), applies_to_parking_fee);
        assert_eq!(tax_rule.applies_to_overstay_fee(), applies_to_overstay_fee);
        assert_eq!(
            tax_rule.applies_to_minimum_maximum_cost(),
            applies_to_minimum_maximum_cost
        );
        assert_eq!(tax_rule.tax_rate(), &tax_rate);
        assert_eq!(tax_rule.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let tax_rule_id = 1;
        let applies_to_energy_fee = true;
        let applies_to_parking_fee = false;
        let applies_to_overstay_fee = true;
        let applies_to_minimum_maximum_cost = false;
        let tax_rate = RationalNumberType::new(-2, 2100); // 21.00%
        let tax_rule_name = "VAT".to_string();
        let tax_included_in_price = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tax_rule = TaxRuleType::new(
            tax_rule_id,
            applies_to_energy_fee,
            applies_to_parking_fee,
            applies_to_overstay_fee,
            applies_to_minimum_maximum_cost,
            tax_rate.clone(),
        )
        .with_tax_rule_name(tax_rule_name.clone())
        .with_tax_included_in_price(tax_included_in_price)
        .with_custom_data(custom_data.clone());

        assert_eq!(tax_rule.tax_rule_id(), tax_rule_id);
        assert_eq!(tax_rule.tax_rule_name(), Some(tax_rule_name.as_str()));
        assert_eq!(
            tax_rule.tax_included_in_price(),
            Some(tax_included_in_price)
        );
        assert_eq!(tax_rule.applies_to_energy_fee(), applies_to_energy_fee);
        assert_eq!(tax_rule.applies_to_parking_fee(), applies_to_parking_fee);
        assert_eq!(tax_rule.applies_to_overstay_fee(), applies_to_overstay_fee);
        assert_eq!(
            tax_rule.applies_to_minimum_maximum_cost(),
            applies_to_minimum_maximum_cost
        );
        assert_eq!(tax_rule.tax_rate(), &tax_rate);
        assert_eq!(tax_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let tax_rule_id1 = 1;
        let applies_to_energy_fee1 = true;
        let applies_to_parking_fee1 = false;
        let applies_to_overstay_fee1 = true;
        let applies_to_minimum_maximum_cost1 = false;
        let tax_rate1 = RationalNumberType::new(-2, 2100); // 21.00%

        let tax_rule_id2 = 2;
        let applies_to_energy_fee2 = false;
        let applies_to_parking_fee2 = true;
        let applies_to_overstay_fee2 = false;
        let applies_to_minimum_maximum_cost2 = true;
        let tax_rate2 = RationalNumberType::new(-2, 1000); // 10.00%
        let tax_rule_name = "GST".to_string();
        let tax_included_in_price = false;
        let custom_data = CustomDataType::new("VendorY".to_string());

        let mut tax_rule = TaxRuleType::new(
            tax_rule_id1,
            applies_to_energy_fee1,
            applies_to_parking_fee1,
            applies_to_overstay_fee1,
            applies_to_minimum_maximum_cost1,
            tax_rate1,
        );

        tax_rule
            .set_tax_rule_id(tax_rule_id2)
            .set_applies_to_energy_fee(applies_to_energy_fee2)
            .set_applies_to_parking_fee(applies_to_parking_fee2)
            .set_applies_to_overstay_fee(applies_to_overstay_fee2)
            .set_applies_to_minimum_maximum_cost(applies_to_minimum_maximum_cost2)
            .set_tax_rate(tax_rate2.clone())
            .set_tax_rule_name(Some(tax_rule_name.clone()))
            .set_tax_included_in_price(Some(tax_included_in_price))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tax_rule.tax_rule_id(), tax_rule_id2);
        assert_eq!(tax_rule.tax_rule_name(), Some(tax_rule_name.as_str()));
        assert_eq!(
            tax_rule.tax_included_in_price(),
            Some(tax_included_in_price)
        );
        assert_eq!(tax_rule.applies_to_energy_fee(), applies_to_energy_fee2);
        assert_eq!(tax_rule.applies_to_parking_fee(), applies_to_parking_fee2);
        assert_eq!(tax_rule.applies_to_overstay_fee(), applies_to_overstay_fee2);
        assert_eq!(
            tax_rule.applies_to_minimum_maximum_cost(),
            applies_to_minimum_maximum_cost2
        );
        assert_eq!(tax_rule.tax_rate(), &tax_rate2);
        assert_eq!(tax_rule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tax_rule
            .set_tax_rule_name(None)
            .set_tax_included_in_price(None)
            .set_custom_data(None);

        assert_eq!(tax_rule.tax_rule_name(), None);
        assert_eq!(tax_rule.tax_included_in_price(), None);
        assert_eq!(tax_rule.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid tax rule
        let tax_rule = TaxRuleType::new(
            1,
            true,
            false,
            true,
            false,
            RationalNumberType::new(-2, 2100),
        );
        assert!(tax_rule.validate().is_ok());

        // Test with invalid tax_rule_id (negative value)
        let invalid_tax_rule = TaxRuleType::new(
            -1,
            true,
            false,
            true,
            false,
            RationalNumberType::new(-2, 2100),
        );
        assert!(invalid_tax_rule.validate().is_err());

        // Test with invalid tax_rule_name (too long)
        let invalid_tax_rule = TaxRuleType::new(
            1,
            true,
            false,
            true,
            false,
            RationalNumberType::new(-2, 2100),
        )
        .with_tax_rule_name("a".repeat(101));
        assert!(invalid_tax_rule.validate().is_err());
    }
}
