/// Defines parameters required for initiating and maintaining wireless communication with other devices.
/// ModemType is used by: BootNotificationRequest.ChargingStationType
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}
