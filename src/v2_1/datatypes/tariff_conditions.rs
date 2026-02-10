use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::{day_of_week::DayOfWeekEnumType, evse_kind::EvseKindEnumType};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// These conditions describe if and when a TariffEnergyType or TariffTimeType applies during a transaction.
///
/// When more than one restriction is set, they are to be treated as a logical AND. All need to be valid before this price is active.
///
/// For reverse energy flow (discharging) negative values of energy, power and current are used.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsType {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,

    /// Optional. End date in local time, for example: 2015-12-27.
    /// Valid until this day (exclusive). Same syntax as _validFromDate_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,

    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,

    /// Optional. Minimum consumed energy in Wh, for example 20000 Wh.
    /// Valid from this amount of energy (inclusive) being used.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_energy: Option<Decimal>,

    /// Optional. Maximum consumed energy in Wh, for example 50000 Wh.
    /// Valid until this amount of energy (exclusive) being used.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_energy: Option<Decimal>,

    /// Optional. Sum of the minimum current (in Amperes) over all phases, for example 5 A.
    /// When the EV is charging with more than, or equal to, the defined amount of current, this price is/becomes active.
    /// If the charging current is or becomes lower, this price is not or no longer valid and becomes inactive.
    /// This is NOT about the minimum current over the entire transaction.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_current: Option<Decimal>,

    /// Optional. Sum of the maximum current (in Amperes) over all phases, for example 20 A.
    /// When the EV is charging with less than the defined amount of current, this price becomes/is active.
    /// If the charging current is or becomes higher, this price is not or no longer valid and becomes inactive.
    /// This is NOT about the maximum current over the entire transaction.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_current: Option<Decimal>,

    /// Optional. Minimum power in W, for example 5000 W.
    /// When the EV is charging with more than, or equal to, the defined amount of power, this price is/becomes active.
    /// If the charging power is or becomes lower, this price is not or no longer valid and becomes inactive.
    /// This is NOT about the minimum power over the entire transaction.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_power: Option<Decimal>,

    /// Optional. Maximum power in W, for example 20000 W.
    /// When the EV is charging with less than the defined amount of power, this price becomes/is active.
    /// If the charging power is or becomes higher, this price is not or no longer valid and becomes inactive.
    /// This is NOT about the maximum power over the entire transaction.
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub max_power: Option<Decimal>,

    /// Optional. Minimum duration in seconds the transaction (charging & idle) MUST last (inclusive).
    /// When the duration of a transaction is longer than the defined value, this price is or becomes active.
    /// Before that moment, this price is not yet active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_time: Option<i32>,

    /// Optional. Maximum duration in seconds the transaction (charging & idle) MUST last (exclusive).
    /// When the duration of a transaction is shorter than the defined value, this price is or becomes active.
    /// After that moment, this price is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i32>,

    /// Optional. Minimum duration in seconds the charging MUST last (inclusive).
    /// When the duration of a charging is longer than the defined value, this price is or becomes active.
    /// Before that moment, this price is not yet active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_time: Option<i32>,

    /// Optional. Maximum duration in seconds the charging MUST last (exclusive).
    /// When the duration of a charging is shorter than the defined value, this price is or becomes active.
    /// After that moment, this price is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charging_time: Option<i32>,

    /// Optional. Minimum duration in seconds the idle period (i.e. not charging) MUST last (inclusive).
    /// When the duration of the idle time is longer than the defined value, this price is or becomes active.
    /// Before that moment, this price is not yet active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_time: Option<i32>,

    /// Optional. Maximum duration in seconds the idle period (i.e. not charging) MUST last (exclusive).
    /// When the duration of idle time is shorter than the defined value, this price is or becomes active.
    /// After that moment, this price is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_time: Option<i32>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffConditionsType {
    /// Creates a new empty `TariffConditionsType` with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// A new instance of `TariffConditionsType` with all optional fields set to `None`
    pub fn new() -> Self {
        Self {
            start_time_of_day: None,
            end_time_of_day: None,
            day_of_week: None,
            valid_from_date: None,
            valid_to_date: None,
            evse_kind: None,
            min_energy: None,
            max_energy: None,
            min_current: None,
            max_current: None,
            min_power: None,
            max_power: None,
            min_time: None,
            max_time: None,
            min_charging_time: None,
            max_charging_time: None,
            min_idle_time: None,
            max_idle_time: None,
            custom_data: None,
        }
    }

    /// Sets the start time of day.
    ///
    /// # Arguments
    ///
    /// * `start_time_of_day` - Start time of day in local time, format: HH:MM (24h)
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
    /// * `end_time_of_day` - End time of day in local time, format: HH:MM (24h)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_end_time_of_day(mut self, end_time_of_day: String) -> Self {
        self.end_time_of_day = Some(end_time_of_day);
        self
    }

    /// Sets the days of the week.
    ///
    /// # Arguments
    ///
    /// * `day_of_week` - Days of the week this tariff applies to
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
    /// * `valid_from_date` - Start date in local time, format: YYYY-MM-DD
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
    /// * `valid_to_date` - End date in local time, format: YYYY-MM-DD
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_to_date(mut self, valid_to_date: String) -> Self {
        self.valid_to_date = Some(valid_to_date);
        self
    }

    /// Sets the EVSE kind.
    ///
    /// # Arguments
    ///
    /// * `evse_kind` - Type of EVSE (AC, DC) this tariff applies to
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_evse_kind(mut self, evse_kind: EvseKindEnumType) -> Self {
        self.evse_kind = Some(evse_kind);
        self
    }

    /// Sets the minimum energy.
    ///
    /// # Arguments
    ///
    /// * `min_energy` - Minimum consumed energy in Wh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_energy(mut self, min_energy: Decimal) -> Self {
        self.min_energy = Some(min_energy);
        self
    }

    /// Sets the maximum energy.
    ///
    /// # Arguments
    ///
    /// * `max_energy` - Maximum consumed energy in Wh
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_energy(mut self, max_energy: Decimal) -> Self {
        self.max_energy = Some(max_energy);
        self
    }

    /// Sets the minimum current.
    ///
    /// # Arguments
    ///
    /// * `min_current` - Sum of the minimum current (in Amperes) over all phases
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_current(mut self, min_current: Decimal) -> Self {
        self.min_current = Some(min_current);
        self
    }

    /// Sets the maximum current.
    ///
    /// # Arguments
    ///
    /// * `max_current` - Sum of the maximum current (in Amperes) over all phases
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_current(mut self, max_current: Decimal) -> Self {
        self.max_current = Some(max_current);
        self
    }

    /// Sets the minimum power.
    ///
    /// # Arguments
    ///
    /// * `min_power` - Minimum power in W
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_power(mut self, min_power: Decimal) -> Self {
        self.min_power = Some(min_power);
        self
    }

    /// Sets the maximum power.
    ///
    /// # Arguments
    ///
    /// * `max_power` - Maximum power in W
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_power(mut self, max_power: Decimal) -> Self {
        self.max_power = Some(max_power);
        self
    }

    /// Sets the minimum time.
    ///
    /// # Arguments
    ///
    /// * `min_time` - Minimum duration in seconds the transaction must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_time(mut self, min_time: i32) -> Self {
        self.min_time = Some(min_time);
        self
    }

    /// Sets the maximum time.
    ///
    /// # Arguments
    ///
    /// * `max_time` - Maximum duration in seconds the transaction must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_time(mut self, max_time: i32) -> Self {
        self.max_time = Some(max_time);
        self
    }

    /// Sets the minimum charging time.
    ///
    /// # Arguments
    ///
    /// * `min_charging_time` - Minimum duration in seconds the charging must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_charging_time(mut self, min_charging_time: i32) -> Self {
        self.min_charging_time = Some(min_charging_time);
        self
    }

    /// Sets the maximum charging time.
    ///
    /// # Arguments
    ///
    /// * `max_charging_time` - Maximum duration in seconds the charging must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_charging_time(mut self, max_charging_time: i32) -> Self {
        self.max_charging_time = Some(max_charging_time);
        self
    }

    /// Sets the minimum idle time.
    ///
    /// # Arguments
    ///
    /// * `min_idle_time` - Minimum duration in seconds the idle period must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_idle_time(mut self, min_idle_time: i32) -> Self {
        self.min_idle_time = Some(min_idle_time);
        self
    }

    /// Sets the maximum idle time.
    ///
    /// # Arguments
    ///
    /// * `max_idle_time` - Maximum duration in seconds the idle period must last
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_idle_time(mut self, max_idle_time: i32) -> Self {
        self.max_idle_time = Some(max_idle_time);
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
    /// An optional reference to the start time of day in local time
    pub fn start_time_of_day(&self) -> Option<&str> {
        self.start_time_of_day.as_deref()
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
    /// An optional reference to the end time of day in local time
    pub fn end_time_of_day(&self) -> Option<&str> {
        self.end_time_of_day.as_deref()
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

    /// Gets the days of the week.
    ///
    /// # Returns
    ///
    /// An optional reference to the days of the week this tariff applies to
    pub fn day_of_week(&self) -> Option<&Vec<DayOfWeekEnumType>> {
        self.day_of_week.as_ref()
    }

    /// Sets the days of the week.
    ///
    /// # Arguments
    ///
    /// * `day_of_week` - Days of the week this tariff applies to, or None to clear
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
    /// An optional reference to the start date in local time
    pub fn valid_from_date(&self) -> Option<&str> {
        self.valid_from_date.as_deref()
    }

    /// Sets the valid from date.
    ///
    /// # Arguments
    ///
    /// * `valid_from_date` - Start date in local time, or None to clear
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
    /// An optional reference to the end date in local time
    pub fn valid_to_date(&self) -> Option<&str> {
        self.valid_to_date.as_deref()
    }

    /// Sets the valid to date.
    ///
    /// # Arguments
    ///
    /// * `valid_to_date` - End date in local time, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_to_date(&mut self, valid_to_date: Option<String>) -> &mut Self {
        self.valid_to_date = valid_to_date;
        self
    }

    /// Gets the EVSE kind.
    ///
    /// # Returns
    ///
    /// An optional reference to the type of EVSE this tariff applies to
    pub fn evse_kind(&self) -> Option<&EvseKindEnumType> {
        self.evse_kind.as_ref()
    }

    /// Sets the EVSE kind.
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

    /// Gets the minimum energy.
    ///
    /// # Returns
    ///
    /// An optional minimum consumed energy in Wh
    pub fn min_energy(&self) -> Option<Decimal> {
        self.min_energy
    }

    /// Sets the minimum energy.
    ///
    /// # Arguments
    ///
    /// * `min_energy` - Minimum consumed energy in Wh, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_energy(&mut self, min_energy: Option<Decimal>) -> &mut Self {
        self.min_energy = min_energy;
        self
    }

    /// Gets the maximum energy.
    ///
    /// # Returns
    ///
    /// An optional maximum consumed energy in Wh
    pub fn max_energy(&self) -> Option<Decimal> {
        self.max_energy
    }

    /// Sets the maximum energy.
    ///
    /// # Arguments
    ///
    /// * `max_energy` - Maximum consumed energy in Wh, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_energy(&mut self, max_energy: Option<Decimal>) -> &mut Self {
        self.max_energy = max_energy;
        self
    }

    /// Gets the minimum current.
    ///
    /// # Returns
    ///
    /// An optional sum of the minimum current (in Amperes) over all phases
    pub fn min_current(&self) -> Option<Decimal> {
        self.min_current
    }

    /// Sets the minimum current.
    ///
    /// # Arguments
    ///
    /// * `min_current` - Sum of the minimum current (in Amperes) over all phases, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_current(&mut self, min_current: Option<Decimal>) -> &mut Self {
        self.min_current = min_current;
        self
    }

    /// Gets the maximum current.
    ///
    /// # Returns
    ///
    /// An optional sum of the maximum current (in Amperes) over all phases
    pub fn max_current(&self) -> Option<Decimal> {
        self.max_current
    }

    /// Sets the maximum current.
    ///
    /// # Arguments
    ///
    /// * `max_current` - Sum of the maximum current (in Amperes) over all phases, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_current(&mut self, max_current: Option<Decimal>) -> &mut Self {
        self.max_current = max_current;
        self
    }

    /// Gets the minimum power.
    ///
    /// # Returns
    ///
    /// An optional minimum power in W
    pub fn min_power(&self) -> Option<Decimal> {
        self.min_power
    }

    /// Sets the minimum power.
    ///
    /// # Arguments
    ///
    /// * `min_power` - Minimum power in W, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_power(&mut self, min_power: Option<Decimal>) -> &mut Self {
        self.min_power = min_power;
        self
    }

    /// Gets the maximum power.
    ///
    /// # Returns
    ///
    /// An optional maximum power in W
    pub fn max_power(&self) -> Option<Decimal> {
        self.max_power
    }

    /// Sets the maximum power.
    ///
    /// # Arguments
    ///
    /// * `max_power` - Maximum power in W, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_power(&mut self, max_power: Option<Decimal>) -> &mut Self {
        self.max_power = max_power;
        self
    }

    /// Gets the minimum time.
    ///
    /// # Returns
    ///
    /// An optional minimum duration in seconds the transaction must last
    pub fn min_time(&self) -> Option<i32> {
        self.min_time
    }

    /// Sets the minimum time.
    ///
    /// # Arguments
    ///
    /// * `min_time` - Minimum duration in seconds the transaction must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_time(&mut self, min_time: Option<i32>) -> &mut Self {
        self.min_time = min_time;
        self
    }

    /// Gets the maximum time.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds the transaction must last
    pub fn max_time(&self) -> Option<i32> {
        self.max_time
    }

    /// Sets the maximum time.
    ///
    /// # Arguments
    ///
    /// * `max_time` - Maximum duration in seconds the transaction must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_time(&mut self, max_time: Option<i32>) -> &mut Self {
        self.max_time = max_time;
        self
    }

    /// Gets the minimum charging time.
    ///
    /// # Returns
    ///
    /// An optional minimum duration in seconds the charging must last
    pub fn min_charging_time(&self) -> Option<i32> {
        self.min_charging_time
    }

    /// Sets the minimum charging time.
    ///
    /// # Arguments
    ///
    /// * `min_charging_time` - Minimum duration in seconds the charging must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_charging_time(&mut self, min_charging_time: Option<i32>) -> &mut Self {
        self.min_charging_time = min_charging_time;
        self
    }

    /// Gets the maximum charging time.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds the charging must last
    pub fn max_charging_time(&self) -> Option<i32> {
        self.max_charging_time
    }

    /// Sets the maximum charging time.
    ///
    /// # Arguments
    ///
    /// * `max_charging_time` - Maximum duration in seconds the charging must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_charging_time(&mut self, max_charging_time: Option<i32>) -> &mut Self {
        self.max_charging_time = max_charging_time;
        self
    }

    /// Gets the minimum idle time.
    ///
    /// # Returns
    ///
    /// An optional minimum duration in seconds the idle period must last
    pub fn min_idle_time(&self) -> Option<i32> {
        self.min_idle_time
    }

    /// Sets the minimum idle time.
    ///
    /// # Arguments
    ///
    /// * `min_idle_time` - Minimum duration in seconds the idle period must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_idle_time(&mut self, min_idle_time: Option<i32>) -> &mut Self {
        self.min_idle_time = min_idle_time;
        self
    }

    /// Gets the maximum idle time.
    ///
    /// # Returns
    ///
    /// An optional maximum duration in seconds the idle period must last
    pub fn max_idle_time(&self) -> Option<i32> {
        self.max_idle_time
    }

    /// Sets the maximum idle time.
    ///
    /// # Arguments
    ///
    /// * `max_idle_time` - Maximum duration in seconds the idle period must last, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_idle_time(&mut self, max_idle_time: Option<i32>) -> &mut Self {
        self.max_idle_time = max_idle_time;
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

impl Default for TariffConditionsType {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tariff_conditions() {
        let tariff_conditions = TariffConditionsType::new();

        assert_eq!(tariff_conditions.start_time_of_day(), None);
        assert_eq!(tariff_conditions.end_time_of_day(), None);
        assert_eq!(tariff_conditions.day_of_week(), None);
        assert_eq!(tariff_conditions.valid_from_date(), None);
        assert_eq!(tariff_conditions.valid_to_date(), None);
        assert_eq!(tariff_conditions.evse_kind(), None);
        assert_eq!(tariff_conditions.min_energy(), None);
        assert_eq!(tariff_conditions.max_energy(), None);
        assert_eq!(tariff_conditions.min_current(), None);
        assert_eq!(tariff_conditions.max_current(), None);
        assert_eq!(tariff_conditions.min_power(), None);
        assert_eq!(tariff_conditions.max_power(), None);
        assert_eq!(tariff_conditions.min_time(), None);
        assert_eq!(tariff_conditions.max_time(), None);
        assert_eq!(tariff_conditions.min_charging_time(), None);
        assert_eq!(tariff_conditions.max_charging_time(), None);
        assert_eq!(tariff_conditions.min_idle_time(), None);
        assert_eq!(tariff_conditions.max_idle_time(), None);
        assert_eq!(tariff_conditions.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let start_time_of_day = "08:00".to_string();
        let end_time_of_day = "20:00".to_string();
        let day_of_week = vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday];
        let valid_from_date = "2023-01-01".to_string();
        let valid_to_date = "2023-12-31".to_string();
        let evse_kind = EvseKindEnumType::AC;
        let min_energy = Decimal::new(10000, 1); // 1000.0
        let max_energy = Decimal::new(500000, 1); // 50000.0
        let min_current = Decimal::new(50, 1); // 5.0
        let max_current = Decimal::new(320, 1); // 32.0
        let min_power = Decimal::new(37000, 1); // 3700.0
        let max_power = Decimal::new(220000, 1); // 22000.0
        let min_time = 300;
        let max_time = 18000;
        let min_charging_time = 600;
        let max_charging_time = 7200;
        let min_idle_time = 300;
        let max_idle_time = 1200;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_conditions = TariffConditionsType::new()
            .with_start_time_of_day(start_time_of_day.clone())
            .with_end_time_of_day(end_time_of_day.clone())
            .with_day_of_week(day_of_week.clone())
            .with_valid_from_date(valid_from_date.clone())
            .with_valid_to_date(valid_to_date.clone())
            .with_evse_kind(evse_kind.clone())
            .with_min_energy(min_energy)
            .with_max_energy(max_energy)
            .with_min_current(min_current)
            .with_max_current(max_current)
            .with_min_power(min_power)
            .with_max_power(max_power)
            .with_min_time(min_time)
            .with_max_time(max_time)
            .with_min_charging_time(min_charging_time)
            .with_max_charging_time(max_charging_time)
            .with_min_idle_time(min_idle_time)
            .with_max_idle_time(max_idle_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(
            tariff_conditions.start_time_of_day(),
            Some(start_time_of_day.as_str())
        );
        assert_eq!(
            tariff_conditions.end_time_of_day(),
            Some(end_time_of_day.as_str())
        );
        assert_eq!(tariff_conditions.day_of_week().unwrap().len(), 2);
        assert_eq!(
            tariff_conditions.valid_from_date(),
            Some(valid_from_date.as_str())
        );
        assert_eq!(
            tariff_conditions.valid_to_date(),
            Some(valid_to_date.as_str())
        );
        assert_eq!(tariff_conditions.evse_kind(), Some(&evse_kind));
        assert_eq!(tariff_conditions.min_energy(), Some(min_energy));
        assert_eq!(tariff_conditions.max_energy(), Some(max_energy));
        assert_eq!(tariff_conditions.min_current(), Some(min_current));
        assert_eq!(tariff_conditions.max_current(), Some(max_current));
        assert_eq!(tariff_conditions.min_power(), Some(min_power));
        assert_eq!(tariff_conditions.max_power(), Some(max_power));
        assert_eq!(tariff_conditions.min_time(), Some(min_time));
        assert_eq!(tariff_conditions.max_time(), Some(max_time));
        assert_eq!(
            tariff_conditions.min_charging_time(),
            Some(min_charging_time)
        );
        assert_eq!(
            tariff_conditions.max_charging_time(),
            Some(max_charging_time)
        );
        assert_eq!(tariff_conditions.min_idle_time(), Some(min_idle_time));
        assert_eq!(tariff_conditions.max_idle_time(), Some(max_idle_time));
        assert_eq!(tariff_conditions.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let mut tariff_conditions = TariffConditionsType::new();

        let start_time_of_day = "08:00".to_string();
        let end_time_of_day = "20:00".to_string();
        let day_of_week = vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday];
        let valid_from_date = "2023-01-01".to_string();
        let valid_to_date = "2023-12-31".to_string();
        let evse_kind = EvseKindEnumType::AC;
        let min_energy = Decimal::new(10000, 1); // 1000.0
        let max_energy = Decimal::new(500000, 1); // 50000.0
        let min_current = Decimal::new(50, 1); // 5.0
        let max_current = Decimal::new(320, 1); // 32.0
        let min_power = Decimal::new(37000, 1); // 3700.0
        let max_power = Decimal::new(220000, 1); // 22000.0
        let min_time = 300;
        let max_time = 18000;
        let min_charging_time = 600;
        let max_charging_time = 7200;
        let min_idle_time = 300;
        let max_idle_time = 1200;
        let custom_data = CustomDataType::new("VendorX".to_string());

        tariff_conditions
            .set_start_time_of_day(Some(start_time_of_day.clone()))
            .set_end_time_of_day(Some(end_time_of_day.clone()))
            .set_day_of_week(Some(day_of_week.clone()))
            .set_valid_from_date(Some(valid_from_date.clone()))
            .set_valid_to_date(Some(valid_to_date.clone()))
            .set_evse_kind(Some(evse_kind.clone()))
            .set_min_energy(Some(min_energy))
            .set_max_energy(Some(max_energy))
            .set_min_current(Some(min_current))
            .set_max_current(Some(max_current))
            .set_min_power(Some(min_power))
            .set_max_power(Some(max_power))
            .set_min_time(Some(min_time))
            .set_max_time(Some(max_time))
            .set_min_charging_time(Some(min_charging_time))
            .set_max_charging_time(Some(max_charging_time))
            .set_min_idle_time(Some(min_idle_time))
            .set_max_idle_time(Some(max_idle_time))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            tariff_conditions.start_time_of_day(),
            Some(start_time_of_day.as_str())
        );
        assert_eq!(
            tariff_conditions.end_time_of_day(),
            Some(end_time_of_day.as_str())
        );
        assert_eq!(tariff_conditions.day_of_week().unwrap().len(), 2);
        assert_eq!(
            tariff_conditions.valid_from_date(),
            Some(valid_from_date.as_str())
        );
        assert_eq!(
            tariff_conditions.valid_to_date(),
            Some(valid_to_date.as_str())
        );
        assert_eq!(tariff_conditions.evse_kind(), Some(&evse_kind));
        assert_eq!(tariff_conditions.min_energy(), Some(min_energy));
        assert_eq!(tariff_conditions.max_energy(), Some(max_energy));
        assert_eq!(tariff_conditions.min_current(), Some(min_current));
        assert_eq!(tariff_conditions.max_current(), Some(max_current));
        assert_eq!(tariff_conditions.min_power(), Some(min_power));
        assert_eq!(tariff_conditions.max_power(), Some(max_power));
        assert_eq!(tariff_conditions.min_time(), Some(min_time));
        assert_eq!(tariff_conditions.max_time(), Some(max_time));
        assert_eq!(
            tariff_conditions.min_charging_time(),
            Some(min_charging_time)
        );
        assert_eq!(
            tariff_conditions.max_charging_time(),
            Some(max_charging_time)
        );
        assert_eq!(tariff_conditions.min_idle_time(), Some(min_idle_time));
        assert_eq!(tariff_conditions.max_idle_time(), Some(max_idle_time));
        assert_eq!(tariff_conditions.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_conditions
            .set_start_time_of_day(None)
            .set_end_time_of_day(None)
            .set_day_of_week(None)
            .set_valid_from_date(None)
            .set_valid_to_date(None)
            .set_evse_kind(None)
            .set_min_energy(None)
            .set_max_energy(None)
            .set_min_current(None)
            .set_max_current(None)
            .set_min_power(None)
            .set_max_power(None)
            .set_min_time(None)
            .set_max_time(None)
            .set_min_charging_time(None)
            .set_max_charging_time(None)
            .set_min_idle_time(None)
            .set_max_idle_time(None)
            .set_custom_data(None);

        assert_eq!(tariff_conditions.start_time_of_day(), None);
        assert_eq!(tariff_conditions.end_time_of_day(), None);
        assert_eq!(tariff_conditions.day_of_week(), None);
        assert_eq!(tariff_conditions.valid_from_date(), None);
        assert_eq!(tariff_conditions.valid_to_date(), None);
        assert_eq!(tariff_conditions.evse_kind(), None);
        assert_eq!(tariff_conditions.min_energy(), None);
        assert_eq!(tariff_conditions.max_energy(), None);
        assert_eq!(tariff_conditions.min_current(), None);
        assert_eq!(tariff_conditions.max_current(), None);
        assert_eq!(tariff_conditions.min_power(), None);
        assert_eq!(tariff_conditions.max_power(), None);
        assert_eq!(tariff_conditions.min_time(), None);
        assert_eq!(tariff_conditions.max_time(), None);
        assert_eq!(tariff_conditions.min_charging_time(), None);
        assert_eq!(tariff_conditions.max_charging_time(), None);
        assert_eq!(tariff_conditions.min_idle_time(), None);
        assert_eq!(tariff_conditions.max_idle_time(), None);
        assert_eq!(tariff_conditions.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid conditions
        let valid_conditions = TariffConditionsType::new()
            .with_day_of_week(vec![DayOfWeekEnumType::Monday, DayOfWeekEnumType::Tuesday]);
        assert!(valid_conditions.validate().is_ok());

        // Test with invalid day_of_week (empty vector)
        let day_of_week: Vec<DayOfWeekEnumType> = vec![];
        let mut invalid_conditions = TariffConditionsType::new();
        invalid_conditions.day_of_week = Some(day_of_week);
        assert!(invalid_conditions.validate().is_err());

        // Test with too many days (more than 7)
        let day_of_week = vec![
            DayOfWeekEnumType::Monday,
            DayOfWeekEnumType::Tuesday,
            DayOfWeekEnumType::Wednesday,
            DayOfWeekEnumType::Thursday,
            DayOfWeekEnumType::Friday,
            DayOfWeekEnumType::Saturday,
            DayOfWeekEnumType::Sunday,
            DayOfWeekEnumType::Monday, // Duplicate to exceed the limit
        ];
        let mut invalid_conditions = TariffConditionsType::new();
        invalid_conditions.day_of_week = Some(day_of_week);
        assert!(invalid_conditions.validate().is_err());
    }
}
