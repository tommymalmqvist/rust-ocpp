use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, StatusInfoType},
    enumerations::{BootReasonEnumType, RegistrationStatusEnumType},
};

/// Defines parameters required for initiating and maintaining wireless communication with other devices.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. This contains the ICCID of the modem's SIM card.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub iccid: Option<String>,

    /// Optional. This contains the IMSI of the modem's SIM card.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub imsi: Option<String>,
}

/// The physical system where an Electrical Vehicle (EV) can be charged.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStationType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Optional. This contains the firmware version of the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub firmware_version: Option<String>,

    /// Required. Defines the model of the device.
    #[validate(length(max = 20))]
    pub model: String,

    /// Optional. Defines parameters required for initiating and maintaining wireless communication with other devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<ModemType>,

    /// Optional. Vendor-specific device identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 25))]
    pub serial_number: Option<String>,

    /// Required. Identifies the vendor (not necessarily in a unique manner).
    #[validate(length(max = 50))]
    pub vendor_name: String,
}

/// Request body for the BootNotification request.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    /// Required. The physical system where an Electrical Vehicle (EV) can be charged.
    pub charging_station: ChargingStationType,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the reason for sending this message to the CSMS.
    pub reason: BootReasonEnumType,
}

impl BootNotificationRequest {
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        validator::Validate::validate(self)?;
        self.charging_station.validate()?;
        if let Some(modem) = &self.charging_station.modem {
            modem.validate()?;
        }
        if let Some(custom_data) = &self.custom_data {
            custom_data.validate()?;
        }
        if let Some(custom_data) = &self.charging_station.custom_data {
            custom_data.validate()?;
        }
        if let Some(modem) = &self.charging_station.modem {
            if let Some(custom_data) = &modem.custom_data {
                custom_data.validate()?;
            }
        }
        Ok(())
    }
}

/// Response body for the BootNotification response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. This contains the CSMS's current time.
    pub current_time: DateTime<Utc>,

    /// Required. When Status is Accepted, this contains the heartbeat interval in seconds.
    /// If the CSMS returns something other than Accepted, the value of the interval field
    /// indicates the minimum wait time before sending a next BootNotification request.
    #[validate(range(min = 0))]
    pub interval: i32,

    /// Required. This contains whether the Charging Station has been registered within the CSMS.
    pub status: RegistrationStatusEnumType,

    /// Optional. Detailed status information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_info: Option<StatusInfoType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_boot_notification_request() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "ModelX".into(),
                vendor_name: "VendorY".into(),
                serial_number: Some("123456".into()),
                firmware_version: Some("v1.0.0".into()),
                modem: Some(ModemType {
                    iccid: Some("89123456789".into()),
                    imsi: Some("123456789".into()),
                    custom_data: None,
                }),
                custom_data: None,
            },
            custom_data: None,
        };

        // Validate the request
        assert!(request.validate().is_ok());

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(
            json,
            json!({
                "reason": "PowerUp",
                "chargingStation": {
                    "model": "ModelX",
                    "vendorName": "VendorY",
                    "serialNumber": "123456",
                    "firmwareVersion": "v1.0.0",
                    "modem": {
                        "iccid": "89123456789",
                        "imsi": "123456789"
                    }
                }
            })
        );
    }

    #[test]
    fn test_invalid_boot_notification_request() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "This model name is way too long and should fail validation".into(),
                vendor_name: "This vendor name is also way too long and should fail validation"
                    .into(),
                serial_number: Some("This serial number is also too long to be valid".into()),
                firmware_version: Some(
                    "This firmware version is way too long and should fail validation".into(),
                ),
                modem: None,
                custom_data: Some(CustomDataType::new("test_vendor".to_string())),
            },
            custom_data: Some(CustomDataType::new("test_vendor".to_string())),
        };

        // Validate the request - should fail
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_boot_notification_response() {
        let current_time = DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let response = BootNotificationResponse {
            current_time,
            interval: 300,
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                reason_code: "OK".into(),
                additional_info: Some("All good".into()),
                custom_data: None,
            }),
            custom_data: None,
        };

        // Validate the response
        assert!(response.validate().is_ok());

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(
            json,
            json!({
                "currentTime": "2023-01-01T12:00:00Z",
                "interval": 300,
                "status": "Accepted",
                "statusInfo": {
                    "reasonCode": "OK",
                    "additionalInfo": "All good"
                }
            })
        );
    }

    #[test]
    fn test_invalid_boot_notification_response() {
        let response = BootNotificationResponse {
            current_time: Utc::now(),
            interval: -1, // Invalid interval
            status: RegistrationStatusEnumType::Accepted,
            status_info: Some(StatusInfoType {
                reason_code: "This reason code is way too long to be valid and should cause validation to fail because it exceeds the maximum length allowed for reason codes in the OCPP specification".into(),
                additional_info: None,
                custom_data: None,
            }),
            custom_data: None,
        };

        // Validate the response - should fail
        assert!(response.validate().is_err());
    }

    #[test]
    fn test_boot_notification_request_with_custom_data() {
        let request = BootNotificationRequest {
            reason: BootReasonEnumType::PowerUp,
            charging_station: ChargingStationType {
                model: "ModelZ".into(),
                vendor_name: "VendorZ".into(),
                serial_number: Some("987654321".into()),
                firmware_version: Some("v9.9.9".into()),
                modem: Some(ModemType {
                    iccid: Some("iccid12345".into()),
                    imsi: Some("imsi54321".into()),
                    custom_data: Some(CustomDataType::new("VendorZ".to_string())),
                }),
                custom_data: Some(CustomDataType::new("VendorZ".to_string())),
            },
            custom_data: Some(CustomDataType::new("VendorZ".to_string())),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: BootNotificationRequest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }
}
