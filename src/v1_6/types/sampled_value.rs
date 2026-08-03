use super::{value_format::ValueFormat, Location, Measurand, Phase, ReadingContext, UnitOfMeasure};
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Single sampled value in MeterValues. Each value can be accompanied by optional fields.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct SampledValue {
    /// Required. Value as a “Raw” (decimal) number or “SignedData”. Field Type is “string” to allow for digitally signed data readings. Decimal numeric values are also acceptable to allow fractional values for measurands such as Temperature and Current.
    pub value: String,
    /// Optional. Type of detail value: start, end or sample. Default = “Sample.Periodic”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContext>,
    /// Optional. Raw or signed data. Default = “Raw”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
    /// Optional. Type of measurement. Default = “Energy.Active.Import.Register”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<Measurand>,
    /// Optional. indicates how the measured value is to be interpreted. For instance between L1 and neutral (L1-N) Please note that not all values of phase are applicable to all Measurands. When phase is absent, the measured value is interpreted as an overall value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
    /// Optional. Location of measurement. Default=”Outlet”
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. Unit of the value. Default = “Wh” if the (default) measurand is an “Energy” type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitOfMeasure>,
}
