use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{cost::CostType, custom_data::CustomDataType};

/// Consumption cost type for consumption blocks.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConsumptionCostType {
    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,

    /// The lowest level of consumption that defines the starting point of this consumption block.
    /// The block interval extends to the start of the next interval.
    #[serde(with = "rust_decimal::serde::arbitrary_precision")]
    pub start_value: Decimal,

    /// List of costs associated with this consumption block.
    #[validate(length(min = 1, max = 3), nested)]
    pub cost: Vec<CostType>,
}

impl ConsumptionCostType {
    /// Creates a new `ConsumptionCostType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_value` - The lowest level of consumption that defines the starting point of this consumption block
    /// * `cost` - List of costs associated with this consumption block
    ///
    /// # Returns
    ///
    /// A new instance of `ConsumptionCostType` with optional fields set to `None`
    pub fn new(start_value: Decimal, cost: Vec<CostType>) -> Self {
        Self {
            start_value,
            cost,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this consumption cost
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start value.
    ///
    /// # Returns
    ///
    /// The lowest level of consumption that defines the starting point of this consumption block
    pub fn start_value(&self) -> Decimal {
        self.start_value
    }

    /// Sets the start value.
    ///
    /// # Arguments
    ///
    /// * `start_value` - The lowest level of consumption that defines the starting point of this consumption block
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_value(&mut self, start_value: Decimal) -> &mut Self {
        self.start_value = start_value;
        self
    }

    /// Gets the costs.
    ///
    /// # Returns
    ///
    /// A reference to the list of costs associated with this consumption block
    pub fn cost(&self) -> &[CostType] {
        &self.cost
    }

    /// Sets the costs.
    ///
    /// # Arguments
    ///
    /// * `cost` - List of costs associated with this consumption block
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_cost(&mut self, cost: Vec<CostType>) -> &mut Self {
        self.cost = cost;
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
    /// * `custom_data` - Custom data for this consumption cost, or None to clear
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
    use crate::v2_1::enumerations::CostKindEnumType;

    #[test]
    fn test_new_consumption_cost() {
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = ConsumptionCostType::new(Decimal::new(100, 1), vec![cost.clone()]);

        assert_eq!(consumption_cost.start_value(), Decimal::new(100, 1));
        assert_eq!(consumption_cost.cost().len(), 1);
        assert_eq!(
            consumption_cost.cost()[0].cost_kind(),
            &CostKindEnumType::CarbonDioxideEmission
        );
        assert_eq!(consumption_cost.cost()[0].amount(), 100);
        assert_eq!(consumption_cost.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let consumption_cost = ConsumptionCostType::new(Decimal::new(100, 1), vec![cost.clone()])
            .with_custom_data(custom_data.clone());

        assert_eq!(consumption_cost.start_value(), Decimal::new(100, 1));
        assert_eq!(consumption_cost.cost().len(), 1);
        assert_eq!(consumption_cost.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let cost1 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 200);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut consumption_cost =
            ConsumptionCostType::new(Decimal::new(100, 1), vec![cost1.clone()]);

        consumption_cost
            .set_start_value(Decimal::new(200, 1))
            .set_cost(vec![cost1.clone(), cost2.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(consumption_cost.start_value(), Decimal::new(200, 1));
        assert_eq!(consumption_cost.cost().len(), 2);
        assert_eq!(
            consumption_cost.cost()[0].cost_kind(),
            &CostKindEnumType::CarbonDioxideEmission
        );
        assert_eq!(
            consumption_cost.cost()[1].cost_kind(),
            &CostKindEnumType::RelativePricePercentage
        );
        assert_eq!(consumption_cost.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        consumption_cost.set_custom_data(None);
        assert_eq!(consumption_cost.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        use serde_json::{json, Value};

        let cost1 =
            CostType::new(CostKindEnumType::CarbonDioxideEmission, 100).with_amount_multiplier(-2);
        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 200);
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let consumption_cost =
            ConsumptionCostType::new(Decimal::try_from(15.5).unwrap(), vec![cost1, cost2])
                .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&consumption_cost).unwrap();

        // Deserialize back and check equality
        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(consumption_cost, deserialized);

        // Check specific JSON structure
        let json_value: Value = serde_json::from_str(&serialized).unwrap();
        let start_value = &json_value["startValue"];
        // Check if the value is close to 15.5, accounting for different possible JSON representations
        let value_check = if let Some(num) = start_value.as_f64() {
            num > 15.4 && num < 15.6
        } else if let Some(num) = start_value.as_str() {
            num == "15.5"
        } else {
            // Also handle possibility of being represented as string or integer
            start_value.to_string().contains("15.5")
        };
        assert!(
            value_check,
            "startValue should be close to 15.5, got {:?}",
            start_value
        );
        assert_eq!(json_value["cost"].as_array().unwrap().len(), 2);
        assert_eq!(json_value["cost"][0]["costKind"], "CarbonDioxideEmission");
        assert_eq!(json_value["cost"][0]["amount"], 100);
        assert_eq!(json_value["cost"][0]["amountMultiplier"], -2);
        assert_eq!(json_value["cost"][1]["costKind"], "RelativePricePercentage");
        assert_eq!(json_value["cost"][1]["amount"], 200);
        assert_eq!(json_value["customData"]["vendorId"], "VendorX");
        assert_eq!(json_value["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization() {
        let json_str = r#"{
            "startValue": 25.5,
            "cost": [
                {
                    "costKind": "CarbonDioxideEmission",
                    "amount": 100,
                    "amountMultiplier": -1
                },
                {
                    "costKind": "RenewableGenerationPercentage",
                    "amount": 80
                }
            ],
            "customData": {
                "vendorId": "TestVendor",
                "extraInfo": "Something"
            }
        }"#;

        let consumption_cost: ConsumptionCostType = serde_json::from_str(json_str).unwrap();

        let expected = Decimal::try_from(25.5).unwrap();
        assert_eq!(consumption_cost.start_value(), expected);
        assert_eq!(consumption_cost.cost().len(), 2);

        assert_eq!(
            consumption_cost.cost()[0].cost_kind(),
            &CostKindEnumType::CarbonDioxideEmission
        );
        assert_eq!(consumption_cost.cost()[0].amount(), 100);
        assert_eq!(consumption_cost.cost()[0].amount_multiplier(), Some(-1));

        assert_eq!(
            consumption_cost.cost()[1].cost_kind(),
            &CostKindEnumType::RenewableGenerationPercentage
        );
        assert_eq!(consumption_cost.cost()[1].amount(), 80);
        assert_eq!(consumption_cost.cost()[1].amount_multiplier(), None);

        assert_eq!(
            consumption_cost.custom_data().unwrap().vendor_id(),
            "TestVendor"
        );

        use serde_json::json;
        assert_eq!(
            consumption_cost
                .custom_data()
                .unwrap()
                .additional_properties()["extraInfo"],
            json!("Something")
        );
    }

    #[test]
    fn test_validation() {
        use validator::Validate;

        // Valid case - one cost element
        let cost1 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost1 = ConsumptionCostType::new(Decimal::new(100, 1), vec![cost1.clone()]);
        assert!(
            consumption_cost1.validate().is_ok(),
            "Consumption cost with one valid cost element should pass validation"
        );

        // Valid case - maximum of 3 cost elements
        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 200);
        let cost3 = CostType::new(CostKindEnumType::RenewableGenerationPercentage, 300);
        let consumption_cost3 = ConsumptionCostType::new(
            Decimal::new(100, 1),
            vec![cost1.clone(), cost2.clone(), cost3.clone()],
        );
        assert!(
            consumption_cost3.validate().is_ok(),
            "Consumption cost with three valid cost elements should pass validation"
        );

        // Invalid case - empty cost vector
        let consumption_cost_empty = ConsumptionCostType::new(Decimal::new(100, 1), vec![]);
        assert!(
            consumption_cost_empty.validate().is_err(),
            "Consumption cost with empty cost vector should fail validation"
        );

        // Invalid case - too many elements (more than 3)
        let cost1_dup = CostType::new(CostKindEnumType::CarbonDioxideEmission, 101);
        let consumption_cost_too_many = ConsumptionCostType::new(
            Decimal::new(100, 1),
            vec![
                cost1.clone(),
                cost2.clone(),
                cost3.clone(),
                cost1_dup.clone(),
            ],
        );
        assert!(
            consumption_cost_too_many.validate().is_err(),
            "Consumption cost with more than 3 elements should fail validation"
        );

        // Invalid nested validation - cost with invalid amount_multiplier
        let invalid_cost =
            CostType::new(CostKindEnumType::CarbonDioxideEmission, 100).with_amount_multiplier(4); // Should be in range -3 to 3
        let consumption_cost_invalid_nested =
            ConsumptionCostType::new(Decimal::new(100, 1), vec![invalid_cost]);
        assert!(
            consumption_cost_invalid_nested.validate().is_err(),
            "Consumption cost with invalid nested cost should fail validation"
        );
    }

    #[test]
    fn test_complex_scenario() {
        use serde_json::json;

        // Create costs with different types and multipliers
        let cost1 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100)
            .with_amount_multiplier(-2)
            .with_custom_data(
                CustomDataType::new("VendorA".to_string())
                    .with_property("unit".to_string(), json!("g/kWh")),
            );

        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 120)
            .with_amount_multiplier(0)
            .with_custom_data(
                CustomDataType::new("VendorB".to_string())
                    .with_property("baseline".to_string(), json!(100)),
            );

        let cost3 = CostType::new(CostKindEnumType::RenewableGenerationPercentage, 85);

        // Create consumption cost with custom data
        let custom_data = CustomDataType::new("VendorConsumption".to_string()).with_property(
            "info".to_string(),
            json!({
                "region": "Europe",
                "provider": "EnergyX",
                "validUntil": "2023-12-31"
            }),
        );

        let consumption_cost =
            ConsumptionCostType::new(Decimal::try_from(25.75).unwrap(), vec![cost1, cost2, cost3])
                .with_custom_data(custom_data);

        // Serialize and deserialize
        let serialized = serde_json::to_string(&consumption_cost).unwrap();
        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();

        // Basic validation
        assert_eq!(consumption_cost, deserialized);
        let expected = Decimal::try_from(25.75).unwrap();
        assert_eq!(deserialized.start_value(), expected);
        assert_eq!(deserialized.cost().len(), 3);

        // Detailed validation of each cost element
        let costs = deserialized.cost();

        // Cost 1
        assert_eq!(
            costs[0].cost_kind(),
            &CostKindEnumType::CarbonDioxideEmission
        );
        assert_eq!(costs[0].amount(), 100);
        assert_eq!(costs[0].amount_multiplier(), Some(-2));
        let cost1_custom = costs[0].custom_data().unwrap();
        assert_eq!(cost1_custom.vendor_id(), "VendorA");
        assert_eq!(cost1_custom.additional_properties()["unit"], "g/kWh");

        // Cost 2
        assert_eq!(
            costs[1].cost_kind(),
            &CostKindEnumType::RelativePricePercentage
        );
        assert_eq!(costs[1].amount(), 120);
        assert_eq!(costs[1].amount_multiplier(), Some(0));
        let cost2_custom = costs[1].custom_data().unwrap();
        assert_eq!(cost2_custom.vendor_id(), "VendorB");
        assert_eq!(cost2_custom.additional_properties()["baseline"], 100);

        // Cost 3
        assert_eq!(
            costs[2].cost_kind(),
            &CostKindEnumType::RenewableGenerationPercentage
        );
        assert_eq!(costs[2].amount(), 85);
        assert_eq!(costs[2].amount_multiplier(), None);

        // Consumption custom data
        let consumption_custom = deserialized.custom_data().unwrap();
        assert_eq!(consumption_custom.vendor_id(), "VendorConsumption");
        let info = &consumption_custom.additional_properties()["info"];
        assert_eq!(info["region"], "Europe");
        assert_eq!(info["provider"], "EnergyX");
        assert_eq!(info["validUntil"], "2023-12-31");
    }

    #[test]
    fn test_decimal_precision() {
        // Test that the Decimal type preserves precision correctly
        let consumption_cost = ConsumptionCostType::new(
            Decimal::try_from(123.456).unwrap(),
            vec![CostType::new(CostKindEnumType::CarbonDioxideEmission, 100)],
        );

        // Serialize and deserialize
        let serialized = serde_json::to_string(&consumption_cost).unwrap();
        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();

        // The exact decimal value should be preserved
        let expected = Decimal::try_from(123.456).unwrap();
        assert_eq!(consumption_cost.start_value(), expected);
        assert_eq!(deserialized.start_value(), expected);

        // Test very small number
        let small_value = ConsumptionCostType::new(
            Decimal::try_from(0.0001).unwrap(),
            vec![CostType::new(CostKindEnumType::CarbonDioxideEmission, 100)],
        );

        let serialized = serde_json::to_string(&small_value).unwrap();
        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();

        let expected = Decimal::try_from(0.0001).unwrap();
        assert_eq!(small_value.start_value(), expected);
        assert_eq!(deserialized.start_value(), expected);

        // Test very large number
        let large_value = ConsumptionCostType::new(
            Decimal::try_from(9999999.9999).unwrap(),
            vec![CostType::new(CostKindEnumType::CarbonDioxideEmission, 100)],
        );

        let serialized = serde_json::to_string(&large_value).unwrap();
        let deserialized: ConsumptionCostType = serde_json::from_str(&serialized).unwrap();

        let expected = Decimal::try_from(9999999.9999).unwrap();
        assert_eq!(large_value.start_value(), expected);
        assert_eq!(deserialized.start_value(), expected);
    }

    #[test]
    fn test_validation_errors_content() {
        use validator::Validate;

        // Test validation error for empty cost vector
        let consumption_cost_empty = ConsumptionCostType::new(Decimal::new(100, 1), vec![]);

        let validation_result = consumption_cost_empty.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the cost field for length validation
        assert!(
            field_errors.contains_key("cost"),
            "Validation errors should contain cost field"
        );
        let cost_errors = &field_errors["cost"];
        assert!(
            !cost_errors.is_empty(),
            "Cost field should have validation errors"
        );
        assert_eq!(
            cost_errors[0].code, "length",
            "Cost field should have a length error"
        );

        // Test validation error for too many elements
        let cost1 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let cost2 = CostType::new(CostKindEnumType::RelativePricePercentage, 200);
        let cost3 = CostType::new(CostKindEnumType::RenewableGenerationPercentage, 300);
        let cost4 = CostType::new(CostKindEnumType::CarbonDioxideEmission, 400);

        let consumption_cost_too_many =
            ConsumptionCostType::new(Decimal::new(100, 1), vec![cost1, cost2, cost3, cost4]);

        let validation_result = consumption_cost_too_many.validate();
        assert!(validation_result.is_err());

        let errors = validation_result.unwrap_err();
        let field_errors = errors.field_errors();

        // Verify error is on the cost field for length validation
        assert!(
            field_errors.contains_key("cost"),
            "Validation errors should contain cost field"
        );
        let cost_errors = &field_errors["cost"];
        assert!(
            !cost_errors.is_empty(),
            "Cost field should have validation errors"
        );
        assert_eq!(
            cost_errors[0].code, "length",
            "Cost field should have a length error"
        );
    }

    #[test]
    fn test_nested_validation() {
        use validator::Validate;

        // Create a cost with invalid amount_multiplier (outside -3 to 3 range)
        let invalid_cost =
            CostType::new(CostKindEnumType::CarbonDioxideEmission, 100).with_amount_multiplier(4); // Should be in range -3 to 3

        let consumption_cost = ConsumptionCostType::new(Decimal::new(100, 1), vec![invalid_cost]);

        // Validation should fail due to nested validation
        let validation_result = consumption_cost.validate();
        assert!(
            validation_result.is_err(),
            "Validation should fail with invalid nested cost"
        );

        // Only assert that validation fails, without checking specific error structure
        // Since error reporting for nested validation can vary depending on validator implementation
        println!("Validation errors: {:?}", validation_result.unwrap_err());
    }

    #[test]
    fn test_custom_data_validation() {
        use validator::Validate;

        // Create a cost element
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);

        // Create custom data with invalid vendor_id (too long)
        let too_long_vendor_id = "X".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let consumption_cost = ConsumptionCostType::new(Decimal::new(100, 1), vec![cost])
            .with_custom_data(invalid_custom_data);

        // Validation should fail due to invalid custom_data
        let validation_result = consumption_cost.validate();
        assert!(
            validation_result.is_err(),
            "Invalid custom_data should cause validation failure"
        );
    }
}
