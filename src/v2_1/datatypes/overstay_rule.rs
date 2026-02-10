use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, rational_number::RationalNumberType};

/// Rule that describes the pricing of overstaying.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Time in seconds after trigger of the parent Overstay Rules for this particular fee to apply.
    pub start_time: i32,

    /// Required. Time till overstay will be reapplied in seconds.
    pub overstay_fee_period: i32,

    /// Required. Fee applied for overstaying.
    pub overstay_fee: RationalNumberType,

    /// Optional. Human readable string to identify the overstay rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 32))]
    pub overstay_rule_description: Option<String>,
}

impl OverstayRuleType {
    /// Creates a new `OverstayRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time in seconds after trigger of the parent Overstay Rules for this particular fee to apply
    /// * `overstay_fee_period` - Time till overstay will be reapplied in seconds
    /// * `overstay_fee` - Fee applied for overstaying
    ///
    /// # Returns
    ///
    /// A new instance of `OverstayRuleType` with optional fields set to `None`
    pub fn new(
        start_time: i32,
        overstay_fee_period: i32,
        overstay_fee: RationalNumberType,
    ) -> Self {
        Self {
            start_time,
            overstay_fee_period,
            overstay_fee,
            custom_data: None,
            overstay_rule_description: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this overstay rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the overstay rule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Human readable string to identify the overstay rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_overstay_rule_description(mut self, description: String) -> Self {
        self.overstay_rule_description = Some(description);
        self
    }

    /// Gets the start time.
    ///
    /// # Returns
    ///
    /// The time in seconds after trigger of the parent Overstay Rules for this particular fee to apply
    pub fn start_time(&self) -> i32 {
        self.start_time
    }

    /// Sets the start time.
    ///
    /// # Arguments
    ///
    /// * `start_time` - Time in seconds after trigger of the parent Overstay Rules for this particular fee to apply
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_time(&mut self, start_time: i32) -> &mut Self {
        self.start_time = start_time;
        self
    }

    /// Gets the overstay fee period.
    ///
    /// # Returns
    ///
    /// The time till overstay will be reapplied in seconds
    pub fn overstay_fee_period(&self) -> i32 {
        self.overstay_fee_period
    }

    /// Sets the overstay fee period.
    ///
    /// # Arguments
    ///
    /// * `overstay_fee_period` - Time till overstay will be reapplied in seconds
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_overstay_fee_period(&mut self, overstay_fee_period: i32) -> &mut Self {
        self.overstay_fee_period = overstay_fee_period;
        self
    }

    /// Gets the overstay fee.
    ///
    /// # Returns
    ///
    /// The fee applied for overstaying
    pub fn overstay_fee(&self) -> &RationalNumberType {
        &self.overstay_fee
    }

    /// Sets the overstay fee.
    ///
    /// # Arguments
    ///
    /// * `overstay_fee` - Fee applied for overstaying
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_overstay_fee(&mut self, overstay_fee: RationalNumberType) -> &mut Self {
        self.overstay_fee = overstay_fee;
        self
    }

    /// Gets the overstay rule description.
    ///
    /// # Returns
    ///
    /// An optional reference to the human readable string identifying the overstay rule
    pub fn overstay_rule_description(&self) -> Option<&String> {
        self.overstay_rule_description.as_ref()
    }

    /// Sets the overstay rule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Human readable string to identify the overstay rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_overstay_rule_description(&mut self, description: Option<String>) -> &mut Self {
        self.overstay_rule_description = description;
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
    /// * `custom_data` - Custom data for this overstay rule, or None to clear
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
    use crate::v2_1::datatypes::rational_number::RationalNumberType;

    #[test]
    fn test_new_overstay_rule() {
        let start_time = 3600;
        let fee_period = 1800;
        let fee = RationalNumberType::new(0, 5);

        let rule = OverstayRuleType::new(start_time, fee_period, fee.clone());

        assert_eq!(rule.start_time(), start_time);
        assert_eq!(rule.overstay_fee_period(), fee_period);
        assert_eq!(rule.overstay_fee(), &fee);
        assert_eq!(rule.custom_data(), None);
        assert_eq!(rule.overstay_rule_description(), None);
    }

    #[test]
    fn test_with_methods() {
        let start_time = 3600;
        let fee_period = 1800;
        let fee = RationalNumberType::new(0, 5);
        let custom_data = CustomDataType::new("VendorX".to_string());
        let description = "Overstay Penalty".to_string();

        let rule = OverstayRuleType::new(start_time, fee_period, fee.clone())
            .with_custom_data(custom_data.clone())
            .with_overstay_rule_description(description.clone());

        assert_eq!(rule.start_time(), start_time);
        assert_eq!(rule.overstay_fee_period(), fee_period);
        assert_eq!(rule.overstay_fee(), &fee);
        assert_eq!(rule.custom_data(), Some(&custom_data));
        assert_eq!(rule.overstay_rule_description(), Some(&description));
    }

    #[test]
    fn test_setter_methods() {
        let start_time1 = 3600;
        let fee_period1 = 1800;
        let fee1 = RationalNumberType::new(0, 5);
        let start_time2 = 7200;
        let fee_period2 = 3600;
        let fee2 = RationalNumberType::new(0, 10);
        let custom_data = CustomDataType::new("VendorX".to_string());
        let description = "Overstay Penalty".to_string();

        let mut rule = OverstayRuleType::new(start_time1, fee_period1, fee1.clone());

        rule.set_start_time(start_time2)
            .set_overstay_fee_period(fee_period2)
            .set_overstay_fee(fee2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_overstay_rule_description(Some(description.clone()));

        assert_eq!(rule.start_time(), start_time2);
        assert_eq!(rule.overstay_fee_period(), fee_period2);
        assert_eq!(rule.overstay_fee(), &fee2);
        assert_eq!(rule.custom_data(), Some(&custom_data));
        assert_eq!(rule.overstay_rule_description(), Some(&description));

        // Test clearing optional fields
        rule.set_custom_data(None);
        rule.set_overstay_rule_description(None);
        assert_eq!(rule.custom_data(), None);
        assert_eq!(rule.overstay_rule_description(), None);
    }
}
