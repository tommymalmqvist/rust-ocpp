//! # From OCPP Specification
//! 4.2. Boot Notification
//! After start-up, a Charge Point SHALL send a request to the Central System with information
//! about its configuration (e.g. version, vendor, etc.). The Central System SHALL respond to
//! indicate whether it will accept the Charge Point.
//!
//! The Charge Point SHALL send a BootNotification.req PDU each time it boots or reboots. Between
//! the physical power-on/reboot and the successful completion of a BootNotification, where Central
//! System returns Accepted or Pending, the Charge Point SHALL NOT send any other request to the
//! Central System. This includes cached messages that are still present in the Charge Point from
//! before.
//!
//! When the Central System responds with a BootNotification.conf with a status Accepted, the
//! Charge Point will adjust the heartbeat interval in accordance with the interval from the
//! response PDU and it is RECOMMENDED to synchronize its internal clock with the supplied Central
//! System’s current time. If the Central System returns something other than Accepted, the value
//! of the interval field indicates the minimum wait time before sending a next BootNotification
//! request. If that interval value is zero, the Charge Point chooses a waiting interval on its
//! own, in a way that avoids flooding the Central System with requests. A Charge Point SHOULD NOT
//! send a BootNotification.req earlier, unless requested to do so with a TriggerMessage.req.
//! If the Central System returns the status Rejected, the Charge Point SHALL NOT send any OCPP
//! message to the Central System until the aforementioned retry interval has expired. During this
//! interval the Charge Point may no longer be reachable from the Central System. It MAY for
//! instance close its communication channel or shut down its communication hardware. Also the
//! Central System MAY close the communication channel, for instance to free up system resources.
//! While Rejected, the Charge Point SHALL NOT respond to any Central System initiated message. the
//! Central System SHOULD NOT initiate any.
//!
//! The Central System MAY also return a Pending registration status to indicate that it wants to
//! retrieve or set certain information on the Charge Point before the Central System will accept
//! the Charge Point. If the Central System returns the Pending status, the communication channel
//! SHOULD NOT be closed by either the Charge Point or the Central System. The Central System MAY
//! send request messages to retrieve information from the Charge Point or change its configuration.
//! The Charge Point SHOULD respond to these messages. The Charge Point SHALL NOT send request
//! messages to the Central System unless it has been instructed by the Central System to do so
//! with a TriggerMessage.req request.
//!
//! While in pending state, the following Central System initiated messages are not allowed:
//! RemoteStartTransaction.req and RemoteStopTransaction.req

use crate::v1_6::helpers::datetime_rfc3339;
use crate::v1_6::types::RegistrationStatus;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// # From OCPP Specification
    /// Optional. This contains a value that identifies the serial number of the Charge Box inside
    /// the Charge Point. Deprecated, will be removed in future version
    #[validate(length(min = 1, max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_box_serial_number: Option<String>,

    /// # From OCPP Specification
    /// Required. This contains a value that identifies the model of the ChargePoint.
    #[validate(length(min = 1, max = 20))]
    pub charge_point_model: String,

    /// # From OCPP Specification
    /// Optional. This contains a value that identifies the serial number of the Charge Point.
    #[validate(length(min = 1, max = 25))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_point_serial_number: Option<String>,

    /// # From OCPP Specification
    /// Required. This contains a value that identifies the vendor of the ChargePoint.
    #[validate(length(min = 1, max = 20))]
    pub charge_point_vendor: String,

    /// # From OCPP Specification
    /// Optional. This contains the firmware version of the Charge Point.
    #[validate(length(min = 1, max = 50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,

    /// # From OCPP Specification
    /// Optional. This contains the ICCID of the modem’s SIM card.
    #[validate(length(min = 1, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,

    /// # From OCPP Specification
    /// Optional. This contains the IMSI of the modem’s SIM card.
    #[validate(length(min = 1, max = 20))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,

    /// # From OCPP Specification
    /// Optional. This contains the serial number of the main electrical meter of the Charge Point.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 25))]
    pub meter_serial_number: Option<String>,

    /// # From OCPP Specification
    /// Optional. This contains the type of the main electrical meter of the Charge Point.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 25))]
    pub meter_type: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Validate, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// # From OCPP Specification
    /// Required. This contains the Central System’s current time.
    #[serde(with = "datetime_rfc3339")]
    pub current_time: DateTime<Utc>,

    /// # From OCPP Specification
    /// Required. When RegistrationStatus is Accepted, this contains the heartbeat interval in
    /// seconds. If the Central System returns something other than Accepted, the value of the
    /// interval field indicates the minimum wait time before sending a next BootNotification
    /// request.
    pub interval: u32,

    /// # From OCPP Specification
    /// Required. This contains whether the Charge Point has been registered within the System
    /// Central.
    pub status: RegistrationStatus,
}
