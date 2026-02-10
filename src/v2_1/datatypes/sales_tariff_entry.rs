use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    consumption_cost::ConsumptionCostType, custom_data::CustomDataType,
    relative_time_interval::RelativeTimeIntervalType,
};

/// Sales tariff entry details.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffEntryType {
    /// Required. Time and date at which the tariff becomes valid.
    #[validate(nested)]
    pub relative_time_interval: RelativeTimeIntervalType,

    /// Optional. Defines the price level of this SalesTariffEntry (referring to NumEPriceLevels).
    /// Small values for the EPriceLevel represent a cheaper TariffEntry.
    /// Large values for the EPriceLevel represent a more expensive TariffEntry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub e_price_level: Option<i32>,

    /// Optional. Consumption cost per time interval.
    /// When present, must contain at least 1 and at most 3 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 3), nested)]
    pub consumption_cost: Option<Vec<ConsumptionCostType>>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SalesTariffEntryType {
    /// Creates a new `SalesTariffEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `relative_time_interval` - Time and date at which the tariff becomes valid
    ///
    /// # Returns
    ///
    /// A new instance of `SalesTariffEntryType` with optional fields set to `None`
    pub fn new(relative_time_interval: RelativeTimeIntervalType) -> Self {
        Self {
            relative_time_interval,
            e_price_level: None,
            consumption_cost: None,
            custom_data: None,
        }
    }

    /// Sets the price level.
    ///
    /// # Arguments
    ///
    /// * `e_price_level` - Defines the price level of this SalesTariffEntry.
    ///   Small values represent a cheaper TariffEntry, large values represent a more expensive TariffEntry.
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_e_price_level(mut self, e_price_level: i32) -> Self {
        self.e_price_level = Some(e_price_level);
        self
    }

    /// Sets the consumption cost.
    ///
    /// # Arguments
    ///
    /// * `consumption_cost` - Consumption cost per time interval (1-3 items)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_consumption_cost(mut self, consumption_cost: Vec<ConsumptionCostType>) -> Self {
        self.consumption_cost = Some(consumption_cost);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sales tariff entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the relative time interval.
    ///
    /// # Returns
    ///
    /// A reference to the time and date at which the tariff becomes valid
    pub fn relative_time_interval(&self) -> &RelativeTimeIntervalType {
        &self.relative_time_interval
    }

    /// Sets the relative time interval.
    ///
    /// # Arguments
    ///
    /// * `relative_time_interval` - Time and date at which the tariff becomes valid
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_relative_time_interval(
        &mut self,
        relative_time_interval: RelativeTimeIntervalType,
    ) -> &mut Self {
        self.relative_time_interval = relative_time_interval;
        self
    }

    /// Gets the price level.
    ///
    /// # Returns
    ///
    /// An optional price level value
    pub fn e_price_level(&self) -> Option<i32> {
        self.e_price_level
    }

    /// Sets the price level.
    ///
    /// # Arguments
    ///
    /// * `e_price_level` - Defines the price level of this SalesTariffEntry, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_e_price_level(&mut self, e_price_level: Option<i32>) -> &mut Self {
        self.e_price_level = e_price_level;
        self
    }

    /// Gets the consumption cost.
    ///
    /// # Returns
    ///
    /// An optional reference to consumption cost per time interval
    pub fn consumption_cost(&self) -> Option<&Vec<ConsumptionCostType>> {
        self.consumption_cost.as_ref()
    }

    /// Sets the consumption cost.
    ///
    /// # Arguments
    ///
    /// * `consumption_cost` - Consumption cost per time interval (1-3 items), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_consumption_cost(
        &mut self,
        consumption_cost: Option<Vec<ConsumptionCostType>>,
    ) -> &mut Self {
        self.consumption_cost = consumption_cost;
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
    /// * `custom_data` - Custom data for this sales tariff entry, or None to clear
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
    use crate::v2_1::datatypes::cost::CostType;
    use crate::v2_1::enumerations::CostKindEnumType;
    use rust_decimal::Decimal;

    #[test]
    fn test_new_sales_tariff_entry() {
        let interval = RelativeTimeIntervalType::new_default();
        let entry = SalesTariffEntryType::new(interval.clone());

        assert_eq!(entry.relative_time_interval(), &interval);
        assert_eq!(entry.e_price_level(), None);
        assert_eq!(entry.consumption_cost(), None);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let interval = RelativeTimeIntervalType::new_default();
        let price_level = 3;
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = vec![ConsumptionCostType::new(
            Decimal::new(100, 1),
            vec![cost.clone()],
        )];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let entry = SalesTariffEntryType::new(interval.clone())
            .with_e_price_level(price_level)
            .with_consumption_cost(consumption_cost.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.relative_time_interval(), &interval);
        assert_eq!(entry.e_price_level(), Some(price_level));
        assert_eq!(entry.consumption_cost().unwrap().len(), 1);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let interval1 = RelativeTimeIntervalType::new_default();
        let mut entry = SalesTariffEntryType::new(interval1.clone());

        let interval2 = RelativeTimeIntervalType::new(10, 0);
        let price_level = 5;
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = vec![ConsumptionCostType::new(
            Decimal::new(100, 1),
            vec![cost.clone()],
        )];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        entry
            .set_relative_time_interval(interval2.clone())
            .set_e_price_level(Some(price_level))
            .set_consumption_cost(Some(consumption_cost.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.relative_time_interval(), &interval2);
        assert_eq!(entry.e_price_level(), Some(price_level));
        assert_eq!(entry.consumption_cost().unwrap().len(), 1);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry
            .set_e_price_level(None)
            .set_consumption_cost(None)
            .set_custom_data(None);

        assert_eq!(entry.e_price_level(), None);
        assert_eq!(entry.consumption_cost(), None);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid entry
        let interval = RelativeTimeIntervalType::new_default();
        let cost = CostType::new(CostKindEnumType::CarbonDioxideEmission, 100);
        let consumption_cost = vec![ConsumptionCostType::new(
            Decimal::new(100, 1),
            vec![cost.clone()],
        )];

        let valid_entry = SalesTariffEntryType::new(interval.clone())
            .with_e_price_level(3)
            .with_consumption_cost(consumption_cost.clone());

        // Test with negative price level (should fail validation)
        let mut invalid_entry = valid_entry.clone();
        invalid_entry.e_price_level = Some(-1);

        // Test with empty consumption cost array (should fail validation)
        let mut invalid_entry2 = valid_entry.clone();
        invalid_entry2.consumption_cost = Some(vec![]);

        // Test with too many consumption cost items (should fail validation)
        let mut invalid_entry3 = valid_entry;
        let cost_items = vec![
            ConsumptionCostType::new(Decimal::new(100, 1), vec![cost.clone()]),
            ConsumptionCostType::new(Decimal::new(200, 1), vec![cost.clone()]),
            ConsumptionCostType::new(Decimal::new(300, 1), vec![cost.clone()]),
            ConsumptionCostType::new(Decimal::new(400, 1), vec![cost]),
        ];
        invalid_entry3.consumption_cost = Some(cost_items);
    }
}
