use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, tariff_time_price::TariffTimePriceType, tax_rate::TaxRateType,
};

/// Price elements and tax for time
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimeType {
    /// Required. List of time price elements.
    #[validate(length(min = 1), nested)]
    pub prices: Vec<TariffTimePriceType>,

    /// Optional. List of tax rates applicable to time.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 5), nested)]
    pub tax_rates: Option<Vec<TaxRateType>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffTimeType {
    /// Creates a new `TariffTimeType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `prices` - List of time price elements
    ///
    /// # Returns
    ///
    /// A new instance of `TariffTimeType` with optional fields set to `None`
    pub fn new(prices: Vec<TariffTimePriceType>) -> Self {
        Self {
            prices,
            tax_rates: None,
            custom_data: None,
        }
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - List of tax rates applicable to time
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
    /// * `custom_data` - Custom data for this tariff time
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
    /// A reference to the list of time price elements
    pub fn prices(&self) -> &Vec<TariffTimePriceType> {
        &self.prices
    }

    /// Sets the prices.
    ///
    /// # Arguments
    ///
    /// * `prices` - List of time price elements
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_prices(&mut self, prices: Vec<TariffTimePriceType>) -> &mut Self {
        self.prices = prices;
        self
    }

    /// Gets the tax rates.
    ///
    /// # Returns
    ///
    /// An optional reference to the list of tax rates applicable to time
    pub fn tax_rates(&self) -> Option<&Vec<TaxRateType>> {
        self.tax_rates.as_ref()
    }

    /// Sets the tax rates.
    ///
    /// # Arguments
    ///
    /// * `tax_rates` - List of tax rates applicable to time, or None to clear
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
    /// * `custom_data` - Custom data for this tariff time, or None to clear
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
    fn test_new_tariff_time() {
        let price = TariffTimePriceType::new(Decimal::new(100, 1)); // 10.0
        let prices = vec![price.clone()];
        let tariff_time = TariffTimeType::new(prices.clone());

        assert_eq!(tariff_time.prices(), &prices);
        assert_eq!(tariff_time.tax_rates(), None);
        assert_eq!(tariff_time.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price1 = TariffTimePriceType::new(Decimal::new(100, 1)); // 10.0
        let price2 = TariffTimePriceType::new(Decimal::new(150, 1)); // 15.0
        let prices = vec![price1.clone(), price2.clone()];

        let tax_rate1 = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let tax_rate2 = TaxRateType::new(Decimal::new(50, 1), "GST".to_string()); // 5.0
        let tax_rates = vec![tax_rate1.clone(), tax_rate2.clone()];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_time = TariffTimeType::new(prices.clone())
            .with_tax_rates(tax_rates.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_time.prices(), &prices);
        assert_eq!(tariff_time.tax_rates(), Some(&tax_rates));
        assert_eq!(tariff_time.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price1 = TariffTimePriceType::new(Decimal::new(100, 1)); // 10.0
        let prices1 = vec![price1.clone()];

        let price2 = TariffTimePriceType::new(Decimal::new(150, 1)); // 15.0
        let price3 = TariffTimePriceType::new(Decimal::new(200, 1)); // 20.0
        let prices2 = vec![price2.clone(), price3.clone()];

        let tax_rate1 = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let tax_rate2 = TaxRateType::new(Decimal::new(50, 1), "GST".to_string()); // 5.0
        let tax_rates = vec![tax_rate1.clone(), tax_rate2.clone()];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_time = TariffTimeType::new(prices1);

        tariff_time
            .set_prices(prices2.clone())
            .set_tax_rates(Some(tax_rates.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_time.prices(), &prices2);
        assert_eq!(tariff_time.tax_rates(), Some(&tax_rates));
        assert_eq!(tariff_time.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_time.set_tax_rates(None).set_custom_data(None);

        assert_eq!(tariff_time.tax_rates(), None);
        assert_eq!(tariff_time.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Test with valid prices
        let price = TariffTimePriceType::new(Decimal::new(100, 1)); // 10.0
        let prices = vec![price.clone()];
        let tariff_time = TariffTimeType::new(prices);
        assert!(tariff_time.validate().is_ok());

        // Test with empty prices (invalid)
        let empty_prices: Vec<TariffTimePriceType> = vec![];
        let invalid_tariff = TariffTimeType {
            prices: empty_prices,
            tax_rates: None,
            custom_data: None,
        };
        assert!(invalid_tariff.validate().is_err());

        // Test with empty tax rates (invalid)
        let empty_tax_rates: Vec<TaxRateType> = vec![];
        let invalid_tariff = TariffTimeType {
            prices: vec![price.clone()],
            tax_rates: Some(empty_tax_rates),
            custom_data: None,
        };
        assert!(invalid_tariff.validate().is_err());

        // Test with too many tax rates (invalid)
        let tax_rate = TaxRateType::new(Decimal::new(200, 1), "VAT".to_string()); // 20.0
        let too_many_tax_rates = vec![
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(),
            tax_rate.clone(), // Exceeds the max of 5
        ];
        let invalid_tariff = TariffTimeType {
            prices: vec![price],
            tax_rates: Some(too_many_tax_rates),
            custom_data: None,
        };
        assert!(invalid_tariff.validate().is_err());
    }
}
