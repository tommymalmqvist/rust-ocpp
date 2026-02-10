use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, tariff_fixed_price::TariffFixedPriceType, tax_rate::TaxRateType,
};

/// Fixed tariff structure defining fixed costs.
///
/// This structure contains lists of fixed prices and optional tax rates
/// that apply to a tariff, following the OCPP 2.1 specification.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffFixedType {
    /// Required. List of fixed prices.
    ///
    /// This list must contain at least one element.
    #[validate(length(min = 1))]
    pub prices: Vec<TariffFixedPriceType>,

    /// Optional. List of taxes. Relevant only to taxes that have a fixed amount.
    ///
    /// When provided, this list must contain at least one and at most 5 elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 5))]
    pub tax_rates: Option<Vec<TaxRateType>>,

    /// Optional. Custom data from the Charging Station.
    ///
    /// This field MAY contain any custom data sent by the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffFixedType {
    /// Creates a new `TariffFixedType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `prices` - List of fixed prices. Must contain at least one element.
    ///
    /// # Returns
    ///
    /// A new instance of `TariffFixedType` with optional fields set to `None`
    /// A reference to first price in the prices list
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use rust_ocpp::v2_1::datatypes::{tariff_fixed::TariffFixedType, tariff_fixed_price::TariffFixedPriceType};
    /// # use rust_decimal::Decimal;
    /// let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
    /// let tariff_fixed = TariffFixedType::new(vec![fixed_price]);
    /// ```
    pub fn new(prices: Vec<TariffFixedPriceType>) -> Self {
        Self {
            prices,
            tax_rates: None,
            custom_data: None,
        }
    }

    /// Creates a new `TariffFixedType` with a single fixed price for backward compatibility.
    ///
    /// # Arguments
    ///
    /// * `fixed_price` - The fixed price to use as the only element in the prices list
    ///
    /// # Returns
    ///
    /// A new instance of `TariffFixedType` with a vec containing the single price
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use rust_ocpp::v2_1::datatypes::{tariff_fixed::TariffFixedType, tariff_fixed_price::TariffFixedPriceType};
    /// # use rust_decimal::Decimal;
    /// let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
    /// let tariff_fixed = TariffFixedType::from_single_price(fixed_price);
    /// ```
    #[doc(hidden)]
    #[deprecated(since = "3.0.1", note = "Use new() with a vector instead")]
    pub fn from_single_price(fixed_price: TariffFixedPriceType) -> Self {
        Self::new(vec![fixed_price])
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - List of taxes to apply. Must contain at least 1 and at most 5 elements.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use rust_ocpp::v2_1::datatypes::{tariff_fixed::TariffFixedType, tariff_fixed_price::TariffFixedPriceType, tax_rate::TaxRateType};
    /// # use rust_decimal::Decimal;
    /// let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
    /// let tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string()); // 21.0%
    /// let tariff_fixed = TariffFixedType::new(vec![fixed_price])
    ///     .with_tax_rates(vec![tax_rate]);
    /// ```
    pub fn with_tax_rates(mut self, tax_rates: Vec<TaxRateType>) -> Self {
        self.tax_rates = Some(tax_rates);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff fixed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the prices.
    ///
    /// # Returns
    ///
    /// A reference to the list of fixed prices
    pub fn prices(&self) -> &[TariffFixedPriceType] {
        &self.prices
    }

    /// Gets the fixed price (for backward compatibility).
    /// Returns the first price in the prices list.
    ///
    /// # Returns
    ///
    /// A reference to the first fixed price in the prices list
    ///
    /// # Panics
    ///
    /// Will panic if the prices list is empty, which should never happen due to validation
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use rust_ocpp::v2_1::datatypes::{tariff_fixed::TariffFixedType, tariff_fixed_price::TariffFixedPriceType};
    /// # use rust_decimal::Decimal;
    /// let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
    /// let tariff_fixed = TariffFixedType::new(vec![fixed_price.clone()]);
    /// #[allow(deprecated)]
    /// let price = tariff_fixed.fixed_price();
    /// assert_eq!(price, &fixed_price);
    /// ```
    #[deprecated(since = "3.0.1", note = "Use prices() instead")]
    pub fn fixed_price(&self) -> &TariffFixedPriceType {
        &self.prices[0]
    }

    /// Sets the prices.
    ///
    /// # Arguments
    ///
    /// * `prices` - List of fixed prices
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_prices(&mut self, prices: Vec<TariffFixedPriceType>) -> &mut Self {
        self.prices = prices;
        self
    }

    /// Gets the tax rates.
    ///
    /// # Returns
    ///
    /// An optional reference to the list of taxes
    pub fn tax_rates(&self) -> Option<&[TaxRateType]> {
        self.tax_rates.as_deref()
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - List of taxes, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tax_rates(&mut self, tax_rates: Option<Vec<TaxRateType>>) -> &mut Self {
        self.tax_rates = tax_rates;
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
    /// * `custom_data` - Custom data for this tariff fixed, or None to clear
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
    use rust_decimal::Decimal;

    #[test]
    fn test_new_tariff_fixed() {
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
        let prices = vec![fixed_price.clone()];
        let tariff_fixed = TariffFixedType::new(prices.clone());

        assert_eq!(tariff_fixed.prices(), &prices);
        assert_eq!(tariff_fixed.tax_rates(), None);
        assert_eq!(tariff_fixed.custom_data(), None);
    }

    #[test]
    #[allow(deprecated)]
    fn test_from_single_price() {
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
        let tariff_fixed = TariffFixedType::from_single_price(fixed_price.clone());

        assert_eq!(tariff_fixed.prices().len(), 1);
        assert_eq!(tariff_fixed.prices()[0], fixed_price);
        assert_eq!(tariff_fixed.tax_rates(), None);
        assert_eq!(tariff_fixed.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
        let prices = vec![fixed_price.clone()];
        let tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string()); // 21.0%
        let tax_rates = vec![tax_rate.clone()];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_fixed = TariffFixedType::new(prices.clone())
            .with_tax_rates(tax_rates.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_fixed.prices(), &prices);
        assert_eq!(tariff_fixed.tax_rates(), Some(&tax_rates[..]));
        assert_eq!(tariff_fixed.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let fixed_price1 = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
        let fixed_price2 = TariffFixedPriceType::new(Decimal::new(150, 1)); // 15.0
        let prices1 = vec![fixed_price1.clone()];
        let prices2 = vec![fixed_price2.clone()];
        let tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string()); // 21.0%
        let tax_rates = vec![tax_rate.clone()];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_fixed = TariffFixedType::new(prices1.clone());

        tariff_fixed
            .set_prices(prices2.clone())
            .set_tax_rates(Some(tax_rates.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_fixed.prices(), &prices2);
        assert_eq!(tariff_fixed.tax_rates(), Some(&tax_rates[..]));
        assert_eq!(tariff_fixed.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_fixed.set_tax_rates(None).set_custom_data(None);

        assert_eq!(tariff_fixed.tax_rates(), None);
        assert_eq!(tariff_fixed.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid tariff fixed
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1));
        let prices = vec![fixed_price.clone()];
        let tariff_fixed = TariffFixedType::new(prices);
        assert!(tariff_fixed.validate().is_ok());

        // Test with empty prices vector
        let invalid_tariff_fixed = TariffFixedType::new(vec![]);
        assert!(invalid_tariff_fixed.validate().is_err());

        // Test with too many tax rates
        let tax_rate = TaxRateType::new(Decimal::new(210, 1), "VAT".to_string());
        let tax_rates = vec![
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(), // 6 is more than the max of 5
        ];
        let invalid_tariff_fixed =
            TariffFixedType::new(vec![fixed_price.clone()]).with_tax_rates(tax_rates);
        assert!(invalid_tariff_fixed.validate().is_err());
    }
}
