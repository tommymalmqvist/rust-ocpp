use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use validator::Validate;

use super::charging_schedule_period_type::ChargingSchedulePeriodType;
use super::sales_tariff_type::SalesTariffType;
use crate::v2_0_1::enumerations::charging_rate_unit_enum_type::ChargingRateUnitEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Charging schedule structure defines a list of charging periods, as used in: GetCompositeSchedule.conf and ChargingProfile.
/// ChargingScheduleType is used by: Common:ChargingProfileType , NotifyChargingLimitRequest, NotifyEVChargingScheduleRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingScheduleType {
    /// Required. Identifies the ChargingSchedule.
    pub id: i32,
    /// Optional. Starting point of an absolute schedule. If absent the schedule will be relative to start of charging
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub start_schedule: Option<DateTime<Utc>>,
    /// Optional. Duration of the charging schedule in seconds. If the duration is left empty, the last period will continue indefinitely or until end of the transaction if chargingProfilePurpose = TxProfile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Required. The unit of measure Limit is expressed in.
    pub charging_rate_unit: ChargingRateUnitEnumType,
    /// Optional. Minimum charging rate supported by the EV. The unit of measure is defined by the chargingRateUnit. This parameter is intended to be used by a local smart charging algorithm to optimize the power allocation for in the case a charging process is inefficient at lower charging rates. Accepts at most one digit fraction (e.g. 8.1)
    #[serde(
        with = "rust_decimal::serde::arbitrary_precision_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub min_charging_rate: Option<Decimal>,
    /// Required. List of ChargingSchedulePeriod elements defining maximum power or current usage over time. The maximum number of periods, that is supported by the Charging Station, if less than 1024, is set by device model variable SmartChargingCtrlr.PeriodsPerSchedule
    #[validate(length(min = 1))]
    pub charging_schedule_period: Vec<ChargingSchedulePeriodType>,
    /// Optional. Sales tariff associated with this charging schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff: Option<SalesTariffType>,
}
