use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, MessageContentType, StatusInfoType},
    enumerations::{DayOfWeekEnumType, EvseKindEnumType, TariffChangeStatusEnumType},
};

/// These conditions describe if a FixedPrice applies at start of the transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsFixedType {
    /// Optional. Start time of day in local time.
    /// Format as per RFC 3339: time-hour ":" time-minute
    /// Must be in 24h format with leading zeros. Hour/Minute separator: ":"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,

    /// Optional. End time of day in local time. Same syntax as start_time_of_day.
    /// If end time < start time then the period wraps around to the next day.
    /// To stop at end of the day use: 00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_of_day: Option<String>,

    /// Optional. Day(s) of the week this tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 7))]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,

    /// Optional. Start date in local time, for example: 2015-12-24.
    /// Valid from this day (inclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,

    /// Optional. End date in local time, for example: 2015-12-27.
    /// Valid until this day (exclusive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,

    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,

    /// Optional. For which payment brand this (adhoc) tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub payment_brand: Option<String>,

    /// Optional. Type of adhoc payment, e.g. CC, Debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub payment_recognition: Option<String>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// These conditions describe if and when a TariffEnergyType or TariffTimeType applies during a transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffConditionsType {
    /// Optional. Start time of day in local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_of_day: Option<String>,

    /// Optional. End time of day in local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_of_day: Option<String>,

    /// Optional. Day(s) of the week this tariff applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 7))]
    pub day_of_week: Option<Vec<DayOfWeekEnumType>>,

    /// Optional. Start date in local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<String>,

    /// Optional. End date in local time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<String>,

    /// Optional. Type of EVSE (AC, DC) this tariff applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_kind: Option<EvseKindEnumType>,

    /// Optional. Minimum consumed energy in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_energy: Option<f64>,

    /// Optional. Maximum consumed energy in Wh.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_energy: Option<f64>,

    /// Optional. Minimum current in Amperes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_current: Option<f64>,

    /// Optional. Maximum current in Amperes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_current: Option<f64>,

    /// Optional. Minimum power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<f64>,

    /// Optional. Maximum power in W.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_power: Option<f64>,

    /// Optional. Minimum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_time: Option<i32>,

    /// Optional. Maximum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_time: Option<i32>,

    /// Optional. Minimum charging duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_time: Option<i32>,

    /// Optional. Maximum charging duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_charging_time: Option<i32>,

    /// Optional. Minimum idle duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_idle_time: Option<i32>,

    /// Optional. Maximum idle duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_time: Option<i32>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Tariff with optional conditions for an energy price.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffEnergyPriceType {
    /// Required. Price per kWh (excl. tax) for this element.
    pub price_kwh: f64,

    /// Optional. Conditions when this tariff element applies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<TariffConditionsType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Request to change the tariff for an ongoing transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffRequest {
    /// Required. Transaction Id for which the tariff needs to be changed.
    pub transaction_id: String,

    /// Required. The new tariff that should be applied.
    pub tariff_id: String,

    /// Optional. Message content to be displayed to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_content: Option<MessageContentType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}

/// Response to a ChangeTransactionTariffRequest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChangeTransactionTariffResponse {
    /// Required. Status indicating whether the Charging Station accepts the request.
    pub status: TariffChangeStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,
}
