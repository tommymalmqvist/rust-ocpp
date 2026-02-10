use super::{
    CustomData, IdToken, IdTokenInfo, MessageContent, MeterValue, Transaction,
    TransactionEventEnum, TransactionLimit, TriggerReasonEnum, EVSE,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionEventRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    pub event_type: TransactionEventEnum,
    pub meter_value: Vec<MeterValue>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub trigger_reason: TriggerReasonEnum,
    pub seq_no: i32,
    pub transaction_info: Transaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_phases_used: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cable_max_current: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSE>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdToken>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TransactionEventResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token_info: Option<IdTokenInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_limit: Option<TransactionLimit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message: Option<MessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_personal_message_extra: Option<Vec<MessageContent>>,
}

impl TransactionEventRequest {
    pub fn new(
        event_type: TransactionEventEnum,
        meter_value: Vec<MeterValue>,
        timestamp: chrono::DateTime<chrono::Utc>,
        trigger_reason: TriggerReasonEnum,
        seq_no: i32,
        transaction_info: Transaction,
    ) -> Self {
        Self {
            custom_data: None,
            event_type,
            meter_value,
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info,
            offline: None,
            number_of_phases_used: None,
            cable_max_current: None,
            reservation_id: None,
            evse: None,
            id_token: None,
        }
    }
}

impl TransactionEventResponse {
    pub fn new() -> Self {
        Self {
            custom_data: None,
            total_cost: None,
            charging_priority: None,
            id_token_info: None,
            transaction_limit: None,
            updated_personal_message: None,
            updated_personal_message_extra: None,
        }
    }
}
