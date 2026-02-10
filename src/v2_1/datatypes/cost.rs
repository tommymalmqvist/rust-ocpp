use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::CostKindEnumType;

/// Cost type for consumption costs.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CostType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The kind of cost referred to in the message element amount.
    pub cost_kind: CostKindEnumType,

    /// The estimated or actual cost per kWh.
    pub amount: i32,

    /// Values: -3..3, The amountMultiplier defines the exponent to base 10 (dec).
    /// The final value is determined by: amount * 10 ^ amountMultiplier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = -3, max = 3))]
    pub amount_multiplier: Option<i8>,
}

impl CostType {
    /// Creates a new `CostType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `cost_kind` - The kind of cost referred to in the message element amount
    /// * `amount` - The estimated or actual cost per kWh
    ///
    /// # Returns
    ///
    /// A new instance of `CostType` with optional fields set to `None`
    pub fn new(cost_kind: CostKindEnumType, amount: i32) -> Self {
        Self {
            cost_kind,
            amount,
            amount_multiplier: None,
            custom_data: None,
        }
    }

    /// Sets the amount multiplier.
    ///
    /// # Arguments
    ///
    /// * `amount_multiplier` - The exponent to base 10 (dec) for the amount
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_amount_multiplier(mut self, amount_multiplier: i8) -> Self {
        self.amount_multiplier = Some(amount_multiplier);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the cost kind.
    ///
    /// # Returns
    ///
    /// The kind of cost referred to in the message element amount
    pub fn cost_kind(&self) -> &CostKindEnumType {
        &self.cost_kind
    }

    /// Sets the cost kind.
    ///
    /// # Arguments
    ///
    /// * `cost_kind` - The kind of cost referred to in the message element amount
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost_kind(&mut self, cost_kind: CostKindEnumType) -> &mut Self {
        self.cost_kind = cost_kind;
        self
    }

    /// Gets the amount.
    ///
    /// # Returns
    ///
    /// The estimated or actual cost per kWh
    pub fn amount(&self) -> i32 {
        self.amount
    }

    /// Sets the amount.
    ///
    /// # Arguments
    ///
    /// * `amount` - The estimated or actual cost per kWh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_amount(&mut self, amount: i32) -> &mut Self {
        self.amount = amount;
        self
    }

    /// Gets the amount multiplier.
    ///
    /// # Returns
    ///
    /// An optional exponent to base 10 (dec) for the amount
    pub fn amount_multiplier(&self) -> Option<i8> {
        self.amount_multiplier
    }

    /// Sets the amount multiplier.
    ///
    /// # Arguments
    ///
    /// * `amount_multiplier` - The exponent to base 10 (dec) for the amount, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_amount_multiplier(&mut self, amount_multiplier: Option<i8>) -> &mut Self {
        self.amount_multiplier = amount_multiplier;
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
    /// * `custom_data` - Custom data for this cost, or None to clear
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
    fn test_new_cost() {
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);

        assert_eq!(cost.cost_kind(), &CostKindEnumType::CarbonDioxideEmission);
        assert_eq!(cost.amount(), 100);
        assert_eq!(cost.amount_multiplier(), None);
        assert_eq!(cost.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let cost = CostType::new(CostKindEnumType::RelativePricePercentage, 100)
            .with_amount_multiplier(-2)
            .with_custom_data(custom_data.clone());

        assert_eq!(cost.cost_kind(), &CostKindEnumType::RelativePricePercentage);
        assert_eq!(cost.amount(), 100);
        assert_eq!(cost.amount_multiplier(), Some(-2));
        assert_eq!(cost.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);

        cost.set_cost_kind(CostKindEnumType::RenewableGenerationPercentage)
            .set_amount(200)
            .set_amount_multiplier(Some(-1))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            cost.cost_kind(),
            &CostKindEnumType::RenewableGenerationPercentage
        );
        assert_eq!(cost.amount(), 200);
        assert_eq!(cost.amount_multiplier(), Some(-1));
        assert_eq!(cost.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cost.set_amount_multiplier(None).set_custom_data(None);

        assert_eq!(cost.amount_multiplier(), None);
        assert_eq!(cost.custom_data(), None);
    }
}
