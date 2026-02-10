use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Cost, energy, time or SoC limit for a transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLimitType {
    /// Maximum allowed cost of transaction in currency of tariff.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_cost: Option<Decimal>,

    /// Maximum allowed energy in Wh to charge in transaction.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_energy: Option<Decimal>,

    /// Maximum duration of transaction in seconds from start to end.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i32>,

    /// Maximum State of Charge of EV in percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub max_so_c: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TransactionLimitType {
    /// Creates a new `TransactionLimitType` with all fields set to `None`.
    pub fn new() -> Self {
        Self {
            max_cost: None,
            max_energy: None,
            max_time: None,
            max_so_c: None,
            custom_data: None,
        }
    }

    /// Gets the maximum cost.
    pub fn max_cost(&self) -> Option<Decimal> {
        self.max_cost
    }

    /// Sets the maximum cost.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_max_cost(&mut self, max_cost: Option<Decimal>) -> &mut Self {
        self.max_cost = max_cost;
        self
    }

    /// Gets the maximum energy.
    pub fn max_energy(&self) -> Option<Decimal> {
        self.max_energy
    }

    /// Sets the maximum energy.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_max_energy(&mut self, max_energy: Option<Decimal>) -> &mut Self {
        self.max_energy = max_energy;
        self
    }

    /// Gets the maximum time.
    pub fn max_time(&self) -> Option<i32> {
        self.max_time
    }

    /// Sets the maximum time.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_max_time(&mut self, max_time: Option<i32>) -> &mut Self {
        self.max_time = max_time;
        self
    }

    /// Gets the maximum SoC.
    pub fn max_so_c(&self) -> Option<i32> {
        self.max_so_c
    }

    /// Sets the maximum SoC.
    ///
    /// Returns a mutable reference to self for method chaining.
    pub fn set_max_so_c(&mut self, max_so_c: Option<i32>) -> &mut Self {
        self.max_so_c = max_so_c;
        self
    }

    /// Gets the custom data.
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

    /// Sets the maximum cost using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_max_cost(mut self, max_cost: Decimal) -> Self {
        self.max_cost = Some(max_cost);
        self
    }

    /// Sets the maximum energy using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_max_energy(mut self, max_energy: Decimal) -> Self {
        self.max_energy = Some(max_energy);
        self
    }

    /// Sets the maximum time using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_max_time(mut self, max_time: i32) -> Self {
        self.max_time = Some(max_time);
        self
    }

    /// Sets the maximum SoC using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_max_so_c(mut self, max_so_c: i32) -> Self {
        self.max_so_c = Some(max_so_c);
        self
    }

    /// Sets the custom data using the builder pattern.
    ///
    /// Returns the modified instance.
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Create a new TransactionLimitType from floating point values for cost and energy.
    pub fn new_from_f64(
        max_cost: Option<f64>,
        max_energy: Option<f64>,
        max_time: Option<i32>,
        max_so_c: Option<i32>,
    ) -> Self {
        Self {
            max_cost: max_cost.map(|v| Decimal::from_f64(v).unwrap_or_else(|| Decimal::new(0, 0))),
            max_energy: max_energy
                .map(|v| Decimal::from_f64(v).unwrap_or_else(|| Decimal::new(0, 0))),
            max_time,
            max_so_c,
            custom_data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_transaction_limit() {
        let transaction_limit = TransactionLimitType::new();

        assert_eq!(transaction_limit.max_cost(), None);
        assert_eq!(transaction_limit.max_energy(), None);
        assert_eq!(transaction_limit.max_time(), None);
        assert_eq!(transaction_limit.max_so_c(), None);
        assert_eq!(transaction_limit.custom_data(), None);
    }

    #[test]
    fn test_new_from_f64() {
        let max_cost_f64 = 50.0;
        let max_energy_f64 = 100.0;
        let max_time = 3600;
        let max_so_c = 80;

        let transaction_limit = TransactionLimitType::new_from_f64(
            Some(max_cost_f64),
            Some(max_energy_f64),
            Some(max_time),
            Some(max_so_c),
        );

        assert_eq!(
            transaction_limit.max_cost(),
            Some(Decimal::from_f64(max_cost_f64).unwrap())
        );
        assert_eq!(
            transaction_limit.max_energy(),
            Some(Decimal::from_f64(max_energy_f64).unwrap())
        );
        assert_eq!(transaction_limit.max_time(), Some(max_time));
        assert_eq!(transaction_limit.max_so_c(), Some(max_so_c));
        assert_eq!(transaction_limit.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let max_cost = Decimal::from_f64(50.0).unwrap();
        let max_energy = Decimal::from_f64(100.0).unwrap();
        let max_time = 3600;
        let max_so_c = 80;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let transaction_limit = TransactionLimitType::new()
            .with_max_cost(max_cost)
            .with_max_energy(max_energy)
            .with_max_time(max_time)
            .with_max_so_c(max_so_c)
            .with_custom_data(custom_data.clone());

        assert_eq!(transaction_limit.max_cost(), Some(max_cost));
        assert_eq!(transaction_limit.max_energy(), Some(max_energy));
        assert_eq!(transaction_limit.max_time(), Some(max_time));
        assert_eq!(transaction_limit.max_so_c(), Some(max_so_c));
        assert_eq!(transaction_limit.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let max_cost = Decimal::from_f64(50.0).unwrap();
        let max_energy = Decimal::from_f64(100.0).unwrap();
        let max_time = 3600;
        let max_so_c = 80;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut transaction_limit = TransactionLimitType::new();

        transaction_limit
            .set_max_cost(Some(max_cost))
            .set_max_energy(Some(max_energy))
            .set_max_time(Some(max_time))
            .set_max_so_c(Some(max_so_c))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(transaction_limit.max_cost(), Some(max_cost));
        assert_eq!(transaction_limit.max_energy(), Some(max_energy));
        assert_eq!(transaction_limit.max_time(), Some(max_time));
        assert_eq!(transaction_limit.max_so_c(), Some(max_so_c));
        assert_eq!(transaction_limit.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        transaction_limit
            .set_max_cost(None)
            .set_max_energy(None)
            .set_max_time(None)
            .set_max_so_c(None)
            .set_custom_data(None);

        assert_eq!(transaction_limit.max_cost(), None);
        assert_eq!(transaction_limit.max_energy(), None);
        assert_eq!(transaction_limit.max_time(), None);
        assert_eq!(transaction_limit.max_so_c(), None);
        assert_eq!(transaction_limit.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        let valid_transaction_limit = TransactionLimitType::new().with_max_so_c(80);

        assert!(valid_transaction_limit.validate().is_ok());

        let invalid_transaction_limit = TransactionLimitType {
            max_cost: None,
            max_energy: None,
            max_time: None,
            max_so_c: Some(101), // Invalid: greater than 100
            custom_data: None,
        };

        assert!(invalid_transaction_limit.validate().is_err());

        let invalid_transaction_limit2 = TransactionLimitType {
            max_cost: None,
            max_energy: None,
            max_time: None,
            max_so_c: Some(-1), // Invalid: less than 0
            custom_data: None,
        };

        assert!(invalid_transaction_limit2.validate().is_err());
    }

    #[test]
    fn test_serde() {
        let transaction_limit = TransactionLimitType::new()
            .with_max_cost(Decimal::from_f64(50.0).unwrap())
            .with_max_energy(Decimal::from_f64(100.0).unwrap())
            .with_max_time(3600)
            .with_max_so_c(80);

        let json = serde_json::to_string(&transaction_limit).unwrap();
        let deserialized: TransactionLimitType = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized, transaction_limit);
    }
}
