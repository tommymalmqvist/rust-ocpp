use crate::v2_1::datatypes::{CustomDataType, StatusInfoType};
use crate::v2_1::enumerations::CancelReservationStatusEnumType;
use crate::v2_1::messages::cancel_reservation::{
    CancelReservationRequest, CancelReservationResponse,
};
use jsonschema::Validator;
use serde_json::Value;

const SCHEMA_DIR: &str = "src/tests/schema_validation/schemas/v2.1";

// Helper function to validate schema and instance with detailed error reporting
fn validate_schema_instance(
    schema_name: &str,
    instance: Value,
) -> Result<bool, Box<dyn std::error::Error>> {
    let schema_path = format!("{}/{}", SCHEMA_DIR, schema_name);
    let schema_str = std::fs::read_to_string(schema_path)?;
    let schema = serde_json::from_str(&schema_str)?;
    let compiled = Validator::new(&schema).expect("A valid schema");
    let result = compiled.validate(&instance);

    if result.is_err() {
        for error in compiled.iter_errors(&instance) {
            println!("Validation error: {}", error);
            println!("Instance path: {}", error.instance_path);
        }
    }

    Ok(compiled.is_valid(&instance))
}

#[test]
fn test_valid_boot_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_boot_notification_missing_required_field() -> Result<(), Box<dyn std::error::Error>>
{
    let instance = serde_json::json!({
        "reason": "PowerUp",
        // Missing required chargingStation field
    });

    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            "type": "ISO14443"
        }
    });

    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_authorize_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234",
            // Missing required 'type' field
        }
    });

    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_boot_notification_request_additional_field() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        },
        "additionalField": "this should NOT be allowed"  // OCPP 2.1 is strict about additional properties
    });

    // The validation should fail because OCPP 2.1 doesn't allow additional properties
    assert!(!validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_request_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reason": "PowerUp",
        "chargingStation": {
            "model": "ModelX",
            "vendorName": "VendorY"
        }
    });

    assert!(validate_schema_instance(
        "BootNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_boot_notification_response_v2_1() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "currentTime": "2023-10-10T10:10:10Z",
        "interval": 300,
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "BootNotificationResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_id_token_type_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
    // Test with all optional fields
    let instance = serde_json::json!({
        "idToken": {
            "additionalInfo": [{
                "additionalIdToken": "TEST123",
                "type": "someType"
            }],
            "idToken": "ABCD1234567890",
            "type": "ISO14443",
            "customData": {
                "vendorId": "TestVendor"
            }
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test with only required fields
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890",
            "type": "Central"
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test with maximum length strings
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "A".repeat(255),
            "type": "A".repeat(20)
        }
    });
    assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);

    // Test all predefined values
    for type_value in [
        "Central",
        "DirectPayment",
        "eMAID",
        "EVCCID",
        "ISO14443",
        "ISO15693",
        "KeyCode",
        "Local",
        "MacAddress",
        "NoAuthorization",
        "VIN",
    ] {
        let instance = serde_json::json!({
            "idToken": {
                "idToken": "ABCD1234567890",
                "type": type_value
            }
        });
        assert!(validate_schema_instance("AuthorizeRequest.json", instance)?);
    }

    Ok(())
}

#[test]
fn test_invalid_id_token_type() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890"
            // Missing required 'type' field
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    // Test with empty additionalInfo array (violates minItems: 1)
    let instance = serde_json::json!({
        "idToken": {
            "additionalInfo": [],
            "idToken": "ABCD1234567890",
            "type": "ISO14443"
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    // Test with too long strings
    let instance = serde_json::json!({
        "idToken": {
            "idToken": "A".repeat(256),
            "type": "ISO14443"
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    let instance = serde_json::json!({
        "idToken": {
            "idToken": "ABCD1234567890",
            "type": "A".repeat(21)  // Type string too long
        }
    });
    assert!(!validate_schema_instance(
        "AuthorizeRequest.json",
        instance
    )?);

    Ok(())
}

#[test]
fn test_valid_adjust_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 42,
        "params": {
            "interval": 300,
            "values": 5
        }
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_adjust_periodic_event_stream_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamResponse.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "InvalidParameters",
            "additionalInfo": "Values must be greater than 0"
        }
    });

    assert!(validate_schema_instance(
        "AdjustPeriodicEventStreamResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_adjust_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "id": 42
        // Missing required params field
    });

    assert!(!validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);

    // Test with negative values
    let instance = serde_json::json!({
        "id": -1,  // Must be >= 0
        "params": {
            "interval": 300,
            "values": 5
        }
    });

    assert!(!validate_schema_instance(
        "AdjustPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_afrr_signal_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "2024-01-01T12:00:00Z"
    });

    assert!(validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "2024-01-01T12:00:00Z",
        "customData": {
            "vendorId": "TestVendor"
        }
    });

    assert!(validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_afrr_signal_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });

    assert!(validate_schema_instance(
        "AFRRSignalResponse.json",
        instance
    )?);

    // Test with optional fields
    let instance = serde_json::json!({
        "status": "Rejected",
        "statusInfo": {
            "reasonCode": "InvalidSignal",
            "additionalInfo": "Signal value out of range"
        }
    });

    assert!(validate_schema_instance(
        "AFRRSignalResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_afrr_signal_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "signal": 100
        // Missing required timestamp field
    });

    assert!(!validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);

    // Test with invalid timestamp format
    let instance = serde_json::json!({
        "signal": 100,
        "timestamp": "invalid-date-time"
    });

    assert!(!validate_schema_instance(
        "AFRRSignalRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 80.5,
            "soH": 95.0
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with all optional fields
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 80.5,
            "soH": 95.0,
            "productionDate": "2024-01-01T12:00:00Z",
            "vendorInfo": "Manufacturer XYZ",
            "customData": {
                "vendorId": "TestVendor"
            }
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42,
        "customData": {
            "vendorId": "TestVendor"
        }
    });

    assert!(validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_battery_swap_response() -> Result<(), Box<dyn std::error::Error>> {
    // Test empty response
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "BatterySwapResponse.json",
        instance
    )?);

    // Test with optional custom data
    let instance = serde_json::json!({
        "customData": {
            "vendorId": "TestVendor"
        }
    });
    assert!(validate_schema_instance(
        "BatterySwapResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    // Test with missing required field
    let instance = serde_json::json!({
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
        // Missing required batteryData field
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with empty batteryData array
    let instance = serde_json::json!({
        "batteryData": [],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);

    // Test with invalid SoC value
    let instance = serde_json::json!({
        "batteryData": [{
            "evseId": 1,
            "serialNumber": "BATTERY123",
            "soC": 101.0,  // Must be <= 100
            "soH": 95.0
        }],
        "eventType": "BatteryIn",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        },
        "requestId": 42
    });

    assert!(!validate_schema_instance(
        "BatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn validate_cancel_reservation_request() -> Result<(), Box<dyn std::error::Error>> {
    let test = CancelReservationRequest {
        reservation_id: 42,
        custom_data: None, // Schema doesn't allow custom_data
    };

    let instance = serde_json::to_value(test)?;
    assert!(validate_schema_instance(
        "CancelReservationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn validate_cancel_reservation_response() -> Result<(), Box<dyn std::error::Error>> {
    let test = CancelReservationResponse {
        custom_data: Some(CustomDataType::new("test_vendor".to_string())),
        status: CancelReservationStatusEnumType::Accepted,
        status_info: Some(StatusInfoType {
            reason_code: "NoReservation".to_string(),
            additional_info: Some("No active reservation found".to_string()),
            custom_data: Some(CustomDataType::new("test_vendor".to_string())),
        }),
    };

    let instance = serde_json::to_value(test)?;
    assert!(validate_schema_instance(
        "CancelReservationResponse.json",
        instance
    )?);
    Ok(())
}

// We recommend installing an extension to run rust tests.
