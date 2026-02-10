use chrono::DateTime;
use chrono::Utc;

use super::id_token_type::IdTokenType;
use super::message_content_type::MessageContentType;
use crate::v2_0_1::enumerations::authorization_status_enum_type::AuthorizationStatusEnumType;
use crate::v2_0_1::helpers::datetime_rfc3339;

/// Contains status information about an identifier. It is advised to not stop charging for a token that expires during charging, as ExpiryDate is only used for caching purposes. If ExpiryDate is not given, the status has no end date.
/// IdTokenInfoType is used by: Common:AuthorizationData , AuthorizeResponse , TransactionEventResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenInfoType {
    pub status: AuthorizationStatusEnumType,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "datetime_rfc3339::option"
    )]
    pub cache_expiry_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse_id: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id_token: Option<IdTokenType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<MessageContentType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;
    use chrono::TimeZone;
    use chrono::Utc;
    use serde_json::{json, Value};

    #[test]
    fn test_id_token_info_serialize_with_none_cache_expiry() {
        let token_info = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: Some(1),
            language1: Some("en".to_string()),
            evse_id: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
        };

        let json = serde_json::to_string_pretty(&token_info).unwrap();
        println!("Serialized with None cache_expiry_date_time:\n{}", json);

        let v: Value = serde_json::from_str(&json).unwrap();

        // Verify cacheExpiryDateTime is NOT in the JSON
        assert!(
            v.get("cacheExpiryDateTime").is_none(),
            "cacheExpiryDateTime should not be in JSON when None"
        );
        assert_eq!(v["status"], "Accepted");
        assert_eq!(v["chargingPriority"], 1);
    }

    #[test]
    fn test_id_token_info_serialize_with_some_cache_expiry() {
        let dt = Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap();
        let token_info = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: Some(dt),
            charging_priority: None,
            language1: None,
            evse_id: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
        };

        let json = serde_json::to_string_pretty(&token_info).unwrap();
        println!("Serialized with Some cache_expiry_date_time:\n{}", json);

        let v: Value = serde_json::from_str(&json).unwrap();

        // Verify cacheExpiryDateTime IS in the JSON
        assert!(
            v.get("cacheExpiryDateTime").is_some(),
            "cacheExpiryDateTime should be in JSON when Some"
        );
        let ts_str = v["cacheExpiryDateTime"].as_str().unwrap();
        println!("Timestamp value: {}", ts_str);
        assert!(ts_str.starts_with("2025-12-31T23:59:59"));
        assert!(ts_str.ends_with('Z'));
    }

    #[test]
    fn test_id_token_info_deserialize_with_none_cache_expiry() {
        let json_str = r#"{
            "status": "Accepted",
            "chargingPriority": 1
        }"#;

        let token_info: IdTokenInfoType = serde_json::from_str(json_str).unwrap();
        println!(
            "Deserialized token_info with missing cacheExpiryDateTime: {:?}",
            token_info
        );

        assert_eq!(token_info.status, AuthorizationStatusEnumType::Accepted);
        assert_eq!(token_info.cache_expiry_date_time, None);
        assert_eq!(token_info.charging_priority, Some(1));
    }

    #[test]
    fn test_id_token_info_deserialize_with_some_cache_expiry() {
        let json_str = r#"{
            "status": "Accepted",
            "cacheExpiryDateTime": "2025-12-31T23:59:59.000Z"
        }"#;

        let token_info: IdTokenInfoType = serde_json::from_str(json_str).unwrap();
        println!(
            "Deserialized token_info with cacheExpiryDateTime: {:?}",
            token_info
        );

        assert_eq!(token_info.status, AuthorizationStatusEnumType::Accepted);
        assert!(token_info.cache_expiry_date_time.is_some());
        let dt = token_info.cache_expiry_date_time.unwrap();
        println!("Parsed datetime: {}", dt);
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 31);
    }

    #[test]
    fn test_id_token_info_round_trip_none() {
        let original = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: None,
            charging_priority: Some(5),
            language1: Some("de".to_string()),
            evse_id: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: IdTokenInfoType = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
        println!("Round-trip test (None) passed!");
    }

    #[test]
    fn test_id_token_info_round_trip_some() {
        let dt = Utc.with_ymd_and_hms(2024, 6, 15, 10, 30, 45).unwrap();
        let original = IdTokenInfoType {
            status: AuthorizationStatusEnumType::Accepted,
            cache_expiry_date_time: Some(dt),
            charging_priority: Some(3),
            language1: Some("fr".to_string()),
            evse_id: None,
            language2: None,
            group_id_token: None,
            personal_message: None,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: IdTokenInfoType = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
        println!("Round-trip test (Some) passed!");
    }
}
