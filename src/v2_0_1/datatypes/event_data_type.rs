use chrono::DateTime;
use chrono::Utc;

use super::component_type::ComponentType;
use super::variable_type::VariableType;
use crate::v2_0_1::enumerations::event_notification_enum_type::EventNotificationEnumType;
use crate::v2_0_1::enumerations::event_trigger_enum_type::EventTriggerEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Class to report an event notification for a component-variable.
/// EventDataType is used by: NotifyEventRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventDataType {
    pub event_id: i32,
    #[serde(with = "datetime_rfc3339 ")]
    pub timestamp: DateTime<Utc>,
    pub trigger: EventTriggerEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<i32>,
    pub actual_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_monitoring_id: Option<i32>,
    pub event_notification_type: EventNotificationEnumType,
    pub component: ComponentType,
    pub variable: VariableType,
}
