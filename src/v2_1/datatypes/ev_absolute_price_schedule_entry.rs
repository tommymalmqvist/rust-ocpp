use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::ev_price_rule::EVPriceRuleType;

/// Entry in the EVAbsolutePriceSchedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVAbsolutePriceScheduleEntryType {
    /// Duration of the schedule entry in seconds.
    pub duration: i32,

    /// Price rules for different power ranges.
    #[validate(length(min = 1, max = 8))]
    pub ev_price_rules: Vec<EVPriceRuleType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl EVAbsolutePriceScheduleEntryType {
    /// Creates a new `EVAbsolutePriceScheduleEntryType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `ev_price_rules` - Vector of price rules for different power ranges
    ///
    /// # Returns
    ///
    /// A new instance of `EVAbsolutePriceScheduleEntryType` with optional fields set to `None`
    pub fn new(duration: i32, ev_price_rules: Vec<EVPriceRuleType>) -> Self {
        Self {
            duration,
            ev_price_rules,
            custom_data: None,
        }
    }

    /// Creates a new `EVAbsolutePriceScheduleEntryType` with a single price rule.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    /// * `energy_fee` - Energy fee in the specified currency
    /// * `power_range_start` - Start of the power range in Watts (W)
    ///
    /// # Returns
    ///
    /// A new instance of `EVAbsolutePriceScheduleEntryType` with a single price rule
    pub fn new_with_single_price(duration: i32, energy_fee: f64, power_range_start: f64) -> Self {
        let price_rule = EVPriceRuleType::new(energy_fee, power_range_start);
        Self::new(duration, vec![price_rule])
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this schedule entry
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the duration.
    ///
    /// # Returns
    ///
    /// The duration of the schedule entry in seconds
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Sets the duration.
    ///
    /// # Arguments
    ///
    /// * `duration` - Duration of the schedule entry in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_duration(&mut self, duration: i32) -> &mut Self {
        self.duration = duration;
        self
    }

    /// Gets the price rules.
    ///
    /// # Returns
    ///
    /// A reference to the vector of price rules
    pub fn ev_price_rules(&self) -> &Vec<EVPriceRuleType> {
        &self.ev_price_rules
    }

    /// Sets the price rules.
    ///
    /// # Arguments
    ///
    /// * `ev_price_rules` - Vector of price rules
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_price_rules(&mut self, ev_price_rules: Vec<EVPriceRuleType>) -> &mut Self {
        self.ev_price_rules = ev_price_rules;
        self
    }

    /// Adds a price rule to the existing rules.
    ///
    /// # Arguments
    ///
    /// * `price_rule` - Price rule to add
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn add_price_rule(&mut self, price_rule: EVPriceRuleType) -> &mut Self {
        self.ev_price_rules.push(price_rule);
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
    /// * `custom_data` - Custom data for this schedule entry, or None to clear
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
    fn test_new_ev_absolute_price_schedule_entry() {
        let duration = 3600;
        let price_rule1 = EVPriceRuleType::new(0.25, 0.0);
        let price_rule2 = EVPriceRuleType::new(0.20, 10000.0);
        let ev_price_rules = vec![price_rule1, price_rule2];

        let entry = EVAbsolutePriceScheduleEntryType::new(duration, ev_price_rules.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.ev_price_rules(), &ev_price_rules);
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_new_with_single_price() {
        let duration = 3600;
        let energy_fee = 0.25;
        let power_range_start = 0.0;

        let entry = EVAbsolutePriceScheduleEntryType::new_with_single_price(
            duration,
            energy_fee,
            power_range_start,
        );

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.ev_price_rules().len(), 1);
        assert_eq!(entry.ev_price_rules()[0].energy_fee_as_f64(), energy_fee);
        assert_eq!(
            entry.ev_price_rules()[0].power_range_start_as_f64(),
            power_range_start
        );
        assert_eq!(entry.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let duration = 3600;
        let price_rule = EVPriceRuleType::new(0.25, 0.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let entry = EVAbsolutePriceScheduleEntryType::new(duration, vec![price_rule])
            .with_custom_data(custom_data.clone());

        assert_eq!(entry.duration(), duration);
        assert_eq!(entry.ev_price_rules().len(), 1);
        assert_eq!(entry.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_add_price_rule() {
        let duration = 3600;
        let price_rule1 = EVPriceRuleType::new(0.25, 0.0);
        let price_rule2 = EVPriceRuleType::new(0.20, 10000.0);

        let mut entry = EVAbsolutePriceScheduleEntryType::new(duration, vec![price_rule1.clone()]);
        assert_eq!(entry.ev_price_rules().len(), 1);

        entry.add_price_rule(price_rule2.clone());

        assert_eq!(entry.ev_price_rules().len(), 2);
        assert_eq!(entry.ev_price_rules()[0], price_rule1);
        assert_eq!(entry.ev_price_rules()[1], price_rule2);
    }

    #[test]
    fn test_setter_methods() {
        let duration1 = 3600;
        let duration2 = 7200;
        let price_rule1 = EVPriceRuleType::new(0.25, 0.0);
        let price_rule2 = EVPriceRuleType::new(0.20, 5000.0);
        let price_rule3 = EVPriceRuleType::new(0.15, 10000.0);
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut entry = EVAbsolutePriceScheduleEntryType::new(duration1, vec![price_rule1.clone()]);

        entry
            .set_duration(duration2)
            .set_ev_price_rules(vec![price_rule2.clone(), price_rule3.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(entry.duration(), duration2);
        assert_eq!(entry.ev_price_rules().len(), 2);
        assert_eq!(entry.ev_price_rules()[0], price_rule2);
        assert_eq!(entry.ev_price_rules()[1], price_rule3);
        assert_eq!(entry.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        entry.set_custom_data(None);
        assert_eq!(entry.custom_data(), None);
    }
}
