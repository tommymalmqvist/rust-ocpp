use super::{custom_data::CustomDataType, tax_rate::TaxRateType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Price with and without tax. At least one of exclTax, inclTax must be present.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceType {
    /// Price/cost excluding tax. Can be absent if inclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl_tax: Option<Decimal>,

    /// Price/cost including tax. Can be absent if exclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_tax: Option<Decimal>,

    /// List of tax rates used to calculate tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 5), nested)]
    pub tax_rates: Option<Vec<TaxRateType>>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PriceType {
    /// Creates a new `PriceType` with at least one of the required tax fields.
    ///
    /// # Arguments
    ///
    /// * `price` - The price value (can be either excluding or including tax)
    /// * `is_incl_tax` - Whether the provided price is including tax (true) or excluding tax (false)
    ///
    /// # Returns
    ///
    /// A new instance of `PriceType` with optional fields set to `None`
    pub fn new(price: Decimal, is_incl_tax: bool) -> Self {
        if is_incl_tax {
            Self {
                excl_tax: None,
                incl_tax: Some(price),
                tax_rates: None,
                custom_data: None,
            }
        } else {
            Self {
                excl_tax: Some(price),
                incl_tax: None,
                tax_rates: None,
                custom_data: None,
            }
        }
    }

    /// Gets the price excluding tax.
    ///
    /// # Returns
    ///
    /// An optional reference to the price excluding tax
    pub fn excl_tax(&self) -> Option<Decimal> {
        self.excl_tax
    }

    /// Gets the price including tax.
    ///
    /// # Returns
    ///
    /// An optional reference to the price including tax
    pub fn incl_tax(&self) -> Option<Decimal> {
        self.incl_tax
    }

    /// Gets the tax rates.
    ///
    /// # Returns
    ///
    /// An optional reference to the tax rates
    pub fn tax_rates(&self) -> Option<&Vec<TaxRateType>> {
        self.tax_rates.as_ref()
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the price excluding tax.
    ///
    /// # Arguments
    ///
    /// * `excl_tax` - The price excluding tax, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_excl_tax(&mut self, excl_tax: Option<Decimal>) -> &mut Self {
        self.excl_tax = excl_tax;
        self
    }

    /// Sets the price including tax.
    ///
    /// # Arguments
    ///
    /// * `incl_tax` - The price including tax, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_incl_tax(&mut self, incl_tax: Option<Decimal>) -> &mut Self {
        self.incl_tax = incl_tax;
        self
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - The tax rates, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rates(&mut self, tax_rates: Option<Vec<TaxRateType>>) -> &mut Self {
        self.tax_rates = tax_rates;
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - The custom data, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the price excluding tax.
    ///
    /// # Arguments
    ///
    /// * `excl_tax` - The price excluding tax
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_excl_tax(mut self, excl_tax: Decimal) -> Self {
        self.excl_tax = Some(excl_tax);
        self
    }

    /// Sets the price including tax.
    ///
    /// # Arguments
    ///
    /// * `incl_tax` - The price including tax
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_incl_tax(mut self, incl_tax: Decimal) -> Self {
        self.incl_tax = Some(incl_tax);
        self
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - The tax rates
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tax_rates(mut self, tax_rates: Vec<TaxRateType>) -> Self {
        self.tax_rates = Some(tax_rates);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - The custom data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_price_incl_tax() {
        let price = Decimal::new(1000, 1); // 100.0
        let price_type = PriceType::new(price, true);

        assert_eq!(price_type.excl_tax(), None);
        assert_eq!(price_type.incl_tax(), Some(price));
        assert_eq!(price_type.tax_rates(), None);
        assert_eq!(price_type.custom_data(), None);
    }

    #[test]
    fn test_new_price_excl_tax() {
        let price = Decimal::new(800, 1); // 80.0
        let price_type = PriceType::new(price, false);

        assert_eq!(price_type.excl_tax(), Some(price));
        assert_eq!(price_type.incl_tax(), None);
        assert_eq!(price_type.tax_rates(), None);
        assert_eq!(price_type.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price_excl = Decimal::new(800, 1); // 80.0
        let price_incl = Decimal::new(1000, 1); // 100.0
        let tax_rate = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let custom_data = CustomDataType::new("VendorX".to_string());

        let price_type = PriceType::new(price_excl, false)
            .with_incl_tax(price_incl)
            .with_tax_rates(vec![tax_rate.clone()])
            .with_custom_data(custom_data.clone());

        assert_eq!(price_type.excl_tax(), Some(price_excl));
        assert_eq!(price_type.incl_tax(), Some(price_incl));
        assert_eq!(price_type.tax_rates().unwrap().len(), 1);
        assert_eq!(price_type.tax_rates().unwrap()[0], tax_rate);
        assert_eq!(price_type.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price_excl = Decimal::new(800, 1); // 80.0
        let price_incl = Decimal::new(1000, 1); // 100.0
        let mut price_type = PriceType::new(price_excl, false);
        let tax_rate = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let custom_data = CustomDataType::new("VendorX".to_string());

        price_type
            .set_incl_tax(Some(price_incl))
            .set_tax_rates(Some(vec![tax_rate.clone()]))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(price_type.excl_tax(), Some(price_excl));
        assert_eq!(price_type.incl_tax(), Some(price_incl));
        assert_eq!(price_type.tax_rates().unwrap().len(), 1);
        assert_eq!(price_type.tax_rates().unwrap()[0], tax_rate);
        assert_eq!(price_type.custom_data(), Some(&custom_data));

        // Test clearing values
        price_type
            .set_excl_tax(None)
            .set_tax_rates(None)
            .set_custom_data(None);

        assert_eq!(price_type.excl_tax(), None);
        assert_eq!(price_type.incl_tax(), Some(price_incl));
        assert_eq!(price_type.tax_rates(), None);
        assert_eq!(price_type.custom_data(), None);
    }
}
