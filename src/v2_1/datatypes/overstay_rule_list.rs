use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, overstay_rule::OverstayRuleType,
    rational_number::RationalNumberType,
};

/// List of overstay rules for a charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OverstayRuleListType {
    /// Required. List of overstay rules.
    #[validate(length(min = 1, max = 5))]
    #[validate(nested)]
    pub overstay_rule: Vec<OverstayRuleType>,

    /// Optional. Power threshold for overstay rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub overstay_power_threshold: Option<RationalNumberType>,

    /// Optional. Time till overstay is applied in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overstay_time_threshold: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl OverstayRuleListType {
    /// Creates a new `OverstayRuleListType` with the required fields.
    ///
    /// # Arguments
    ///
    /// * `overstay_rule` - List of overstay rules
    ///
    /// # Returns
    ///
    /// A new `OverstayRuleListType` instance with optional fields set to `None`
    pub fn new(overstay_rule: Vec<OverstayRuleType>) -> Self {
        Self {
            custom_data: None,
            overstay_rule,
            overstay_power_threshold: None,
            overstay_time_threshold: None,
        }
    }

    /// Sets the custom data field.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the overstay power threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_power_threshold` - Power threshold for overstay rules
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn with_overstay_power_threshold(
        mut self,
        overstay_power_threshold: RationalNumberType,
    ) -> Self {
        self.overstay_power_threshold = Some(overstay_power_threshold);
        self
    }

    /// Sets the overstay time threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_time_threshold` - Time till overstay is applied in seconds
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn with_overstay_time_threshold(mut self, overstay_time_threshold: i32) -> Self {
        self.overstay_time_threshold = Some(overstay_time_threshold);
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

    /// Gets the overstay rules.
    ///
    /// # Returns
    ///
    /// A reference to the list of overstay rules
    pub fn overstay_rule(&self) -> &Vec<OverstayRuleType> {
        &self.overstay_rule
    }

    /// Gets the overstay power threshold.
    ///
    /// # Returns
    ///
    /// An optional reference to the overstay power threshold
    pub fn overstay_power_threshold(&self) -> Option<&RationalNumberType> {
        self.overstay_power_threshold.as_ref()
    }

    /// Gets the overstay time threshold.
    ///
    /// # Returns
    ///
    /// An optional reference to the overstay time threshold
    pub fn overstay_time_threshold(&self) -> Option<&i32> {
        self.overstay_time_threshold.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data from the Charging Station, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the overstay rules.
    ///
    /// # Arguments
    ///
    /// * `overstay_rule` - List of overstay rules
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_overstay_rule(&mut self, overstay_rule: Vec<OverstayRuleType>) -> &mut Self {
        self.overstay_rule = overstay_rule;
        self
    }

    /// Sets the overstay power threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_power_threshold` - Power threshold for overstay rules, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_overstay_power_threshold(
        &mut self,
        overstay_power_threshold: Option<RationalNumberType>,
    ) -> &mut Self {
        self.overstay_power_threshold = overstay_power_threshold;
        self
    }

    /// Sets the overstay time threshold.
    ///
    /// # Arguments
    ///
    /// * `overstay_time_threshold` - Time till overstay is applied in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// The modified `OverstayRuleListType` instance
    pub fn set_overstay_time_threshold(
        &mut self,
        overstay_time_threshold: Option<i32>,
    ) -> &mut Self {
        self.overstay_time_threshold = overstay_time_threshold;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::datatypes::{
        overstay_rule::OverstayRuleType, rational_number::RationalNumberType,
    };

    #[test]
    fn test_overstay_rule_list_new() {
        let overstay_rule = OverstayRuleType::new(10, 3600, RationalNumberType::new(0, 5));
        let overstay_rules = vec![overstay_rule.clone()];
        let overstay_rule_list = OverstayRuleListType::new(overstay_rules.clone());

        assert_eq!(overstay_rule_list.overstay_rule(), &overstay_rules);
        assert_eq!(overstay_rule_list.custom_data(), None);
        assert_eq!(overstay_rule_list.overstay_power_threshold(), None);
        assert_eq!(overstay_rule_list.overstay_time_threshold(), None);
    }

    #[test]
    fn test_overstay_rule_list_with_methods() {
        let overstay_rule = OverstayRuleType::new(10, 3600, RationalNumberType::new(0, 5));
        let overstay_rules = vec![overstay_rule.clone()];
        let custom_data = CustomDataType::new("VendorX".to_string());
        let power_threshold = RationalNumberType::new(0, 100);
        let time_threshold = 1800;

        let overstay_rule_list = OverstayRuleListType::new(overstay_rules.clone())
            .with_custom_data(custom_data.clone())
            .with_overstay_power_threshold(power_threshold.clone())
            .with_overstay_time_threshold(time_threshold);

        assert_eq!(overstay_rule_list.overstay_rule(), &overstay_rules);
        assert_eq!(overstay_rule_list.custom_data(), Some(&custom_data));
        assert_eq!(
            overstay_rule_list.overstay_power_threshold(),
            Some(&power_threshold)
        );
        assert_eq!(
            overstay_rule_list.overstay_time_threshold(),
            Some(&time_threshold)
        );
    }

    #[test]
    fn test_overstay_rule_list_setters() {
        let overstay_rule1 = OverstayRuleType::new(10, 3600, RationalNumberType::new(0, 5));
        let overstay_rules1 = vec![overstay_rule1.clone()];

        let overstay_rule2 = OverstayRuleType::new(20, 7200, RationalNumberType::new(0, 10));
        let overstay_rules2 = vec![overstay_rule2.clone()];

        let custom_data = CustomDataType::new("VendorX".to_string());
        let power_threshold = RationalNumberType::new(0, 100);
        let time_threshold = 1800;

        let mut overstay_rule_list = OverstayRuleListType::new(overstay_rules1.clone());

        overstay_rule_list
            .set_overstay_rule(overstay_rules2.clone())
            .set_custom_data(Some(custom_data.clone()))
            .set_overstay_power_threshold(Some(power_threshold.clone()))
            .set_overstay_time_threshold(Some(time_threshold));

        assert_eq!(overstay_rule_list.overstay_rule(), &overstay_rules2);
        assert_eq!(overstay_rule_list.custom_data(), Some(&custom_data));
        assert_eq!(
            overstay_rule_list.overstay_power_threshold(),
            Some(&power_threshold)
        );
        assert_eq!(
            overstay_rule_list.overstay_time_threshold(),
            Some(&time_threshold)
        );

        // Test clearing optional fields
        overstay_rule_list.set_custom_data(None);
        overstay_rule_list.set_overstay_power_threshold(None);
        overstay_rule_list.set_overstay_time_threshold(None);
        assert_eq!(overstay_rule_list.custom_data(), None);
        assert_eq!(overstay_rule_list.overstay_power_threshold(), None);
        assert_eq!(overstay_rule_list.overstay_time_threshold(), None);
    }

    #[test]
    fn test_overstay_rule_list_validation_max_length() {
        let overstay_rule = OverstayRuleType::new(10, 3600, RationalNumberType::new(0, 5));
        let overstay_rules = vec![overstay_rule.clone(); 6]; // Exceeds max length of 5
        let overstay_rule_list = OverstayRuleListType::new(overstay_rules);
        let result = overstay_rule_list.validate();
        assert!(
            result.is_err(),
            "Validation should fail for exceeding max length"
        );
    }

    #[test]
    fn test_overstay_rule_list_validation_min_length() {
        let overstay_rules = vec![];
        let overstay_rule_list = OverstayRuleListType::new(overstay_rules);
        let result = overstay_rule_list.validate();
        assert!(
            result.is_err(),
            "Validation should fail for not meeting min length"
        );
    }
}
