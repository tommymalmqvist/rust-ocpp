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
            println!("Instance path: {}", error.instance_path());
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

// ============================================================
// Authorize Response
// ============================================================

#[test]
fn test_valid_authorize_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "idTokenInfo": {
            "status": "Accepted"
        }
    });
    assert!(validate_schema_instance(
        "AuthorizeResponse.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_authorize_response() -> Result<(), Box<dyn std::error::Error>> {
    // Missing required idTokenInfo field
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "AuthorizeResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// CertificateSigned
// ============================================================

#[test]
fn test_valid_certificate_signed_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "certificateChain": "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkq\n-----END CERTIFICATE-----"
    });
    assert!(validate_schema_instance(
        "CertificateSignedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_certificate_signed_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "CertificateSignedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_certificate_signed_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "CertificateSignedResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ChangeAvailability
// ============================================================

#[test]
fn test_valid_change_availability_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "operationalStatus": "Operative"
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_change_availability_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ChangeAvailabilityRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_change_availability_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ChangeAvailabilityResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ChangeTransactionTariff
// ============================================================

#[test]
fn test_valid_change_transaction_tariff_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX123",
        "tariff": {
            "tariffId": "TARIFF1",
            "currency": "EUR"
        }
    });
    assert!(validate_schema_instance(
        "ChangeTransactionTariffRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_change_transaction_tariff_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ChangeTransactionTariffRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_change_transaction_tariff_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ChangeTransactionTariffResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearCache
// ============================================================

#[test]
fn test_valid_clear_cache_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClearCacheRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_cache_request() -> Result<(), Box<dyn std::error::Error>> {
    // additionalProperties: false
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "ClearCacheRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_cache_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearCacheResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearChargingProfile
// ============================================================

#[test]
fn test_valid_clear_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "ClearChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_charging_profile_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearChargingProfileResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearDERControl
// ============================================================

#[test]
fn test_valid_clear_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "isDefault": true
    });
    assert!(validate_schema_instance(
        "ClearDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_der_control_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearDERControlResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearDisplayMessage
// ============================================================

#[test]
fn test_valid_clear_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 1
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_display_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ClearDisplayMessageResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearedChargingLimit
// ============================================================

#[test]
fn test_valid_cleared_charging_limit_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "chargingLimitSource": "EMS"
    });
    assert!(validate_schema_instance(
        "ClearedChargingLimitRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_cleared_charging_limit_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearedChargingLimitRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_cleared_charging_limit_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClearedChargingLimitResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearTariffs
// ============================================================

#[test]
fn test_valid_clear_tariffs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClearTariffsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_tariffs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "ClearTariffsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_tariffs_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "clearTariffsResult": [{"status": "Accepted"}]
    });
    assert!(validate_schema_instance(
        "ClearTariffsResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClearVariableMonitoring
// ============================================================

#[test]
fn test_valid_clear_variable_monitoring_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": [1, 2, 3]
    });
    assert!(validate_schema_instance(
        "ClearVariableMonitoringRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_clear_variable_monitoring_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClearVariableMonitoringRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_clear_variable_monitoring_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "clearMonitoringResult": [{"status": "Accepted", "id": 1}]
    });
    assert!(validate_schema_instance(
        "ClearVariableMonitoringResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ClosePeriodicEventStream
// ============================================================

#[test]
fn test_valid_close_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 1
    });
    assert!(validate_schema_instance(
        "ClosePeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_close_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ClosePeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_close_periodic_event_stream_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ClosePeriodicEventStreamResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// CostUpdated
// ============================================================

#[test]
fn test_valid_cost_updated_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "totalCost": 12.50,
        "transactionId": "TX789"
    });
    assert!(validate_schema_instance(
        "CostUpdatedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_cost_updated_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "CostUpdatedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_cost_updated_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "CostUpdatedResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// CustomerInformation
// ============================================================

#[test]
fn test_valid_customer_information_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "report": true,
        "clear": false
    });
    assert!(validate_schema_instance(
        "CustomerInformationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_customer_information_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "CustomerInformationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_customer_information_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "CustomerInformationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// DataTransfer
// ============================================================

#[test]
fn test_valid_data_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "vendorId": "com.example.vendor"
    });
    assert!(validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_data_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "DataTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_data_transfer_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "DataTransferResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// DeleteCertificate
// ============================================================

#[test]
fn test_valid_delete_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "certificateHashData": {
            "hashAlgorithm": "SHA256",
            "issuerNameHash": "AABBCCDD",
            "issuerKeyHash": "EEFF0011",
            "serialNumber": "01234567"
        }
    });
    assert!(validate_schema_instance(
        "DeleteCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_delete_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "DeleteCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_delete_certificate_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "DeleteCertificateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// FirmwareStatusNotification
// ============================================================

#[test]
fn test_valid_firmware_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Downloaded"
    });
    assert!(validate_schema_instance(
        "FirmwareStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_firmware_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "FirmwareStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_firmware_status_notification_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "FirmwareStatusNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// Get15118EVCertificate
// ============================================================

#[test]
fn test_valid_get_15118_ev_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "iso15118SchemaVersion": "urn:iso:15118:2:2013:MsgDef",
        "action": "Install",
        "exiRequest": "AAABBB"
    });
    assert!(validate_schema_instance(
        "Get15118EVCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_15118_ev_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "Get15118EVCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_15118_ev_certificate_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted",
        "exiResponse": "AAABBB"
    });
    assert!(validate_schema_instance(
        "Get15118EVCertificateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetBaseReport
// ============================================================

#[test]
fn test_valid_get_base_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "reportBase": "FullInventory"
    });
    assert!(validate_schema_instance(
        "GetBaseReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_base_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetBaseReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_base_report_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetBaseReportResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetCertificateChainStatus
// ============================================================

#[test]
fn test_valid_get_certificate_chain_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "certificateStatusRequests": [{
            "source": "OCSP",
            "urls": ["https://ocsp.example.com"],
            "certificateHashData": {
                "hashAlgorithm": "SHA256",
                "issuerNameHash": "AABBCCDD",
                "issuerKeyHash": "EEFF0011",
                "serialNumber": "01234567"
            }
        }]
    });
    assert!(validate_schema_instance(
        "GetCertificateChainStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_certificate_chain_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetCertificateChainStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_certificate_chain_status_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "certificateStatus": [{
            "source": "OCSP",
            "status": "Good",
            "nextUpdate": "2024-12-31T23:59:59Z",
            "certificateHashData": {
                "hashAlgorithm": "SHA256",
                "issuerNameHash": "AABBCCDD",
                "issuerKeyHash": "EEFF0011",
                "serialNumber": "01234567"
            }
        }]
    });
    assert!(validate_schema_instance(
        "GetCertificateChainStatusResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetCertificateStatus
// ============================================================

#[test]
fn test_valid_get_certificate_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "ocspRequestData": {
            "hashAlgorithm": "SHA256",
            "issuerNameHash": "AABBCCDD",
            "issuerKeyHash": "EEFF0011",
            "serialNumber": "01234567",
            "responderURL": "https://ocsp.example.com"
        }
    });
    assert!(validate_schema_instance(
        "GetCertificateStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_certificate_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetCertificateStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_certificate_status_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetCertificateStatusResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetChargingProfiles
// ============================================================

#[test]
fn test_valid_get_charging_profiles_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "chargingProfile": {}
    });
    assert!(validate_schema_instance(
        "GetChargingProfilesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_charging_profiles_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetChargingProfilesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_charging_profiles_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetChargingProfilesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetCompositeSchedule
// ============================================================

#[test]
fn test_valid_get_composite_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "duration": 3600,
        "evseId": 1
    });
    assert!(validate_schema_instance(
        "GetCompositeScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_composite_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetCompositeScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_composite_schedule_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetCompositeScheduleResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetDERControl
// ============================================================

#[test]
fn test_valid_get_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "GetDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_der_control_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetDERControlResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetDisplayMessages
// ============================================================

#[test]
fn test_valid_get_display_messages_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "GetDisplayMessagesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_display_messages_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetDisplayMessagesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_display_messages_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetDisplayMessagesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetInstalledCertificateIds
// ============================================================

#[test]
fn test_valid_get_installed_certificate_ids_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetInstalledCertificateIdsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_installed_certificate_ids_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "GetInstalledCertificateIdsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_installed_certificate_ids_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetInstalledCertificateIdsResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetLocalListVersion
// ============================================================

#[test]
fn test_valid_get_local_list_version_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetLocalListVersionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_local_list_version_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "GetLocalListVersionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_local_list_version_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "versionNumber": 5
    });
    assert!(validate_schema_instance(
        "GetLocalListVersionResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetLog
// ============================================================

#[test]
fn test_valid_get_log_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "logType": "DiagnosticsLog",
        "requestId": 1,
        "log": {
            "remoteLocation": "ftp://logs.example.com/upload"
        }
    });
    assert!(validate_schema_instance("GetLogRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_get_log_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance("GetLogRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_valid_get_log_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance("GetLogResponse.json", instance)?);
    Ok(())
}

// ============================================================
// GetMonitoringReport
// ============================================================

#[test]
fn test_valid_get_monitoring_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "GetMonitoringReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_monitoring_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetMonitoringReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_monitoring_report_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetMonitoringReportResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetPeriodicEventStream
// ============================================================

#[test]
fn test_valid_get_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "GetPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_periodic_event_stream_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetPeriodicEventStreamResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetReport
// ============================================================

#[test]
fn test_valid_get_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance("GetReportRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_get_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_report_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetReportResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetTariffs
// ============================================================

#[test]
fn test_valid_get_tariffs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1
    });
    assert!(validate_schema_instance(
        "GetTariffsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_tariffs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetTariffsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_tariffs_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "GetTariffsResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetTransactionStatus
// ============================================================

#[test]
fn test_valid_get_transaction_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "GetTransactionStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_transaction_status_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "GetTransactionStatusRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_transaction_status_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "messagesInQueue": false
    });
    assert!(validate_schema_instance(
        "GetTransactionStatusResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// GetVariables
// ============================================================

#[test]
fn test_valid_get_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "getVariableData": [{
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_get_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "GetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_get_variables_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "getVariableResult": [{
            "attributeStatus": "Accepted",
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "GetVariablesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// Heartbeat
// ============================================================

#[test]
fn test_valid_heartbeat_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance("HeartbeatRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_heartbeat_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({"unknownField": "value"});
    assert!(!validate_schema_instance(
        "HeartbeatRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_heartbeat_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "currentTime": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "HeartbeatResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// InstallCertificate
// ============================================================

#[test]
fn test_valid_install_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "certificateType": "CSMSRootCertificate",
        "certificate": "-----BEGIN CERTIFICATE-----\nMIIBIjANBgkq\n-----END CERTIFICATE-----"
    });
    assert!(validate_schema_instance(
        "InstallCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_install_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "InstallCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_install_certificate_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "InstallCertificateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// LogStatusNotification
// ============================================================

#[test]
fn test_valid_log_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Uploaded"
    });
    assert!(validate_schema_instance(
        "LogStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_log_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "LogStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_log_status_notification_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "LogStatusNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// MeterValues
// ============================================================

#[test]
fn test_valid_meter_values_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "meterValue": [{
            "timestamp": "2024-01-01T12:00:00Z",
            "sampledValue": [{"value": 230.0}]
        }]
    });
    assert!(validate_schema_instance(
        "MeterValuesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_meter_values_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "MeterValuesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_meter_values_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "MeterValuesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyAllowedEnergyTransfer
// ============================================================

#[test]
fn test_valid_notify_allowed_energy_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX123",
        "allowedEnergyTransfer": ["AC_single_phase", "DC"]
    });
    assert!(validate_schema_instance(
        "NotifyAllowedEnergyTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_allowed_energy_transfer_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyAllowedEnergyTransferRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_allowed_energy_transfer_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "NotifyAllowedEnergyTransferResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyChargingLimit
// ============================================================

#[test]
fn test_valid_notify_charging_limit_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "chargingLimit": {
            "chargingLimitSource": "EMS"
        }
    });
    assert!(validate_schema_instance(
        "NotifyChargingLimitRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_charging_limit_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyChargingLimitRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_charging_limit_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyChargingLimitResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyCustomerInformation
// ============================================================

#[test]
fn test_valid_notify_customer_information_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "data": "Customer data here",
        "seqNo": 0,
        "generatedAt": "2024-01-01T12:00:00Z",
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "NotifyCustomerInformationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_customer_information_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyCustomerInformationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_customer_information_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyCustomerInformationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyDERAlarm
// ============================================================

#[test]
fn test_valid_notify_der_alarm_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "controlType": "EnterService",
        "timestamp": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "NotifyDERAlarmRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_der_alarm_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyDERAlarmRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_der_alarm_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyDERAlarmResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyDERStartStop
// ============================================================

#[test]
fn test_valid_notify_der_start_stop_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "controlId": "CTRL001",
        "started": true,
        "timestamp": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "NotifyDERStartStopRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_der_start_stop_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyDERStartStopRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_der_start_stop_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyDERStartStopResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyDisplayMessages
// ============================================================

#[test]
fn test_valid_notify_display_messages_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "NotifyDisplayMessagesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_display_messages_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyDisplayMessagesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_display_messages_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyDisplayMessagesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyEVChargingNeeds
// ============================================================

#[test]
fn test_valid_notify_ev_charging_needs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "chargingNeeds": {
            "requestedEnergyTransfer": "AC_three_phase"
        }
    });
    assert!(validate_schema_instance(
        "NotifyEVChargingNeedsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_ev_charging_needs_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyEVChargingNeedsRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_ev_charging_needs_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "NotifyEVChargingNeedsResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyEVChargingSchedule
// ============================================================

#[test]
fn test_valid_notify_ev_charging_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "timeBase": "2024-01-01T12:00:00Z",
        "evseId": 1,
        "chargingSchedule": {
            "id": 1,
            "chargingRateUnit": "W",
            "chargingSchedulePeriod": [{"startPeriod": 0}]
        }
    });
    assert!(validate_schema_instance(
        "NotifyEVChargingScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_ev_charging_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyEVChargingScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_ev_charging_schedule_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "NotifyEVChargingScheduleResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyEvent
// ============================================================

#[test]
fn test_valid_notify_event_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "generatedAt": "2024-01-01T12:00:00Z",
        "seqNo": 0,
        "eventData": [{
            "eventId": 1,
            "timestamp": "2024-01-01T12:00:00Z",
            "trigger": "Alerting",
            "actualValue": "true",
            "eventNotificationType": "HardWiredNotification",
            "component": {"name": "ChargingStation"},
            "variable": {"name": "Available"}
        }]
    });
    assert!(validate_schema_instance(
        "NotifyEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_event_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_event_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyEventResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyMonitoringReport
// ============================================================

#[test]
fn test_valid_notify_monitoring_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "seqNo": 0,
        "generatedAt": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "NotifyMonitoringReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_monitoring_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyMonitoringReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_monitoring_report_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyMonitoringReportResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyPeriodicEventStream (single file, no separate response)
// ============================================================

#[test]
fn test_valid_notify_periodic_event_stream() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 1,
        "pending": 0,
        "basetime": "2024-01-01T12:00:00Z",
        "data": [{"t": 0, "v": "100"}]
    });
    assert!(validate_schema_instance(
        "NotifyPeriodicEventStream.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_periodic_event_stream() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyPeriodicEventStream.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyPriorityCharging
// ============================================================

#[test]
fn test_valid_notify_priority_charging_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX123",
        "activated": true
    });
    assert!(validate_schema_instance(
        "NotifyPriorityChargingRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_priority_charging_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyPriorityChargingRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_priority_charging_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyPriorityChargingResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyReport
// ============================================================

#[test]
fn test_valid_notify_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "generatedAt": "2024-01-01T12:00:00Z",
        "seqNo": 0
    });
    assert!(validate_schema_instance(
        "NotifyReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_report_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyReportRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_report_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyReportResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifySettlement
// ============================================================

#[test]
fn test_valid_notify_settlement_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "pspRef": "PSP-REF-001",
        "status": "Settled",
        "settlementAmount": 15.50,
        "settlementTime": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "NotifySettlementRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_settlement_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifySettlementRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_settlement_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifySettlementResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// NotifyWebPaymentStarted
// ============================================================

#[test]
fn test_valid_notify_web_payment_started_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "timeout": 300
    });
    assert!(validate_schema_instance(
        "NotifyWebPaymentStartedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_notify_web_payment_started_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "NotifyWebPaymentStartedRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_notify_web_payment_started_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "NotifyWebPaymentStartedResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// OpenPeriodicEventStream
// ============================================================

#[test]
fn test_valid_open_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "constantStreamData": {
            "id": 1,
            "variableMonitoringId": 42,
            "params": {
                "interval": 60,
                "values": 10
            }
        }
    });
    assert!(validate_schema_instance(
        "OpenPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_open_periodic_event_stream_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "OpenPeriodicEventStreamRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_open_periodic_event_stream_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "OpenPeriodicEventStreamResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// PublishFirmware
// ============================================================

#[test]
fn test_valid_publish_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "location": "https://firmware.example.com/fw.bin",
        "checksum": "abc123def456",
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "PublishFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_publish_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "PublishFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_publish_firmware_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "PublishFirmwareResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// PublishFirmwareStatusNotification
// ============================================================

#[test]
fn test_valid_publish_firmware_status_notification_request(
) -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Published"
    });
    assert!(validate_schema_instance(
        "PublishFirmwareStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_publish_firmware_status_notification_request(
) -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "PublishFirmwareStatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_publish_firmware_status_notification_response(
) -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "PublishFirmwareStatusNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// PullDynamicScheduleUpdate
// ============================================================

#[test]
fn test_valid_pull_dynamic_schedule_update_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "chargingProfileId": 1
    });
    assert!(validate_schema_instance(
        "PullDynamicScheduleUpdateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_pull_dynamic_schedule_update_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "PullDynamicScheduleUpdateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_pull_dynamic_schedule_update_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "PullDynamicScheduleUpdateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ReportChargingProfiles
// ============================================================

#[test]
fn test_valid_report_charging_profiles_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "chargingLimitSource": "EMS",
        "evseId": 1,
        "chargingProfile": [{
            "id": 1,
            "stackLevel": 0,
            "chargingProfilePurpose": "TxProfile",
            "chargingProfileKind": "Absolute",
            "chargingSchedule": [{
                "id": 1,
                "chargingRateUnit": "W",
                "chargingSchedulePeriod": [{"startPeriod": 0}]
            }]
        }]
    });
    assert!(validate_schema_instance(
        "ReportChargingProfilesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_report_charging_profiles_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ReportChargingProfilesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_report_charging_profiles_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ReportChargingProfilesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ReportDERControl
// ============================================================

#[test]
fn test_valid_report_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1
    });
    assert!(validate_schema_instance(
        "ReportDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_report_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ReportDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_report_der_control_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ReportDERControlResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// RequestBatterySwap
// ============================================================

#[test]
fn test_valid_request_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        }
    });
    assert!(validate_schema_instance(
        "RequestBatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_request_battery_swap_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "RequestBatterySwapRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_battery_swap_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "RequestBatterySwapResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// RequestStartTransaction
// ============================================================

#[test]
fn test_valid_request_start_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "remoteStartId": 1,
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        }
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_request_start_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "RequestStartTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_start_transaction_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "RequestStartTransactionResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// RequestStopTransaction
// ============================================================

#[test]
fn test_valid_request_stop_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX123"
    });
    assert!(validate_schema_instance(
        "RequestStopTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_request_stop_transaction_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "RequestStopTransactionRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_request_stop_transaction_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "RequestStopTransactionResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ReservationStatusUpdate
// ============================================================

#[test]
fn test_valid_reservation_status_update_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "reservationId": 1,
        "reservationUpdateStatus": "Expired"
    });
    assert!(validate_schema_instance(
        "ReservationStatusUpdateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_reservation_status_update_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ReservationStatusUpdateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_reservation_status_update_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "ReservationStatusUpdateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// ReserveNow
// ============================================================

#[test]
fn test_valid_reserve_now_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "id": 1,
        "expiryDateTime": "2024-12-31T23:59:59Z",
        "idToken": {
            "idToken": "RFID123",
            "type": "ISO14443"
        }
    });
    assert!(validate_schema_instance(
        "ReserveNowRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_reserve_now_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "ReserveNowRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_reserve_now_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "ReserveNowResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// Reset
// ============================================================

#[test]
fn test_valid_reset_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "type": "Immediate"
    });
    assert!(validate_schema_instance("ResetRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_invalid_reset_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance("ResetRequest.json", instance)?);
    Ok(())
}

#[test]
fn test_valid_reset_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance("ResetResponse.json", instance)?);
    Ok(())
}

// ============================================================
// SecurityEventNotification
// ============================================================

#[test]
fn test_valid_security_event_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "type": "MemoryExhaustion",
        "timestamp": "2024-01-01T12:00:00Z"
    });
    assert!(validate_schema_instance(
        "SecurityEventNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_security_event_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SecurityEventNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_security_event_notification_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "SecurityEventNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SendLocalList
// ============================================================

#[test]
fn test_valid_send_local_list_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "versionNumber": 1,
        "updateType": "Full"
    });
    assert!(validate_schema_instance(
        "SendLocalListRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_send_local_list_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SendLocalListRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_send_local_list_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SendLocalListResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetChargingProfile
// ============================================================

#[test]
fn test_valid_set_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "chargingProfile": {
            "id": 1,
            "stackLevel": 0,
            "chargingProfilePurpose": "TxProfile",
            "chargingProfileKind": "Absolute",
            "chargingSchedule": [{
                "id": 1,
                "chargingRateUnit": "W",
                "chargingSchedulePeriod": [{"startPeriod": 0}]
            }]
        }
    });
    assert!(validate_schema_instance(
        "SetChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_charging_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetChargingProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_charging_profile_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetChargingProfileResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetDefaultTariff
// ============================================================

#[test]
fn test_valid_set_default_tariff_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 0,
        "tariff": {
            "tariffId": "TARIFF1",
            "currency": "EUR"
        }
    });
    assert!(validate_schema_instance(
        "SetDefaultTariffRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_default_tariff_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetDefaultTariffRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_default_tariff_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetDefaultTariffResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetDERControl
// ============================================================

#[test]
fn test_valid_set_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "isDefault": false,
        "controlId": "CTRL001",
        "controlType": "EnterService"
    });
    assert!(validate_schema_instance(
        "SetDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_der_control_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetDERControlRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_der_control_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetDERControlResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetDisplayMessage
// ============================================================

#[test]
fn test_valid_set_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "message": {
            "id": 1,
            "priority": "NormalCycle",
            "message": {
                "format": "UTF8",
                "content": "Hello, World!"
            }
        }
    });
    assert!(validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_display_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetDisplayMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_display_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetDisplayMessageResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetMonitoringBase
// ============================================================

#[test]
fn test_valid_set_monitoring_base_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "monitoringBase": "All"
    });
    assert!(validate_schema_instance(
        "SetMonitoringBaseRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_monitoring_base_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetMonitoringBaseRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_monitoring_base_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetMonitoringBaseResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetMonitoringLevel
// ============================================================

#[test]
fn test_valid_set_monitoring_level_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "severity": 5
    });
    assert!(validate_schema_instance(
        "SetMonitoringLevelRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_monitoring_level_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetMonitoringLevelRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_monitoring_level_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetMonitoringLevelResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetNetworkProfile
// ============================================================

#[test]
fn test_valid_set_network_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "configurationSlot": 1,
        "connectionData": {
            "ocppInterface": "Wired0",
            "ocppTransport": "JSON",
            "messageTimeout": 30,
            "ocppCsmsUrl": "ws://csms.example.com:8080",
            "securityProfile": 0
        }
    });
    assert!(validate_schema_instance(
        "SetNetworkProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_network_profile_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetNetworkProfileRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_network_profile_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SetNetworkProfileResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetVariableMonitoring
// ============================================================

#[test]
fn test_valid_set_variable_monitoring_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setMonitoringData": [{
            "value": 100.0,
            "type": "UpperThreshold",
            "severity": 5,
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "SetVariableMonitoringRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_variable_monitoring_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetVariableMonitoringRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_variable_monitoring_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setMonitoringResult": [{
            "status": "Accepted",
            "type": "UpperThreshold",
            "severity": 5,
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "SetVariableMonitoringResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SetVariables
// ============================================================

#[test]
fn test_valid_set_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setVariableData": [{
            "attributeValue": "Operative",
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_set_variables_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SetVariablesRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_set_variables_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "setVariableResult": [{
            "attributeStatus": "Accepted",
            "component": {"name": "ChargingStation"},
            "variable": {"name": "AvailabilityState"}
        }]
    });
    assert!(validate_schema_instance(
        "SetVariablesResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// SignCertificate
// ============================================================

#[test]
fn test_valid_sign_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "csr": "-----BEGIN CERTIFICATE REQUEST-----\nMIIBIjANBgkq\n-----END CERTIFICATE REQUEST-----"
    });
    assert!(validate_schema_instance(
        "SignCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_sign_certificate_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "SignCertificateRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_sign_certificate_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "SignCertificateResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// StatusNotification
// ============================================================

#[test]
fn test_valid_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "timestamp": "2024-01-01T12:00:00Z",
        "connectorStatus": "Available",
        "evseId": 1,
        "connectorId": 1
    });
    assert!(validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_status_notification_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "StatusNotificationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_status_notification_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "StatusNotificationResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// TransactionEvent
// ============================================================

#[test]
fn test_valid_transaction_event_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "eventType": "Started",
        "timestamp": "2024-01-01T12:00:00Z",
        "triggerReason": "Authorized",
        "seqNo": 0,
        "transactionInfo": {
            "transactionId": "TX123"
        }
    });
    assert!(validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_transaction_event_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "TransactionEventRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_transaction_event_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(validate_schema_instance(
        "TransactionEventResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// TriggerMessage
// ============================================================

#[test]
fn test_valid_trigger_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestedMessage": "Heartbeat"
    });
    assert!(validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_trigger_message_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "TriggerMessageRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_trigger_message_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "TriggerMessageResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// UnlockConnector
// ============================================================

#[test]
fn test_valid_unlock_connector_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "evseId": 1,
        "connectorId": 1
    });
    assert!(validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_unlock_connector_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "UnlockConnectorRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_unlock_connector_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Unlocked"
    });
    assert!(validate_schema_instance(
        "UnlockConnectorResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// UnpublishFirmware
// ============================================================

#[test]
fn test_valid_unpublish_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "checksum": "abc123def456"
    });
    assert!(validate_schema_instance(
        "UnpublishFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_unpublish_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "UnpublishFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_unpublish_firmware_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Unpublished"
    });
    assert!(validate_schema_instance(
        "UnpublishFirmwareResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// UpdateDynamicSchedule
// ============================================================

#[test]
fn test_valid_update_dynamic_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "chargingProfileId": 1,
        "scheduleUpdate": {}
    });
    assert!(validate_schema_instance(
        "UpdateDynamicScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_update_dynamic_schedule_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "UpdateDynamicScheduleRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_update_dynamic_schedule_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "UpdateDynamicScheduleResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// UpdateFirmware
// ============================================================

#[test]
fn test_valid_update_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "requestId": 1,
        "firmware": {
            "location": "https://firmware.example.com/fw.bin",
            "retrieveDateTime": "2024-01-01T12:00:00Z"
        }
    });
    assert!(validate_schema_instance(
        "UpdateFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_update_firmware_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "UpdateFirmwareRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_update_firmware_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "UpdateFirmwareResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// UsePriorityCharging
// ============================================================

#[test]
fn test_valid_use_priority_charging_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "transactionId": "TX123",
        "activate": true
    });
    assert!(validate_schema_instance(
        "UsePriorityChargingRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_use_priority_charging_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "UsePriorityChargingRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_use_priority_charging_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "UsePriorityChargingResponse.json",
        instance
    )?);
    Ok(())
}

// ============================================================
// VatNumberValidation
// ============================================================

#[test]
fn test_valid_vat_number_validation_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "vatNumber": "DE123456789"
    });
    assert!(validate_schema_instance(
        "VatNumberValidationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_invalid_vat_number_validation_request() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({});
    assert!(!validate_schema_instance(
        "VatNumberValidationRequest.json",
        instance
    )?);
    Ok(())
}

#[test]
fn test_valid_vat_number_validation_response() -> Result<(), Box<dyn std::error::Error>> {
    let instance = serde_json::json!({
        "vatNumber": "DE123456789",
        "status": "Accepted"
    });
    assert!(validate_schema_instance(
        "VatNumberValidationResponse.json",
        instance
    )?);
    Ok(())
}
