/// Represent a signed version of the meter value.
/// SignedMeterValueType is used by: Common:SampledValueType
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    pub signed_meter_data: String,
    pub signing_method: String,
    pub encoding_method: String,
    pub public_key: String,
}
