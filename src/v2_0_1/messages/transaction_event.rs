#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::v2_0_1::datatypes::evse_type::EVSEType;
use crate::v2_0_1::datatypes::id_token_info_type::IdTokenInfoType;
use crate::v2_0_1::datatypes::id_token_type::IdTokenType;
use crate::v2_0_1::datatypes::message_content_type::MessageContentType;
use crate::v2_0_1::datatypes::meter_value_type::MeterValueType;
use crate::v2_0_1::datatypes::transaction_type::TransactionType;
use crate::v2_0_1::enumerations::transaction_event_enum_type::TransactionEventEnumType;
use crate::v2_0_1::enumerations::trigger_reason_enum_type::TriggerReasonEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Sent by the Charging Station to the CSMS to request that the Certificate Authority signs the public key into a certificate.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    pub event_type: TransactionEventEnumType,
    #[serde(with = "datetime_rfc3339")]
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnumType,
    pub seq_no: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    pub transaction_info: TransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter_value: Option<Vec<MeterValueType>>,
}

/// This contains the field definition of the TransactionEventResponse PDU sent by the CSMS to the Charging Station in response to a TransactionEventRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[cfg_attr(
        feature = "std",
        serde(with = "rust_decimal::serde::arbitrary_precision_option")
    )]
    #[cfg_attr(
        not(feature = "std"),
        serde(with = "crate::helpers::decimal_arbitrary_precision::option")
    )]
    pub total_cost: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfoType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContentType>,
}
