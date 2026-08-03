//! CostUpdated
#[cfg(not(feature = "std"))]
use alloc::string::String;
use rust_decimal::Decimal;

/// CostUpdatedRequest, sent by the CSMS to the Charging Station.
///
/// With this request the CSMS can send the current cost of a transaction to a Charging Station.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedRequest {
    /// Current total cost, based on the information known by the CSMS, of the transaction including taxes. In the currency configured with the configuration Variable: [Currency]
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision")
    )]
    pub total_cost: Decimal,
    /// Transaction Id of the transaction the current cost are asked for.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 36)))]
    pub transaction_id: String,
}

/// CostUpdatedResponse, sent by the Charging Station to the CSMS in response to [`CostUpdatedRequest`].
///
/// No fields are defined.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct CostUpdatedResponse {
    // No fields are defined.
}
