use super::{custom_data::CustomDataType, tariff_conditions::TariffConditionsType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Tariff with optional conditions for a time duration price.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffTimePriceType {
    /// Required. Price per minute (excl. tax) for this element.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub price_minute: Decimal,

    /// Optional. Conditions when this tariff element applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub conditions: Option<TariffConditionsType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffTimePriceType {
    /// Creates a new `TariffTimePriceType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `price_minute` - Price per minute (excl. tax) for this element
    ///
    /// # Returns
    ///
    /// A new instance of `TariffTimePriceType` with optional fields set to `None`
    pub fn new(price_minute: Decimal) -> Self {
        Self {
            price_minute,
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
    pub fn with_conditions(mut self, conditions: TariffConditionsType) -> Self {
        self.conditions = Some(conditions);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff time price
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the price per minute.
    ///
    /// # Returns
    ///
    /// The price per minute (excl. tax) for this element
    pub fn price_minute(&self) -> Decimal {
        self.price_minute
    }

    /// Sets the price per minute.
    ///
    /// # Arguments
    ///
    /// * `price_minute` - Price per minute (excl. tax) for this element
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_minute(&mut self, price_minute: Decimal) -> &mut Self {
        self.price_minute = price_minute;
        self
    }

    /// Gets the conditions.
    ///
    /// # Returns
    ///
    /// An optional reference to the conditions when this tariff element applies
    pub fn conditions(&self) -> Option<&TariffConditionsType> {
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
    pub fn set_conditions(&mut self, conditions: Option<TariffConditionsType>) -> &mut Self {
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
    /// * `custom_data` - Custom data for this tariff time price, or None to clear
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
    fn test_new_tariff_time_price() {
        let price_minute = Decimal::new(100, 1); // 10.0
        let tariff_time_price = TariffTimePriceType::new(price_minute);

        assert_eq!(tariff_time_price.price_minute(), price_minute);
        assert_eq!(tariff_time_price.conditions(), None);
        assert_eq!(tariff_time_price.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let price_minute = Decimal::new(100, 1); // 10.0
        let conditions = TariffConditionsType::new();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_time_price = TariffTimePriceType::new(price_minute)
            .with_conditions(conditions.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_time_price.price_minute(), price_minute);
        assert_eq!(tariff_time_price.conditions(), Some(&conditions));
        assert_eq!(tariff_time_price.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let price_minute1 = Decimal::new(100, 1); // 10.0
        let price_minute2 = Decimal::new(150, 1); // 15.0
        let conditions = TariffConditionsType::new();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_time_price = TariffTimePriceType::new(price_minute1);

        tariff_time_price
            .set_price_minute(price_minute2)
            .set_conditions(Some(conditions.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_time_price.price_minute(), price_minute2);
        assert_eq!(tariff_time_price.conditions(), Some(&conditions));
        assert_eq!(tariff_time_price.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_time_price.set_conditions(None).set_custom_data(None);

        assert_eq!(tariff_time_price.conditions(), None);
        assert_eq!(tariff_time_price.custom_data(), None);
    }
}
