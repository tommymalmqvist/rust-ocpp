//! BootNotification
//!
//! The objective of this use case is to enable a Charging Station that is
//! powering up to register itself at a CSMS and provide the right state information.
//!
//! To be able to control Charging Stations connecting to a CSMS, Charging Stations
//! are required to send [`BootNotificationRequest`]. This request contains some
//! general information about the Charging Station.
//!
//!
use core::fmt;

use chrono::DateTime;
use chrono::Utc;

use crate::v2_0_1::datatypes::charging_station_type::ChargingStationType;
use crate::v2_0_1::datatypes::status_info_type::StatusInfoType;
use crate::v2_0_1::enumerations::boot_reason_enum_type::BootReasonEnumType;
use crate::v2_0_1::enumerations::registration_status_enum_type::RegistrationStatusEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// `BootNotificationRequest`, sent by the Charging Station to the CSMS when booting.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// This contains the reason for sending this message to the CSMS.
    pub reason: BootReasonEnumType,
    /// Identifies the Charging Station.
    pub charging_station: ChargingStationType,
}

/// `BootNotificationResponse`, sent by the CSMS to the Charging Station in response to a [`BootNotificationRequest`].
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// This contains the CSMS’s current time.
    #[serde(with = "datetime_rfc3339 ")]
    pub current_time: DateTime<Utc>,
    /// When [status](BootNotificationResponse::status) is Accepted, this contains the
    /// heartbeat interval in seconds.
    ///
    /// If the CSMS returns something other than Accepted, the value of the interval
    /// field indicates the minimum wait time before sending a next [`BootNotificationRequest`].
    pub interval: u16,
    /// This contains whether the Charging Station has been registered within the CSMS.
    pub status: RegistrationStatusEnumType,
    /// Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

impl fmt::Display for BootNotificationRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for BootNotificationResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
