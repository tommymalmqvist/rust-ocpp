use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    charging_period::ChargingPeriodType, custom_data::CustomDataType, total_cost::TotalCostType,
    total_usage::TotalUsageType,
};

/// CostDetailsType contains the cost as calculated by Charging Station based on provided TariffType.
/// NOTE: Reservation is not shown as a chargingPeriod, because it took place outside of the transaction.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CostDetailsType {
    /// List of Charging Periods that make up this
    /// charging session. A finished session has of 1 or more
    /// periods, where each period has a different list of
    /// dimensions that determined the price. When sent as a
    /// running cost update during a transaction chargingPeriods
    /// are omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub charging_periods: Option<Vec<ChargingPeriodType>>,

    /// Total cost of this transaction, including taxes.
    #[validate(nested)]
    pub total_cost: TotalCostType,

    /// Total usage of energy and time during this transaction.
    #[validate(nested)]
    pub total_usage: TotalUsageType,

    /// If set to true, then Charging Station has failed to calculate the cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_to_calculate: Option<bool>,

    /// Optional human-readable reason text in case of failure to calculate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 500))]
    pub failure_reason: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CostDetailsType {
    /// Creates a new `CostDetailsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `charging_periods` - List of charging periods that make up this transaction
    /// * `total_cost` - Total cost of this transaction, including taxes
    /// * `total_usage` - Total usage of energy and time during this transaction
    ///
    /// # Returns
    ///
    /// A new instance of `CostDetailsType` with optional fields set to `None`
    pub fn new(
        charging_periods: Vec<ChargingPeriodType>,
        total_cost: TotalCostType,
        total_usage: TotalUsageType,
    ) -> Self {
        Self {
            charging_periods: Some(charging_periods),
            total_cost,
            total_usage,
            failure_to_calculate: None,
            failure_reason: None,
            custom_data: None,
        }
    }

    /// Sets the failure to calculate flag.
    ///
    /// # Arguments
    ///
    /// * `failure_to_calculate` - If true, then Charging Station has failed to calculate the cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_failure_to_calculate(mut self, failure_to_calculate: bool) -> Self {
        self.failure_to_calculate = Some(failure_to_calculate);
        self
    }

    /// Sets the failure reason.
    ///
    /// # Arguments
    ///
    /// * `failure_reason` - Human-readable reason text in case of failure to calculate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_failure_reason(mut self, failure_reason: String) -> Self {
        self.failure_reason = Some(failure_reason);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this cost details
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the charging periods.
    ///
    /// # Returns
    ///
    /// A reference to the list of charging periods that make up this transaction, or an empty slice if none
    pub fn charging_periods(&self) -> &[ChargingPeriodType] {
        self.charging_periods.as_deref().unwrap_or(&[])
    }

    /// Sets the charging periods.
    ///
    /// # Arguments
    ///
    /// * `charging_periods` - List of charging periods that make up this transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_periods(&mut self, charging_periods: Vec<ChargingPeriodType>) -> &mut Self {
        self.charging_periods = Some(charging_periods);
        self
    }

    /// Gets the total cost.
    ///
    /// # Returns
    ///
    /// A reference to the total cost of this transaction, including taxes
    pub fn total_cost(&self) -> &TotalCostType {
        &self.total_cost
    }

    /// Sets the total cost.
    ///
    /// # Arguments
    ///
    /// * `total_cost` - Total cost of this transaction, including taxes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_total_cost(&mut self, total_cost: TotalCostType) -> &mut Self {
        self.total_cost = total_cost;
        self
    }

    /// Gets the total usage.
    ///
    /// # Returns
    ///
    /// A reference to the total usage of energy and time during this transaction
    pub fn total_usage(&self) -> &TotalUsageType {
        &self.total_usage
    }

    /// Sets the total usage.
    ///
    /// # Arguments
    ///
    /// * `total_usage` - Total usage of energy and time during this transaction
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_total_usage(&mut self, total_usage: TotalUsageType) -> &mut Self {
        self.total_usage = total_usage;
        self
    }

    /// Gets the failure to calculate flag.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating if the Charging Station has failed to calculate the cost
    pub fn failure_to_calculate(&self) -> Option<bool> {
        self.failure_to_calculate
    }

    /// Sets the failure to calculate flag.
    ///
    /// # Arguments
    ///
    /// * `failure_to_calculate` - If true, then Charging Station has failed to calculate the cost, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_failure_to_calculate(&mut self, failure_to_calculate: Option<bool>) -> &mut Self {
        self.failure_to_calculate = failure_to_calculate;
        self
    }

    /// Gets the failure reason.
    ///
    /// # Returns
    ///
    /// An optional reference to the human-readable reason text in case of failure to calculate
    pub fn failure_reason(&self) -> Option<&str> {
        self.failure_reason.as_deref()
    }

    /// Sets the failure reason.
    ///
    /// # Arguments
    ///
    /// * `failure_reason` - Human-readable reason text in case of failure to calculate, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_failure_reason(&mut self, failure_reason: Option<String>) -> &mut Self {
        self.failure_reason = failure_reason;
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
    /// * `custom_data` - Custom data for this cost details, or None to clear
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
    use super::super::total_price::TotalPriceType;
    use super::*;
    use crate::v2_1::enumerations::TariffCostEnumType;
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;
    use validator::Validate;
    #[test]
    fn test_new_cost_details() {
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);

        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        );

        assert_eq!(cost_details.charging_periods().len(), 1);
        assert_eq!(cost_details.total_cost().currency, "EUR");
        assert_eq!(
            cost_details.total_usage().energy,
            Decimal::from_f64(20.0).unwrap()
        );
        assert_eq!(cost_details.failure_to_calculate(), None);
        assert_eq!(cost_details.failure_reason(), None);
        assert_eq!(cost_details.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);

        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        )
        .with_failure_to_calculate(true)
        .with_failure_reason("Calculation error".to_string())
        .with_custom_data(custom_data.clone());

        assert_eq!(cost_details.charging_periods().len(), 1);
        assert_eq!(cost_details.total_cost().currency, "EUR");
        assert_eq!(
            cost_details.total_usage().energy,
            Decimal::from_f64(20.0).unwrap()
        );
        assert_eq!(cost_details.failure_to_calculate(), Some(true));
        assert_eq!(cost_details.failure_reason(), Some("Calculation error"));
        assert_eq!(cost_details.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let charging_period1 = ChargingPeriodType::new(chrono::Utc::now(), vec![]);

        let charging_period2 =
            ChargingPeriodType::new(chrono::Utc::now() + chrono::Duration::hours(1), vec![]);

        let total_cost1 = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
        };

        let total_cost2 = TotalCostType {
            currency: "USD".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
            total: TotalPriceType {
                excl_tax: Some(rust_decimal_macros::dec!(12.0)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
        };

        let total_usage1 = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let total_usage2 = TotalUsageType {
            energy: Decimal::from_f64(25.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut cost_details = CostDetailsType::new(
            vec![charging_period1.clone()],
            total_cost1.clone(),
            total_usage1.clone(),
        );

        cost_details
            .set_charging_periods(vec![charging_period1.clone(), charging_period2.clone()])
            .set_total_cost(total_cost2.clone())
            .set_total_usage(total_usage2.clone())
            .set_failure_to_calculate(Some(true))
            .set_failure_reason(Some("Calculation error".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(cost_details.charging_periods().len(), 2);
        assert_eq!(cost_details.total_cost().currency, "USD");
        assert_eq!(
            cost_details.total_usage().energy,
            Decimal::from_f64(25.0).unwrap()
        );
        assert_eq!(cost_details.failure_to_calculate(), Some(true));
        assert_eq!(cost_details.failure_reason(), Some("Calculation error"));
        assert_eq!(cost_details.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cost_details
            .set_failure_to_calculate(None)
            .set_failure_reason(None)
            .set_custom_data(None);

        assert_eq!(cost_details.failure_to_calculate(), None);
        assert_eq!(cost_details.failure_reason(), None);
        assert_eq!(cost_details.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid case
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);
        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        );

        // Valid instance should pass validation
        assert!(
            cost_details.validate().is_ok(),
            "Valid cost details should pass validation"
        );
    }

    #[test]
    fn test_empty_charging_periods_validation() {
        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(120, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        // Create a CostDetailsType with empty charging periods
        let mut cost_details_empty_periods =
            CostDetailsType::new(vec![], total_cost.clone(), total_usage.clone());

        // Manually override the charging_periods to test validation
        // Since the constructor wraps in Some, we manually set to Some(vec![])
        cost_details_empty_periods.charging_periods = Some(vec![]);

        let validation_result = cost_details_empty_periods.validate();
        assert!(
            validation_result.is_err(),
            "Empty charging periods should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();
        assert!(
            field_errors.contains_key("charging_periods"),
            "Validation errors should contain charging_periods field"
        );
    }

    #[test]
    fn test_failure_reason_length_validation() {
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);
        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::from_f64(20.0).unwrap(),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        // Valid case with reasonable failure reason length
        let valid_cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        )
        .with_failure_to_calculate(true)
        .with_failure_reason("Calculation failed due to network connectivity issue".to_string());

        assert!(
            valid_cost_details.validate().is_ok(),
            "Cost details with valid failure reason should pass validation"
        );

        // Invalid case with failure reason exceeding 500 characters
        let too_long_reason = "X".repeat(501);
        let invalid_cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        )
        .with_failure_to_calculate(true)
        .with_failure_reason(too_long_reason);

        let validation_result = invalid_cost_details.validate();
        assert!(
            validation_result.is_err(),
            "Cost details with too long failure reason should fail validation"
        );

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();
        assert!(
            field_errors.contains_key("failure_reason"),
            "Validation errors should contain failure_reason field"
        );
    }

    #[test]
    fn test_custom_data_validation() {
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);
        let total_cost = TotalCostType {
            currency: "EUR".to_string(),
            type_of_cost: TariffCostEnumType::NormalCost,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::new(20, 1),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        // Create custom data with invalid vendor_id (too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            total_cost.clone(),
            total_usage.clone(),
        )
        .with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = cost_details.validate();
        assert!(
            validation_result.is_err(),
            "Invalid custom_data should cause validation failure"
        );
    }

    #[test]
    fn test_nested_validation() {
        let charging_period = ChargingPeriodType::new(chrono::Utc::now(), vec![]);

        // Create a TotalCostType with invalid currency (exceeds max length of 3)
        let invalid_total_cost = TotalCostType {
            currency: "USDX".to_string(), // Should fail validation - max length is 3
            type_of_cost: TariffCostEnumType::NormalCost,
            total: TotalPriceType {
                excl_tax: Some(Decimal::new(105, 1)),
                incl_tax: None,
                custom_data: None,
            },
            custom_data: None,
            fixed: None,
            energy: None,
            charging_time: None,
            idle_time: None,
            reservation_time: None,
            reservation_fixed: None,
        };

        let total_usage = TotalUsageType {
            energy: Decimal::new(100, 2),
            charging_time: 3600,
            idle_time: 600,
            reservation_time: None,
            custom_data: None,
        };

        let cost_details = CostDetailsType::new(
            vec![charging_period.clone()],
            invalid_total_cost,
            total_usage,
        );

        // Validation should fail due to invalid nested total_cost
        let validation_result = cost_details.validate();
        assert!(
            validation_result.is_err(),
            "Invalid nested total_cost should cause validation failure"
        );

        if let Err(errors) = validation_result {
            println!("Validation errors: {:?}", errors);
        }
    }
}
