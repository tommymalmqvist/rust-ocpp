use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, status_info::StatusInfoType};

/// Status of operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum TariffClearStatusEnumType {
    Accepted,
    Rejected,
    NoTariff,
}

/// Result of clearing a tariff.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClearTariffsResultType {
    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Id of tariff for which _status_ is reported. If no tariffs were found, then this field is absent, and _status_ will be `NoTariff`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 60))]
    pub tariff_id: Option<String>,

    /// Status indicating whether the tariff was cleared.
    pub status: TariffClearStatusEnumType,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearTariffsResultType {
    /// Creates a new `ClearTariffsResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the tariff was cleared
    ///
    /// # Returns
    ///
    /// A new instance of `ClearTariffsResultType` with optional fields set to `None`
    pub fn new(status: TariffClearStatusEnumType) -> Self {
        Self {
            status,
            tariff_id: None,
            status_info: None,
            custom_data: None,
        }
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Id of tariff for which status is reported
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_tariff_id(mut self, tariff_id: String) -> Self {
        self.tariff_id = Some(tariff_id);
        self
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_status_info(mut self, status_info: StatusInfoType) -> Self {
        self.status_info = Some(status_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear tariffs result
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the status.
    ///
    /// # Returns
    ///
    /// The status indicating whether the tariff was cleared
    pub fn status(&self) -> &TariffClearStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Status indicating whether the tariff was cleared
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: TariffClearStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the tariff ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the ID of tariff for which status is reported
    pub fn tariff_id(&self) -> Option<&str> {
        self.tariff_id.as_deref()
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Id of tariff for which status is reported, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: Option<String>) -> &mut Self {
        self.tariff_id = tariff_id;
        self
    }

    /// Gets the status info.
    ///
    /// # Returns
    ///
    /// An optional reference to the element providing more information about the status
    pub fn status_info(&self) -> Option<&StatusInfoType> {
        self.status_info.as_ref()
    }

    /// Sets the status info.
    ///
    /// # Arguments
    ///
    /// * `status_info` - Element providing more information about the status, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status_info(&mut self, status_info: Option<StatusInfoType>) -> &mut Self {
        self.status_info = status_info;
        self
    }

    /// Gets the custom data.
    ///
    /// # Returns
    ///
    /// An optional reference to the custom data
    pub fn custom_data(&self) -> Option<&CustomDataType> {
        self.custom_data.as_ref()
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this clear tariffs result, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, to_string};
    use validator::Validate;

    #[test]
    fn test_new_clear_tariffs_result() {
        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted);

        assert_eq!(result.status(), &TariffClearStatusEnumType::Accepted);
        assert_eq!(result.tariff_id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Rejected)
            .with_tariff_id("tariff-123".to_string())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(result.status(), &TariffClearStatusEnumType::Rejected);
        assert_eq!(result.tariff_id(), Some("tariff-123"));
        assert_eq!(result.status_info().unwrap().reason_code(), "SomeReason");
        assert_eq!(
            result.status_info().unwrap().additional_info(),
            Some("Additional details")
        );
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted);

        result
            .set_status(TariffClearStatusEnumType::NoTariff)
            .set_tariff_id(Some("tariff-456".to_string()))
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.status(), &TariffClearStatusEnumType::NoTariff);
        assert_eq!(result.tariff_id(), Some("tariff-456"));
        assert_eq!(result.status_info().unwrap().reason_code(), "SomeReason");
        assert_eq!(
            result.status_info().unwrap().additional_info(),
            Some("Additional details")
        );
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result
            .set_tariff_id(None)
            .set_status_info(None)
            .set_custom_data(None);

        assert_eq!(result.tariff_id(), None);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_serialization_deserialization() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Rejected)
            .with_tariff_id("tariff-123".to_string())
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = to_string(&result).unwrap();

        // Print the serialized JSON for debugging
        println!("Serialized JSON: {}", serialized);

        // Verify JSON contains expected fields - use lowercase for enum values
        assert!(
            serialized.contains(r#""status":"rejected""#)
                || serialized.contains(r#""status":"Rejected""#)
        );
        assert!(serialized.contains(r#""tariffId":"tariff-123""#));
        assert!(serialized.contains(r#""reasonCode":"SomeReason""#));
        assert!(serialized.contains(r#""additionalInfo":"Additional details""#));
        assert!(serialized.contains(r#""vendorId":"VendorX""#));

        // Deserialize back
        let deserialized: ClearTariffsResultType = from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.status(), result.status());
        assert_eq!(deserialized.tariff_id(), result.tariff_id());
        assert_eq!(
            deserialized.status_info().unwrap().reason_code(),
            status_info.reason_code()
        );
        assert_eq!(
            deserialized.status_info().unwrap().additional_info(),
            status_info.additional_info()
        );
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            custom_data.vendor_id()
        );
    }

    #[test]
    fn test_validation() {
        // Valid result
        let valid_result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_tariff_id("tariff-123".to_string());
        assert!(
            valid_result.validate().is_ok(),
            "Valid result should pass validation"
        );

        // Test tariff_id validation (too long)
        let mut invalid_result = valid_result.clone();
        invalid_result.tariff_id = Some("a".repeat(61)); // Exceeds max length of 60
        assert!(
            invalid_result.validate().is_err(),
            "Result with too long tariff_id should fail validation"
        );

        // Test status_info validation (too long reason_code)
        let mut invalid_result = valid_result.clone();
        let invalid_status_info = StatusInfoType {
            reason_code: "a".repeat(21), // Exceeds max length of 20
            additional_info: None,
            custom_data: None,
        };
        invalid_result.status_info = Some(invalid_status_info);
        assert!(
            invalid_result.validate().is_err(),
            "Result with invalid status_info should fail validation"
        );

        // Test status_info validation (too long additional_info)
        let mut invalid_result = valid_result.clone();
        let invalid_status_info = StatusInfoType {
            reason_code: "ValidReason".to_string(),
            additional_info: Some("a".repeat(1025)), // Exceeds max length of 1024
            custom_data: None,
        };
        invalid_result.status_info = Some(invalid_status_info);
        assert!(
            invalid_result.validate().is_err(),
            "Result with invalid status_info additional_info should fail validation"
        );
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty tariff_id
        let empty_tariff_id_result =
            ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
                .with_tariff_id("".to_string());
        assert!(
            empty_tariff_id_result.validate().is_ok(),
            "Result with empty tariff_id should pass validation"
        );

        // Test with maximum length tariff_id
        let max_tariff_id_result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_tariff_id("a".repeat(60));
        assert!(
            max_tariff_id_result.validate().is_ok(),
            "Result with maximum length tariff_id should pass validation"
        );

        // Test with empty strings in status_info
        let status_info = StatusInfoType::new("".to_string()).with_additional_info("".to_string());
        let empty_strings_result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_status_info(status_info);
        assert!(
            empty_strings_result.validate().is_ok(),
            "Result with empty strings in status_info should pass validation"
        );

        // Test with maximum length strings in status_info
        let status_info =
            StatusInfoType::new("a".repeat(20)).with_additional_info("a".repeat(1024));
        let max_strings_result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_status_info(status_info);
        assert!(
            max_strings_result.validate().is_ok(),
            "Result with maximum length strings in status_info should pass validation"
        );
    }

    #[test]
    fn test_all_status_enum_values() {
        // Test with each enum value
        let statuses = vec![
            TariffClearStatusEnumType::Accepted,
            TariffClearStatusEnumType::Rejected,
            TariffClearStatusEnumType::NoTariff,
        ];

        for status in statuses {
            let result = ClearTariffsResultType::new(status.clone());

            assert_eq!(
                result.status(),
                &status,
                "Result should have the correct status"
            );

            // Serialize and deserialize
            let serialized = to_string(&result).unwrap();
            println!("Serialized JSON for status {:?}: {}", status, serialized);
            let deserialized: ClearTariffsResultType = from_str(&serialized).unwrap();

            assert_eq!(
                deserialized.status(),
                &status,
                "Deserialized result should have the correct status"
            );
        }
    }

    #[test]
    fn test_complex_scenario() {
        // Create a status_info with custom_data
        let status_info_custom_data = CustomDataType::new("StatusInfoVendor".to_string());
        let status_info = StatusInfoType::new("ComplexReason".to_string())
            .with_additional_info("Complex scenario details".to_string())
            .with_custom_data(status_info_custom_data);

        // Create a result with custom_data
        let result_custom_data = CustomDataType::new("ResultVendor".to_string());
        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Rejected)
            .with_tariff_id("complex-tariff-123".to_string())
            .with_status_info(status_info.clone())
            .with_custom_data(result_custom_data.clone());

        // Validate the complex object
        assert!(
            result.validate().is_ok(),
            "Complex result should pass validation"
        );

        // Serialize and deserialize
        let serialized = to_string(&result).unwrap();
        let deserialized: ClearTariffsResultType = from_str(&serialized).unwrap();

        // Verify nested custom data is preserved
        assert_eq!(
            deserialized
                .status_info()
                .unwrap()
                .custom_data()
                .unwrap()
                .vendor_id(),
            "StatusInfoVendor"
        );
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            "ResultVendor"
        );
    }

    #[test]
    fn test_json_field_names() {
        let status_info = StatusInfoType::new("SomeReason".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_tariff_id("tariff-123".to_string())
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = to_string(&result).unwrap();

        // Print the serialized JSON for debugging
        println!("Serialized JSON for field names test: {}", serialized);

        // Check that field names use camelCase as specified in #[serde(rename_all = "camelCase")]
        assert!(serialized.contains(r#""status":"#));
        assert!(serialized.contains(r#""tariffId":"#));
        assert!(serialized.contains(r#""statusInfo":"#));
        assert!(serialized.contains(r#""customData":"#));
        assert!(serialized.contains(r#""reasonCode":"#));
        assert!(serialized.contains(r#""vendorId":"#));

        // Ensure no snake_case field names are present
        assert!(!serialized.contains(r#""tariff_id":"#));
        assert!(!serialized.contains(r#""status_info":"#));
        assert!(!serialized.contains(r#""custom_data":"#));
        assert!(!serialized.contains(r#""reason_code":"#));
        assert!(!serialized.contains(r#""vendor_id":"#));
    }

    #[test]
    fn test_no_tariff_scenario() {
        // When status is NoTariff, tariff_id should be None
        let result = ClearTariffsResultType::new(TariffClearStatusEnumType::NoTariff);

        assert_eq!(result.status(), &TariffClearStatusEnumType::NoTariff);
        assert_eq!(result.tariff_id(), None);

        // Serialize and deserialize
        let serialized = to_string(&result).unwrap();
        let deserialized: ClearTariffsResultType = from_str(&serialized).unwrap();

        assert_eq!(deserialized.status(), &TariffClearStatusEnumType::NoTariff);
        assert_eq!(deserialized.tariff_id(), None);

        // Verify that tariff_id is not in the JSON when it's None
        assert!(!serialized.contains("tariffId"));
    }

    #[test]
    fn test_partial_fields() {
        // Test with only status and tariff_id
        let result_with_tariff = ClearTariffsResultType::new(TariffClearStatusEnumType::Accepted)
            .with_tariff_id("tariff-123".to_string());

        assert_eq!(
            result_with_tariff.status(),
            &TariffClearStatusEnumType::Accepted
        );
        assert_eq!(result_with_tariff.tariff_id(), Some("tariff-123"));
        assert_eq!(result_with_tariff.status_info(), None);
        assert_eq!(result_with_tariff.custom_data(), None);

        // Test with only status and status_info
        let status_info = StatusInfoType::new("SomeReason".to_string());
        let result_with_status_info =
            ClearTariffsResultType::new(TariffClearStatusEnumType::Rejected)
                .with_status_info(status_info.clone());

        assert_eq!(
            result_with_status_info.status(),
            &TariffClearStatusEnumType::Rejected
        );
        assert_eq!(result_with_status_info.tariff_id(), None);
        assert_eq!(
            result_with_status_info.status_info().unwrap().reason_code(),
            "SomeReason"
        );
        assert_eq!(result_with_status_info.custom_data(), None);

        // Validate all partial objects
        assert!(
            result_with_tariff.validate().is_ok(),
            "Result with only tariff_id should pass validation"
        );
        assert!(
            result_with_status_info.validate().is_ok(),
            "Result with only status_info should pass validation"
        );
    }
}
