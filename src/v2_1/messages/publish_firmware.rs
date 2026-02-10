use crate::v2_1::datatypes::status_info::StatusInfoType;
use crate::v2_1::enumerations::generic_status::GenericStatusEnumType;

/// This contains the field definition of the PublishFirmwareRequest PDU sent by the CSMS to the Local Controller.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareRequest {
    /// This contains a string containing a URI pointing to a
    /// location from which to retrieve the firmware.
    pub location: String,

    /// This specifies how many times Charging Station must retry
    /// to download the firmware before giving up. If this field is not
    /// present, it is left to Charging Station to decide how many times it wants to retry.
    /// If the value is 0, it means: no retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,

    /// The MD5 checksum over the entire firmware file as a hexadecimal string of length 32.
    pub checksum: String,

    /// The Id of the request.
    pub request_id: i32,

    /// The interval in seconds
    /// after which a retry may be
    /// attempted. If this field is not
    /// present, it is left to Charging
    /// Station to decide how long to wait
    /// between attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i32>,
}

/// This contains the field definition of the PublishFirmwareResponse PDU sent by the Local Controller to the CSMS in response to a PublishFirmwareRequest.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishFirmwareResponse {
    /// Indicates whether the request was accepted.
    pub status: GenericStatusEnumType,

    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}
