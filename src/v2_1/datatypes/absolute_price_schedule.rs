use crate::v2_1::datatypes::{
    additional_selected_services::AdditionalSelectedServicesType, custom_data::CustomDataType,
    overstay_rule_list::OverstayRuleListType, price_rule_stack::PriceRuleStackType,
    rational_number::RationalNumberType, tax_rule::TaxRuleType,
};
use crate::v2_1::helpers::datetime_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// The AbsolutePriceScheduleType is modeled after the same type that is defined in ISO 15118-20,
/// such that if it is supplied by an EMSP as a signed EXI message, the conversion from EXI to JSON
/// (in OCPP) and back to EXI (for ISO 15118-20) does not change the digest and therefore does not
/// invalidate the signature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AbsolutePriceScheduleType {
    /// Starting point of price schedule.
    #[serde(with = "datetime_rfc3339 ")]
    pub time_anchor: DateTime<Utc>, // date-time format

    /// Unique ID of price schedule
    #[serde(rename = "priceScheduleID")]
    #[validate(range(min = 0))]
    pub price_schedule_id: i32,

    /// Description of the price schedule.
    #[serde(rename = "priceScheduleDescription")]
    #[validate(length(max = 160))]
    pub price_schedule_description: Option<String>,

    /// Currency according to ISO 4217.
    #[validate(length(max = 3))]
    pub currency: String,

    /// String that indicates what language is used for the human readable strings in the price schedule.
    /// Based on ISO 639.
    #[validate(length(max = 8))]
    pub language: String,

    /// A string in URN notation which shall uniquely identify an algorithm that defines how to compute
    /// an energy fee sum for a specific power profile based on the EnergyFee information from the PriceRule elements.
    #[validate(length(max = 2000))]
    pub price_algorithm: String,

    /// Stack of price rules, defining the price of charging.
    #[validate(length(min = 1, max = 1024), nested)]
    pub price_rule_stacks: Vec<PriceRuleStackType>,

    /// List of tax rules that apply to the price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 10), nested)]
    pub tax_rules: Option<Vec<TaxRuleType>>,

    /// List of additional services selected by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 5), nested)]
    pub additional_selected_services: Option<Vec<AdditionalSelectedServicesType>>,

    /// Rules for overstay pricing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub overstay_rule_list: Option<OverstayRuleListType>,

    /// Minimum cost of a charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub minimum_cost: Option<RationalNumberType>,

    /// Maximum cost of a charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub maximum_cost: Option<RationalNumberType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AbsolutePriceScheduleType {
    /// Creates a new `AbsolutePriceScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of price schedule as DateTime<Utc>
    /// * `price_schedule_id` - Unique ID of price schedule
    /// * `currency` - Currency according to ISO 4217
    /// * `language` - Language used for human readable strings based on ISO 639
    /// * `price_algorithm` - Algorithm that defines how to compute energy fee
    /// * `price_rule_stacks` - Stack of price rules defining the price of charging
    ///
    /// # Returns
    ///
    /// A new instance of `AbsolutePriceScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        price_schedule_id: i32,
        currency: String,
        language: String,
        price_algorithm: String,
        price_rule_stacks: Vec<PriceRuleStackType>,
    ) -> Self {
        Self {
            time_anchor,
            price_schedule_id,
            price_schedule_description: None,
            currency,
            language,
            price_algorithm,
            minimum_cost: None,
            maximum_cost: None,
            price_rule_stacks,
            tax_rules: None,
            overstay_rule_list: None,
            additional_selected_services: None,
            custom_data: None,
        }
    }

    /// Creates a new `AbsolutePriceScheduleType` from a string time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor_str` - Starting point of price schedule in RFC3339 date-time format
    /// * `price_schedule_id` - Unique ID of price schedule
    /// * `currency` - Currency according to ISO 4217
    /// * `language` - Language used for human readable strings based on ISO 639
    /// * `price_algorithm` - Algorithm that defines how to compute energy fee
    /// * `price_rule_stacks` - Stack of price rules defining the price of charging
    ///
    /// # Returns
    ///
    /// A new instance of `AbsolutePriceScheduleType` with optional fields set to `None`
    pub fn new_from_str(
        time_anchor_str: &str,
        price_schedule_id: i32,
        currency: String,
        language: String,
        price_algorithm: String,
        price_rule_stacks: Vec<PriceRuleStackType>,
    ) -> Self {
        // Parse the time_anchor string into DateTime<Utc>
        let time_anchor = DateTime::parse_from_rfc3339(time_anchor_str)
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        Self::new(
            time_anchor,
            price_schedule_id,
            currency,
            language,
            price_algorithm,
            price_rule_stacks,
        )
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_schedule_description(mut self, description: String) -> Self {
        self.price_schedule_description = Some(description);
        self
    }

    /// Sets the price algorithm.
    ///
    /// # Arguments
    ///
    /// * `price_algorithm` - Algorithm that defines how to compute energy fee
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_algorithm(mut self, price_algorithm: String) -> Self {
        self.price_algorithm = price_algorithm;
        self
    }

    /// Sets the minimum cost.
    ///
    /// # Arguments
    ///
    /// * `minimum_cost` - Minimum cost of a charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_minimum_cost(mut self, minimum_cost: RationalNumberType) -> Self {
        self.minimum_cost = Some(minimum_cost);
        self
    }

    /// Sets the maximum cost.
    ///
    /// # Arguments
    ///
    /// * `maximum_cost` - Maximum cost of a charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_maximum_cost(mut self, maximum_cost: RationalNumberType) -> Self {
        self.maximum_cost = Some(maximum_cost);
        self
    }

    /// Sets the tax rules.
    ///
    /// # Arguments
    ///
    /// * `tax_rules` - List of tax rules that apply to the price
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tax_rules(mut self, tax_rules: Vec<TaxRuleType>) -> Self {
        self.tax_rules = Some(tax_rules);
        self
    }

    /// Sets the overstay rule list.
    ///
    /// # Arguments
    ///
    /// * `overstay_rule_list` - Rules for overstay pricing
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_overstay_rule_list(mut self, overstay_rule_list: OverstayRuleListType) -> Self {
        self.overstay_rule_list = Some(overstay_rule_list);
        self
    }

    /// Sets the additional selected services.
    ///
    /// # Arguments
    ///
    /// * `additional_selected_services` - List of additional services selected by the user
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_additional_selected_services(
        mut self,
        additional_selected_services: Vec<AdditionalSelectedServicesType>,
    ) -> Self {
        self.additional_selected_services = Some(additional_selected_services);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the time anchor.
    ///
    /// # Returns
    ///
    /// A reference to the time anchor as DateTime<Utc>
    pub fn time_anchor(&self) -> &DateTime<Utc> {
        &self.time_anchor
    }

    /// Gets the time anchor as a formatted string.
    ///
    /// # Returns
    ///
    /// The time anchor formatted as an RFC3339 string with 'Z' timezone format
    pub fn time_anchor_str(&self) -> String {
        // Format with 'Z' instead of '+00:00' for UTC timezone
        self.time_anchor.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    /// Sets the time anchor from a string in RFC3339 format.
    ///
    /// # Arguments
    ///
    /// * `time_anchor_str` - Starting point of price schedule in RFC3339 date-time format as a string
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor_str(&mut self, time_anchor_str: &str) -> &mut Self {
        self.time_anchor = DateTime::parse_from_rfc3339(time_anchor_str)
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);
        self
    }

    /// Sets the time anchor directly with a DateTime<Utc> value.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of price schedule as DateTime<Utc>
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: DateTime<Utc>) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the price schedule ID.
    ///
    /// # Returns
    ///
    /// The unique ID of the price schedule
    pub fn price_schedule_id(&self) -> i32 {
        self.price_schedule_id
    }

    /// Sets the price schedule ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique ID of price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_id(&mut self, id: i32) -> &mut Self {
        self.price_schedule_id = id;
        self
    }

    /// Gets the price schedule description.
    ///
    /// # Returns
    ///
    /// An optional reference to the price schedule description
    pub fn price_schedule_description(&self) -> Option<&String> {
        self.price_schedule_description.as_ref()
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_description(&mut self, description: Option<String>) -> &mut Self {
        self.price_schedule_description = description;
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// A reference to the currency code
    pub fn currency(&self) -> &String {
        &self.currency
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency according to ISO 4217
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: String) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the language.
    ///
    /// # Returns
    ///
    /// A reference to the language code
    pub fn language(&self) -> &String {
        &self.language
    }

    /// Sets the language.
    ///
    /// # Arguments
    ///
    /// * `language` - Language code based on ISO 639
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language(&mut self, language: String) -> &mut Self {
        self.language = language;
        self
    }

    /// Gets the price algorithm.
    ///
    /// # Returns
    ///
    /// A reference to the price algorithm
    pub fn price_algorithm(&self) -> &String {
        &self.price_algorithm
    }

    /// Sets the price algorithm.
    ///
    /// # Arguments
    ///
    /// * `price_algorithm` - Algorithm that defines how to compute energy fee
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_algorithm(&mut self, price_algorithm: String) -> &mut Self {
        self.price_algorithm = price_algorithm;
        self
    }

    /// Gets the price rule stacks.
    ///
    /// # Returns
    ///
    /// A reference to the price rule stacks
    pub fn price_rule_stacks(&self) -> &Vec<PriceRuleStackType> {
        &self.price_rule_stacks
    }

    /// Sets the price rule stacks.
    ///
    /// # Arguments
    ///
    /// * `price_rule_stacks` - Stack of price rules defining the price of charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_rule_stacks(
        &mut self,
        price_rule_stacks: Vec<PriceRuleStackType>,
    ) -> &mut Self {
        self.price_rule_stacks = price_rule_stacks;
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
    /// * `custom_data` - Custom data for this price schedule, or None to clear
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
    use crate::v2_1::datatypes::price_rule::PriceRuleType;

    #[test]
    fn test_new_absolute_price_schedule() {
        // Create a DateTime<Utc> for testing
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        // Create a simple price rule for testing
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        // Create a simple price rule stack for testing
        let price_rule_stack = PriceRuleStackType::new(3600, vec![price_rule]);

        let schedule = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack],
        );

        // Use time_anchor_str() to get the string representation for comparison
        assert_eq!(schedule.time_anchor_str(), "2023-01-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 123);
        assert_eq!(schedule.price_schedule_description(), None);
        assert_eq!(schedule.currency(), &"USD".to_string());
        assert_eq!(schedule.language(), &"en".to_string());
        assert_eq!(
            schedule.price_algorithm(),
            &"urn:algorithm:energy-fee:1.0".to_string()
        );
    }

    #[test]
    fn test_with_methods() {
        // Create a DateTime<Utc> for testing
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        // Create a simple price rule for testing
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        // Create a simple price rule stack for testing
        let price_rule_stack = PriceRuleStackType::new(3600, vec![price_rule]);

        let schedule = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack],
        )
        .with_price_schedule_description("Test Schedule".to_string());

        // Use time_anchor_str() to get the string representation for comparison
        assert_eq!(schedule.time_anchor_str(), "2023-01-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 123);
        assert_eq!(
            schedule.price_schedule_description(),
            Some(&"Test Schedule".to_string())
        );
        assert_eq!(schedule.currency(), &"USD".to_string());
        assert_eq!(schedule.language(), &"en".to_string());
    }

    #[test]
    fn test_setter_methods() {
        // Create a DateTime<Utc> for testing
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        // Create a simple price rule for testing
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        // Create a simple price rule stack for testing
        let price_rule_stack = PriceRuleStackType::new(3600, vec![price_rule]);

        let mut schedule = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack],
        );

        // We don't need to create another DateTime since we're using the string version in set_time_anchor_str

        schedule
            .set_time_anchor_str("2023-02-01T00:00:00Z")
            .set_price_schedule_id(456)
            .set_price_schedule_description(Some("Updated Schedule".to_string()))
            .set_currency("EUR".to_string())
            .set_language("fr".to_string())
            .set_price_algorithm("urn:algorithm:energy-fee:2.0".to_string());

        // Use time_anchor_str() to get the string representation for comparison
        assert_eq!(schedule.time_anchor_str(), "2023-02-01T00:00:00Z");
        assert_eq!(schedule.price_schedule_id(), 456);
        assert_eq!(
            schedule.price_schedule_description(),
            Some(&"Updated Schedule".to_string())
        );
        assert_eq!(schedule.currency(), &"EUR".to_string());
        assert_eq!(schedule.language(), &"fr".to_string());
        assert_eq!(
            schedule.price_algorithm(),
            &"urn:algorithm:energy-fee:2.0".to_string()
        );

        // Test clearing optional fields
        schedule.set_price_schedule_description(None);

        assert_eq!(schedule.price_schedule_description(), None);
    }

    #[test]
    fn test_with_custom_data() {
        // Create a DateTime<Utc> for testing
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        // Create a simple price rule for testing
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        // Create a simple price rule stack for testing
        let price_rule_stack1 = PriceRuleStackType::new(3600, vec![price_rule.clone()]);
        let price_rule_stack2 = PriceRuleStackType::new(3600, vec![price_rule]);

        // Create custom data
        let custom_data = CustomDataType::new("VendorX".to_string());

        // Test with_custom_data method
        let schedule = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack1],
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test setter method
        let mut schedule2 = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack2],
        );

        schedule2.set_custom_data(Some(custom_data.clone()));
        assert_eq!(schedule2.custom_data(), Some(&custom_data));

        // Test clearing custom data
        schedule2.set_custom_data(None);
        assert_eq!(schedule2.custom_data(), None);
    }

    #[test]
    fn test_validate_absolute_price_schedule() {
        use validator::Validate;

        // Create a DateTime<Utc> for testing
        let time = DateTime::parse_from_rfc3339("2023-01-01T00:00:00Z")
            .expect("Invalid RFC3339 datetime format")
            .with_timezone(&Utc);

        // Create a simple price rule for testing
        let energy_fee = RationalNumberType::new(2, 25); // Represents 0.25 with exponent 2
        let power_range_start = RationalNumberType::new(0, 0);
        let price_rule = PriceRuleType::new(energy_fee, power_range_start);

        // Create a simple price rule stack for testing
        let price_rule_stack = PriceRuleStackType::new(3600, vec![price_rule.clone()]);

        // 1. Test valid instance - should pass validation
        let valid_schedule = AbsolutePriceScheduleType::new(
            time,
            123,
            "USD".to_string(),
            "en".to_string(),
            "urn:algorithm:energy-fee:1.0".to_string(),
            vec![price_rule_stack.clone()],
        );

        assert!(
            valid_schedule.validate().is_ok(),
            "Valid schedule should pass validation"
        );

        // 2. Test invalid price_schedule_id (negative value)
        let mut invalid_id_schedule = valid_schedule.clone();
        invalid_id_schedule.set_price_schedule_id(-1);

        let validation_result = invalid_id_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with negative ID should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("price_schedule_id"),
            "Error should mention price_schedule_id: {}",
            error
        );

        // 3. Test invalid price_schedule_description (too long)
        let mut invalid_desc_schedule = valid_schedule.clone();
        let long_description = "a".repeat(161); // 161 characters, exceeds max of 160
        invalid_desc_schedule.set_price_schedule_description(Some(long_description));

        let validation_result = invalid_desc_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with too long description should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("price_schedule_description"),
            "Error should mention price_schedule_description: {}",
            error
        );

        // 4. Test invalid currency (too long)
        let mut invalid_currency_schedule = valid_schedule.clone();
        invalid_currency_schedule.set_currency("USDT".to_string()); // 4 characters, exceeds max of 3

        let validation_result = invalid_currency_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with too long currency should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("currency"),
            "Error should mention currency: {}",
            error
        );

        // 5. Test invalid language (too long)
        let mut invalid_language_schedule = valid_schedule.clone();
        invalid_language_schedule.set_language("en-US-ext".to_string()); // 9 characters, exceeds max of 8

        let validation_result = invalid_language_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with too long language should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("language"),
            "Error should mention language: {}",
            error
        );

        // 6. Test invalid price_algorithm (too long)
        let mut invalid_algorithm_schedule = valid_schedule.clone();
        let long_algorithm = "urn:".to_string() + &"a".repeat(2000); // Exceeds max of 2000
        invalid_algorithm_schedule.set_price_algorithm(long_algorithm);

        let validation_result = invalid_algorithm_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with too long price_algorithm should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("price_algorithm"),
            "Error should mention price_algorithm: {}",
            error
        );

        // 7. Test invalid price_rule_stacks (empty)
        let mut invalid_stacks_schedule = valid_schedule.clone();
        invalid_stacks_schedule.set_price_rule_stacks(vec![]);

        let validation_result = invalid_stacks_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with empty price_rule_stacks should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("price_rule_stacks"),
            "Error should mention price_rule_stacks: {}",
            error
        );

        // 8. Test invalid price_rule_stacks (too many)
        let mut invalid_many_stacks_schedule = valid_schedule.clone();
        let many_stacks = vec![price_rule_stack; 1025]; // 1025 elements, exceeds max of 1024
        invalid_many_stacks_schedule.set_price_rule_stacks(many_stacks);

        let validation_result = invalid_many_stacks_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with too many price_rule_stacks should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("price_rule_stacks"),
            "Error should mention price_rule_stacks: {}",
            error
        );

        // 9. Test invalid tax_rules (empty when set)
        let invalid_tax_rules_schedule = valid_schedule.clone().with_tax_rules(vec![]);

        let validation_result = invalid_tax_rules_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with empty tax_rules should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("tax_rules"),
            "Error should mention tax_rules: {}",
            error
        );

        // 10. Test invalid additional_selected_services (empty when set)
        let invalid_services_schedule = valid_schedule
            .clone()
            .with_additional_selected_services(vec![]);

        let validation_result = invalid_services_schedule.validate();
        assert!(
            validation_result.is_err(),
            "Schedule with empty additional_selected_services should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("additional_selected_services"),
            "Error should mention additional_selected_services: {}",
            error
        );
    }
}
