use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::{day_of_week::DayOfWeekEnumType, evse_kind::EvseKindEnumType};

/// These conditions describe if a FixedPrice applies at start of the transaction.
///
/// When more than one restriction is set, they are to be treated as a logical AND. All need to be valid before this price is active.
///
/// NOTE: _startTimeOfDay_ and _endTimeOfDay_ are in local time, because it is the time in the tariff as it is shown to the EV driver at the Charging Station.
/// A Charging Station will convert this to the internal time zone that it uses (which is recommended to be UTC, see section Generic chapter 3.1) when performing cost calculation.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsFixedType {
    /// Optional. Start time of day in local time.
    /// Format as per RFC 3339: time-hour ":" time-minute
    /// Must be in 24h format with leading zeros. Hour/Minute separator: ":"
    /// Regex: ([0-1][0-9]|2[0-3]):[0-5][0-9]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,

    /// Optional. End time of day in local time. Same syntax as _startTimeOfDay_.
    /// If end time < start time then the period wraps around to the next day.
    /// To stop at end of the day use: 00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_of_day: Option<String>,

    /// Optional. Day(s) of the week this is tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 7))]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,

    /// Optional. Start date in local time, for example: 2015-12-24.
    /// Valid from this day (inclusive).
    /// Format as per RFC 3339: full-date
    /// Regex: ([12][0-9]{3})-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,

    /// Optional. End date in local time, for example: 2015-12-27.
    /// Valid until this day (exclusive). Same syntax as _validFromDate_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,

    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,

    /// Optional. For which payment brand this (adhoc) tariff applies. Can be used to add a surcharge for certain payment brands.
    /// Based on value of _additionalIdToken_ from _idToken.additionalInfo.type_ = "PaymentBrand".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub payment_brand: Option<String>,

    /// Optional. Type of adhoc payment, e.g. CC, Debit.
    /// Based on value of _additionalIdToken_ from _idToken.additionalInfo.type_ = "PaymentRecognition".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub payment_recognition: Option<String>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffConditionsFixedType {
    /// Creates a new `TariffConditionsFixedType` with all fields as `None`
    ///
    /// # Returns
    ///
    /// A new instance of `TariffConditionsFixedType`
    pub fn new() -> Self {
        Self {
            start_time_of_day: None,
            end_time_of_day: None,
            day_of_week: None,
            valid_from_date: None,
            valid_to_date: None,
            evse_kind: None,
            payment_brand: None,
            payment_recognition: None,
            custom_data: None,
        }
    }

    /// Sets the start time of day.
    ///
    /// # Arguments
    ///
    /// * `start_time_of_day` - Start time of day in local time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_start_time_of_day(mut self, start_time_of_day: String) -> Self {
        self.start_time_of_day = Some(start_time_of_day);
        self
    }

    /// Sets the end time of day.
    ///
    /// # Arguments
    ///
    /// * `end_time_of_day` - End time of day in local time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_end_time_of_day(mut self, end_time_of_day: String) -> Self {
        self.end_time_of_day = Some(end_time_of_day);
        self
    }

    /// Sets the day(s) of the week this tariff applies.
    ///
    /// # Arguments
    ///
    /// * `day_of_week` - Day(s) of the week this tariff applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_day_of_week(mut self, day_of_week: Vec<DayOfWeekEnumType>) -> Self {
        self.day_of_week = Some(day_of_week);
        self
    }

    /// Sets the valid from date.
    ///
    /// # Arguments
    ///
    /// * `valid_from_date` - Start date in local time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_from_date(mut self, valid_from_date: String) -> Self {
        self.valid_from_date = Some(valid_from_date);
        self
    }

    /// Sets the valid to date.
    ///
    /// # Arguments
    ///
    /// * `valid_to_date` - End date in local time
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_to_date(mut self, valid_to_date: String) -> Self {
        self.valid_to_date = Some(valid_to_date);
        self
    }

    /// Sets the type of EVSE this tariff applies to.
    ///
    /// # Arguments
    ///
    /// * `evse_kind` - Type of EVSE (AC, DC) this tariff applies to
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse_kind(mut self, evse_kind: EvseKindEnumType) -> Self {
        self.evse_kind = Some(evse_kind.clone());
        self
    }

    /// Sets the payment brand this tariff applies to.
    ///
    /// # Arguments
    ///
    /// * `payment_brand` - For which payment brand this (adhoc) tariff applies
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_payment_brand(mut self, payment_brand: String) -> Self {
        self.payment_brand = Some(payment_brand);
        self
    }

    /// Sets the type of adhoc payment.
    ///
    /// # Arguments
    ///
    /// * `payment_recognition` - Type of adhoc payment, e.g. CC, Debit
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_payment_recognition(mut self, payment_recognition: String) -> Self {
        self.payment_recognition = Some(payment_recognition);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff condition
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the start time of day.
    ///
    /// # Returns
    ///
    /// An optional reference to the start time of day
    pub fn start_time_of_day(&self) -> Option<&String> {
        self.start_time_of_day.as_ref()
    }

    /// Sets the start time of day.
    ///
    /// # Arguments
    ///
    /// * `start_time_of_day` - Start time of day in local time, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_time_of_day(&mut self, start_time_of_day: Option<String>) -> &mut Self {
        self.start_time_of_day = start_time_of_day;
        self
    }

    /// Gets the end time of day.
    ///
    /// # Returns
    ///
    /// An optional reference to the end time of day
    pub fn end_time_of_day(&self) -> Option<&String> {
        self.end_time_of_day.as_ref()
    }

    /// Sets the end time of day.
    ///
    /// # Arguments
    ///
    /// * `end_time_of_day` - End time of day in local time, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_end_time_of_day(&mut self, end_time_of_day: Option<String>) -> &mut Self {
        self.end_time_of_day = end_time_of_day;
        self
    }

    /// Gets the day(s) of the week this tariff applies.
    ///
    /// # Returns
    ///
    /// An optional reference to the day(s) of the week this tariff applies
    pub fn day_of_week(&self) -> Option<&Vec<DayOfWeekEnumType>> {
        self.day_of_week.as_ref()
    }

    /// Sets the day(s) of the week this tariff applies.
    ///
    /// # Arguments
    ///
    /// * `day_of_week` - Day(s) of the week this tariff applies, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_day_of_week(&mut self, day_of_week: Option<Vec<DayOfWeekEnumType>>) -> &mut Self {
        self.day_of_week = day_of_week;
        self
    }

    /// Gets the valid from date.
    ///
    /// # Returns
    ///
    /// An optional reference to the valid from date
    pub fn valid_from_date(&self) -> Option<&String> {
        self.valid_from_date.as_ref()
    }

    /// Sets the valid from date.
    ///
    /// # Arguments
    ///
    /// * `valid_from_date` - Valid from date in local time, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_from_date(&mut self, valid_from_date: Option<String>) -> &mut Self {
        self.valid_from_date = valid_from_date;
        self
    }

    /// Gets the valid to date.
    ///
    /// # Returns
    ///
    /// An optional reference to the valid to date
    pub fn valid_to_date(&self) -> Option<&String> {
        self.valid_to_date.as_ref()
    }

    /// Sets the valid to date.
    ///
    /// # Arguments
    ///
    /// * `valid_to_date` - Valid to date in local time, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_to_date(&mut self, valid_to_date: Option<String>) -> &mut Self {
        self.valid_to_date = valid_to_date;
        self
    }

    /// Gets the type of EVSE this tariff applies to.
    ///
    /// # Returns
    ///
    /// An optional reference to the type of EVSE this tariff applies to
    pub fn evse_kind(&self) -> Option<&EvseKindEnumType> {
        self.evse_kind.as_ref()
    }

    /// Sets the type of EVSE this tariff applies to.
    ///
    /// # Arguments
    ///
    /// * `evse_kind` - Type of EVSE this tariff applies to, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_evse_kind(&mut self, evse_kind: Option<EvseKindEnumType>) -> &mut Self {
        self.evse_kind = evse_kind;
        self
    }

    /// Gets the payment brand this tariff applies to.
    ///
    /// # Returns
    ///
    /// An optional reference to the payment brand
    pub fn payment_brand(&self) -> Option<&String> {
        self.payment_brand.as_ref()
    }

    /// Sets the payment brand this tariff applies to.
    ///
    /// # Arguments
    ///
    /// * `payment_brand` - Payment brand this tariff applies to, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_payment_brand(&mut self, payment_brand: Option<String>) -> &mut Self {
        self.payment_brand = payment_brand;
        self
    }

    /// Gets the type of adhoc payment.
    ///
    /// # Returns
    ///
    /// An optional reference to the type of adhoc payment
    pub fn payment_recognition(&self) -> Option<&String> {
        self.payment_recognition.as_ref()
    }

    /// Sets the type of adhoc payment.
    ///
    /// # Arguments
    ///
    /// * `payment_recognition` - Type of adhoc payment, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_payment_recognition(&mut self, payment_recognition: Option<String>) -> &mut Self {
        self.payment_recognition = payment_recognition;
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
    /// * `custom_data` - Custom data for this tariff condition, or None to clear
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
    fn test_new_tariff_conditions_fixed() {
        let tariff_conditions_fixed = TariffConditionsFixedType::new();

        assert_eq!(tariff_conditions_fixed.start_time_of_day(), None);
        assert_eq!(tariff_conditions_fixed.end_time_of_day(), None);
        assert_eq!(tariff_conditions_fixed.day_of_week(), None);
        assert_eq!(tariff_conditions_fixed.valid_from_date(), None);
        assert_eq!(tariff_conditions_fixed.valid_to_date(), None);
        assert_eq!(tariff_conditions_fixed.evse_kind(), None);
        assert_eq!(tariff_conditions_fixed.payment_brand(), None);
        assert_eq!(tariff_conditions_fixed.payment_recognition(), None);
        assert_eq!(tariff_conditions_fixed.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let start_time_of_day = "08:00".to_string();
        let end_time_of_day = "20:00".to_string();
        let day_of_week = vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday].clone();
        let valid_from_date = "2023-01-01".to_string();
        let valid_to_date = "2023-12-31".to_string();
        let evse_kind = EvseKindEnumType::AC.clone();
        let payment_brand = "VISA".to_string();
        let payment_recognition = "CC".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_conditions_fixed = TariffConditionsFixedType::new()
            .with_start_time_of_day(start_time_of_day.clone())
            .with_end_time_of_day(end_time_of_day.clone())
            .with_day_of_week(day_of_week.clone())
            .with_valid_from_date(valid_from_date.clone())
            .with_valid_to_date(valid_to_date.clone())
            .with_evse_kind(evse_kind.clone())
            .with_payment_brand(payment_brand.clone())
            .with_payment_recognition(payment_recognition.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(
            tariff_conditions_fixed.start_time_of_day(),
            Some(&start_time_of_day)
        );
        assert_eq!(
            tariff_conditions_fixed.end_time_of_day(),
            Some(&end_time_of_day)
        );
        assert_eq!(tariff_conditions_fixed.day_of_week(), Some(&day_of_week));
        assert_eq!(
            tariff_conditions_fixed.valid_from_date(),
            Some(&valid_from_date)
        );
        assert_eq!(
            tariff_conditions_fixed.valid_to_date(),
            Some(&valid_to_date)
        );
        assert_eq!(tariff_conditions_fixed.evse_kind(), Some(&evse_kind));
        assert_eq!(
            tariff_conditions_fixed.payment_brand(),
            Some(&payment_brand)
        );
        assert_eq!(
            tariff_conditions_fixed.payment_recognition(),
            Some(&payment_recognition)
        );
        assert_eq!(tariff_conditions_fixed.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let mut tariff_conditions_fixed = TariffConditionsFixedType::new();

        let start_time_of_day = "08:00".to_string();
        let end_time_of_day = "20:00".to_string();
        let day_of_week = vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday].clone();
        let valid_from_date = "2023-01-01".to_string();
        let valid_to_date = "2023-12-31".to_string();
        let evse_kind = EvseKindEnumType::AC.clone();
        let payment_brand = "VISA".to_string();
        let payment_recognition = "CC".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        tariff_conditions_fixed
            .set_start_time_of_day(Some(start_time_of_day.clone()))
            .set_end_time_of_day(Some(end_time_of_day.clone()))
            .set_day_of_week(Some(day_of_week.clone()))
            .set_valid_from_date(Some(valid_from_date.clone()))
            .set_valid_to_date(Some(valid_to_date.clone()))
            .set_evse_kind(Some(evse_kind.clone()))
            .set_payment_brand(Some(payment_brand.clone()))
            .set_payment_recognition(Some(payment_recognition.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            tariff_conditions_fixed.start_time_of_day(),
            Some(&start_time_of_day)
        );
        assert_eq!(
            tariff_conditions_fixed.end_time_of_day(),
            Some(&end_time_of_day)
        );
        assert_eq!(tariff_conditions_fixed.day_of_week(), Some(&day_of_week));
        assert_eq!(
            tariff_conditions_fixed.valid_from_date(),
            Some(&valid_from_date)
        );
        assert_eq!(
            tariff_conditions_fixed.valid_to_date(),
            Some(&valid_to_date)
        );
        assert_eq!(tariff_conditions_fixed.evse_kind(), Some(&evse_kind));
        assert_eq!(
            tariff_conditions_fixed.payment_brand(),
            Some(&payment_brand)
        );
        assert_eq!(
            tariff_conditions_fixed.payment_recognition(),
            Some(&payment_recognition)
        );
        assert_eq!(tariff_conditions_fixed.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_conditions_fixed
            .set_start_time_of_day(None)
            .set_end_time_of_day(None)
            .set_day_of_week(None)
            .set_valid_from_date(None)
            .set_valid_to_date(None)
            .set_evse_kind(None)
            .set_payment_brand(None)
            .set_payment_recognition(None)
            .set_custom_data(None);

        assert_eq!(tariff_conditions_fixed.start_time_of_day(), None);
        assert_eq!(tariff_conditions_fixed.end_time_of_day(), None);
        assert_eq!(tariff_conditions_fixed.day_of_week(), None);
        assert_eq!(tariff_conditions_fixed.valid_from_date(), None);
        assert_eq!(tariff_conditions_fixed.valid_to_date(), None);
        assert_eq!(tariff_conditions_fixed.evse_kind(), None);
        assert_eq!(tariff_conditions_fixed.payment_brand(), None);
        assert_eq!(tariff_conditions_fixed.payment_recognition(), None);
        assert_eq!(tariff_conditions_fixed.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Test with valid day_of_week
        let valid_conditions = TariffConditionsFixedType::new()
            .with_day_of_week(vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday]);
        assert!(valid_conditions.validate().is_ok());

        // Test with invalid day_of_week (empty vector)
        let day_of_week: Vec<DayOfWeekEnumType> = vec![];
        let mut invalid_conditions = TariffConditionsFixedType::new();
        invalid_conditions.day_of_week = Some(day_of_week);
        assert!(invalid_conditions.validate().is_err());

        // Test with too many days of week
        let day_of_week = vec![
            DayOfWeekEnumType::Monday,
            DayOfWeekEnumType::Tuesday,
            DayOfWeekEnumType::Wednesday,
            DayOfWeekEnumType::Thursday,
            DayOfWeekEnumType::Friday,
            DayOfWeekEnumType::Saturday,
            DayOfWeekEnumType::Sunday,
            DayOfWeekEnumType::Monday, // Duplicate to exceed the limit
        ]
        .clone();
        let mut invalid_conditions = TariffConditionsFixedType::new();
        invalid_conditions.day_of_week = Some(day_of_week);
        assert!(invalid_conditions.validate().is_err());

        // Test with valid payment_brand
        let valid_conditions =
            TariffConditionsFixedType::new().with_payment_brand("VISA".to_string());
        assert!(valid_conditions.validate().is_ok());

        // Test with too long payment_brand
        let payment_brand = "X".repeat(21); // 21 characters
        let mut invalid_conditions = TariffConditionsFixedType::new();
        invalid_conditions.payment_brand = Some(payment_brand);
        assert!(invalid_conditions.validate().is_err());

        // Test with valid payment_recognition
        let valid_conditions =
            TariffConditionsFixedType::new().with_payment_recognition("CC".to_string());
        assert!(valid_conditions.validate().is_ok());

        // Test with too long payment_recognition
        let payment_recognition = "X".repeat(21); // 21 characters
        let mut invalid_conditions = TariffConditionsFixedType::new();
        invalid_conditions.payment_recognition = Some(payment_recognition);
        assert!(invalid_conditions.validate().is_err());
    }
}
