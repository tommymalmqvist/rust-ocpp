use rust_decimal::Decimal;

/// Charging schedule period structure defines a time period in a charging schedule, as used in: ChargingSchedule.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    /// Required. Start of the period, in seconds from the start of schedule. The value of StartPeriod also defines the stop time of the previous period.
    pub start_period: i32,
    /// Required. Charging rate limit during the schedule period, in the applicable chargingRateUnit, for example in Amperes or Watts. Accepts at most one digit fraction (e.g. 8.1).
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision")
    )]
    pub limit: Decimal,
    /// Optional. The number of phases that can be used for charging. If a number of phases is needed, numberPhases=3 will be assumed unless another number is given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
}
