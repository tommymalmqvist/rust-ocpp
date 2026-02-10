use super::{custom_data::CustomDataType, tariff_conditions_fixed::TariffConditionsFixedType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Tariff with optional conditions for a fixed price.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffFixedPriceType {
    /// Required. Fixed price for this element e.g. a start fee.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub price_fixed: Decimal,

    /// Optional. Conditions when this tariff element applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub conditions: Option<TariffConditionsFixedType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffFixedPriceType {
    /// Creates a new `TariffFixedPriceType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `price_fixed` - Fixed price for this element e.g. a start fee
    ///
    /// # Returns
    ///
    /// A new instance of `TariffFixedPriceType` with optional fields set to `None`
    pub fn new(price_fixed: Decimal) -> Self {
        Self {
            price_fixed,
            conditions: None,
            custom_data: None,
        }
    }

    /// Sets the conditions when this tariff element applies.
    ///
    /// # Arguments
    ///
    /// * `conditions` - Conditions when this tariff element applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_conditions(mut self, conditions: TariffConditionsFixedType) -> Self {
        self.conditions = Some(conditions);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff fixed price
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the fixed price.
    ///
    /// # Returns
    ///
    /// The fixed price for this element
    pub fn price_fixed(&self) -> Decimal {
        self.price_fixed
    }

    /// Sets the fixed price.
    ///
    /// # Arguments
    ///
    /// * `price_fixed` - Fixed price for this element
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_fixed(&mut self, price_fixed: Decimal) -> &mut Self {
        self.price_fixed = price_fixed;
        self
    }

    /// Gets the conditions.
    ///
    /// # Returns
    ///
    /// An optional reference to the conditions when this tariff element applies
    pub fn conditions(&self) -> Option<&TariffConditionsFixedType> {
        self.conditions.as_ref()
    }

    /// Sets the conditions.
    ///
    /// # Arguments
    ///
    /// * `conditions` - Conditions when this tariff element applies, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_conditions(&mut self, conditions: Option<TariffConditionsFixedType>) -> &mut Self {
        self.conditions = conditions;
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
    /// * `custom_data` - Custom data for this tariff fixed price, or None to clear
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
    fn test_new_tariff_fixed_price() {
        let price_fixed = Decimal::new(100, 1); // 10.0
        let tariff_fixed_price = TariffFixedPriceType::new(price_fixed);

        assert_eq!(tariff_fixed_price.price_fixed(), price_fixed);
        assert_eq!(tariff_fixed_price.conditions(), None);
        assert_eq!(tariff_fixed_price.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price_fixed = Decimal::new(100, 1); // 10.0
        let conditions = TariffConditionsFixedType::new();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_fixed_price = TariffFixedPriceType::new(price_fixed)
            .with_conditions(conditions.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_fixed_price.price_fixed(), price_fixed);
        assert_eq!(tariff_fixed_price.conditions(), Some(&conditions));
        assert_eq!(tariff_fixed_price.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price_fixed1 = Decimal::new(100, 1); // 10.0
        let price_fixed2 = Decimal::new(150, 1); // 15.0
        let conditions = TariffConditionsFixedType::new();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_fixed_price = TariffFixedPriceType::new(price_fixed1);

        tariff_fixed_price
            .set_price_fixed(price_fixed2)
            .set_conditions(Some(conditions.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_fixed_price.price_fixed(), price_fixed2);
        assert_eq!(tariff_fixed_price.conditions(), Some(&conditions));
        assert_eq!(tariff_fixed_price.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_fixed_price
            .set_conditions(None)
            .set_custom_data(None);

        assert_eq!(tariff_fixed_price.conditions(), None);
        assert_eq!(tariff_fixed_price.custom_data(), None);
    }
}
