use super::sales_tariff_entry_type::SalesTariffEntryType;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// This dataType is based on dataTypes from ISO 15118-2.
/// SalesTariffType is used by: Common:ChargingScheduleType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tariff_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_e_price_levels: Option<i32>,
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 1024)))]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,
}
