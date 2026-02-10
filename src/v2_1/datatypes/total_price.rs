use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Total cost with and without tax.
///
/// Contains the total of energy, charging time, idle time, fixed and reservation costs
/// including and/or excluding tax.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TotalPriceType {
    /// Price/cost excluding tax. Can be absent if inclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excl_tax: Option<Decimal>,

    /// Price/cost including tax. Can be absent if exclTax is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incl_tax: Option<Decimal>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TotalPriceType {
    /// Creates a new empty `TotalPriceType` with all fields set to `None`.
    pub fn new() -> Self {
        Self {
            excl_tax: None,
            incl_tax: None,
            custom_data: None,
        }
    }

    /// Creates a new `TotalPriceType` with the excluding tax value.
    pub fn new_excl_tax(excl_tax: Decimal) -> Self {
        Self {
            excl_tax: Some(excl_tax),
            incl_tax: None,
            custom_data: None,
        }
    }

    /// Creates a new `TotalPriceType` with the including tax value.
    pub fn new_incl_tax(incl_tax: Decimal) -> Self {
        Self {
            excl_tax: None,
            incl_tax: Some(incl_tax),
            custom_data: None,
        }
    }

    /// Gets the excluding tax value.
    pub fn excl_tax(&self) -> Option<Decimal> {
        self.excl_tax
    }

    /// Sets the excluding tax value.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_excl_tax(&mut self, excl_tax: Option<Decimal>) -> &mut Self {
        self.excl_tax = excl_tax;
        self
    }

    /// Gets the including tax value.
    pub fn incl_tax(&self) -> Option<Decimal> {
        self.incl_tax
    }

    /// Sets the including tax value.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_incl_tax(&mut self, incl_tax: Option<Decimal>) -> &mut Self {
        self.incl_tax = incl_tax;
        self
    }

    /// Gets a reference to the custom data, if present.
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the excluding tax value using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_excl_tax(mut self, excl_tax: Decimal) -> Self {
        self.excl_tax = Some(excl_tax);
        self
    }

    /// Sets the including tax value using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_incl_tax(mut self, incl_tax: Decimal) -> Self {
        self.incl_tax = Some(incl_tax);
        self
    }

    /// Sets the custom data using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::FromPrimitive;

    #[test]
    fn test_new_total_price() {
        let total_price = TotalPriceType::new();
        assert_eq!(total_price.excl_tax(), None);
        assert_eq!(total_price.incl_tax(), None);
        assert_eq!(total_price.custom_data(), None);
    }

    #[test]
    fn test_new_with_excl_tax() {
        let price = Decimal::from_f64(100.0).unwrap();
        let total_price = TotalPriceType::new_excl_tax(price);
        assert_eq!(total_price.excl_tax(), Some(price));
        assert_eq!(total_price.incl_tax(), None);
        assert_eq!(total_price.custom_data(), None);
    }

    #[test]
    fn test_new_with_incl_tax() {
        let price = Decimal::from_f64(100.0).unwrap();
        let total_price = TotalPriceType::new_incl_tax(price);
        assert_eq!(total_price.excl_tax(), None);
        assert_eq!(total_price.incl_tax(), Some(price));
        assert_eq!(total_price.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let excl_tax = Decimal::from_f64(80.0).unwrap();
        let incl_tax = Decimal::from_f64(100.0).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let total_price = TotalPriceType::new()
            .with_excl_tax(excl_tax)
            .with_incl_tax(incl_tax)
            .with_custom_data(custom_data.clone());

        assert_eq!(total_price.excl_tax(), Some(excl_tax));
        assert_eq!(total_price.incl_tax(), Some(incl_tax));
        assert_eq!(total_price.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let excl_tax = Decimal::from_f64(80.0).unwrap();
        let incl_tax = Decimal::from_f64(100.0).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut total_price = TotalPriceType::new();

        total_price
            .set_excl_tax(Some(excl_tax))
            .set_incl_tax(Some(incl_tax))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(total_price.excl_tax(), Some(excl_tax));
        assert_eq!(total_price.incl_tax(), Some(incl_tax));
        assert_eq!(total_price.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        total_price
            .set_excl_tax(None)
            .set_incl_tax(None)
            .set_custom_data(None);

        assert_eq!(total_price.excl_tax(), None);
        assert_eq!(total_price.incl_tax(), None);
        assert_eq!(total_price.custom_data(), None);
    }
}
