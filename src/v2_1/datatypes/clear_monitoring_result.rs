use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use super::status_info::StatusInfoType;
use crate::v2_1::enumerations::ClearMonitoringStatusEnumType;

/// Result of the clear request for this monitor, identified by its Id.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ClearMonitoringResultType {
    /// Required. Result of the clear request for this monitor, identified by its Id.
    pub status: ClearMonitoringStatusEnumType,

    /// Required. Id of the monitor of which a clear was requested.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Element providing more information about the status.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub status_info: Option<StatusInfoType>,

    /// Custom data specific to this class.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ClearMonitoringResultType {
    /// Creates a new `ClearMonitoringResultType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `status` - Result of the clear request for this monitor
    /// * `id` - Id of the monitor of which a clear was requested
    ///
    /// # Returns
    ///
    /// A new instance of `ClearMonitoringResultType` with optional fields set to `None`
    pub fn new(status: ClearMonitoringStatusEnumType, id: i32) -> Self {
        Self {
            status,
            id,
            custom_data: None,
            status_info: None,
        }
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
    /// * `custom_data` - Custom data for this clear monitoring result
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
    /// The result of the clear request for this monitor
    pub fn status(&self) -> &ClearMonitoringStatusEnumType {
        &self.status
    }

    /// Sets the status.
    ///
    /// # Arguments
    ///
    /// * `status` - Result of the clear request for this monitor
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: ClearMonitoringStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the monitor ID.
    ///
    /// # Returns
    ///
    /// The ID of the monitor of which a clear was requested
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the monitor ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the monitor of which a clear was requested
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
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
    /// * `custom_data` - Custom data for this clear monitoring result, or None to clear
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
    fn test_new_clear_monitoring_result() {
        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42);

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::Accepted);
        assert_eq!(result.id(), 42);
        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Rejected, 42)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::Rejected);
        assert_eq!(result.id(), 42);
        assert_eq!(result.status_info().unwrap().reason_code, "SomeReason");
        assert_eq!(
            result.status_info().unwrap().additional_info,
            Some("Additional details".to_string())
        );
        assert_eq!(result.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42);

        result
            .set_status(ClearMonitoringStatusEnumType::NotFound)
            .set_id(43)
            .set_status_info(Some(status_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(result.status(), &ClearMonitoringStatusEnumType::NotFound);
        assert_eq!(result.id(), 43);
        assert_eq!(result.status_info().unwrap().reason_code, "SomeReason");
        assert_eq!(
            result.status_info().unwrap().additional_info,
            Some("Additional details".to_string())
        );
        assert_eq!(result.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        result.set_status_info(None).set_custom_data(None);

        assert_eq!(result.status_info(), None);
        assert_eq!(result.custom_data(), None);
    }

    #[test]
    fn test_serialization_deserialization() {
        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string());

        let custom_data = CustomDataType::new("VendorX".to_string());

        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Rejected, 42)
            .with_status_info(status_info.clone())
            .with_custom_data(custom_data.clone());

        // Serialize to JSON
        let serialized = to_string(&result).unwrap();

        // Verify JSON contains expected fields
        assert!(serialized.contains(r#""status":"Rejected""#));
        assert!(serialized.contains(r#""id":42"#));
        assert!(serialized.contains(r#""reasonCode":"SomeReason""#));
        assert!(serialized.contains(r#""additionalInfo":"Additional details""#));
        assert!(serialized.contains(r#""vendorId":"VendorX""#));

        // Deserialize back
        let deserialized: ClearMonitoringResultType = from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(deserialized.status(), result.status());
        assert_eq!(deserialized.id(), result.id());
        assert_eq!(
            deserialized.status_info().unwrap().reason_code,
            status_info.reason_code
        );
        assert_eq!(
            deserialized.status_info().unwrap().additional_info,
            status_info.additional_info
        );
        assert_eq!(
            deserialized.custom_data().unwrap().vendor_id(),
            custom_data.vendor_id()
        );
    }

    #[test]
    fn test_validation() {
        // Valid result
        let valid_result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42);
        assert!(
            valid_result.validate().is_ok(),
            "Valid result should pass validation"
        );

        // Test id validation (negative value)
        let mut invalid_result = valid_result.clone();
        invalid_result.id = -1; // Invalid: must be >= 0
        assert!(
            invalid_result.validate().is_err(),
            "Result with negative id should fail validation"
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
        // Test with zero id (should be valid)
        let zero_id_result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 0);
        assert!(
            zero_id_result.validate().is_ok(),
            "Result with zero id should pass validation"
        );

        // Test with maximum integer id
        let max_id_result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, i32::MAX);
        assert!(
            max_id_result.validate().is_ok(),
            "Result with maximum integer id should pass validation"
        );

        // Test with empty strings in status_info
        let status_info = StatusInfoType::new("".to_string()).with_additional_info("".to_string());
        let empty_strings_result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42)
                .with_status_info(status_info);
        assert!(
            empty_strings_result.validate().is_ok(),
            "Result with empty strings in status_info should pass validation"
        );

        // Test with maximum length strings in status_info
        let status_info =
            StatusInfoType::new("a".repeat(20)).with_additional_info("a".repeat(1024));
        let max_strings_result =
            ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42)
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
            ClearMonitoringStatusEnumType::Accepted,
            ClearMonitoringStatusEnumType::Rejected,
            ClearMonitoringStatusEnumType::NotFound,
        ];

        for status in statuses {
            let result = ClearMonitoringResultType::new(status.clone(), 42);

            assert_eq!(
                result.status(),
                &status,
                "Result should have the correct status"
            );

            // Serialize and deserialize
            let serialized = to_string(&result).unwrap();
            let deserialized: ClearMonitoringResultType = from_str(&serialized).unwrap();

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
        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Rejected, 42)
            .with_status_info(status_info.clone())
            .with_custom_data(result_custom_data.clone());

        // Validate the complex object
        assert!(
            result.validate().is_ok(),
            "Complex result should pass validation"
        );

        // Serialize and deserialize
        let serialized = to_string(&result).unwrap();
        let deserialized: ClearMonitoringResultType = from_str(&serialized).unwrap();

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

        let result = ClearMonitoringResultType::new(ClearMonitoringStatusEnumType::Accepted, 42)
            .with_status_info(status_info)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = to_string(&result).unwrap();

        // Check that field names use camelCase as specified in #[serde(rename_all = "camelCase")]
        assert!(serialized.contains(r#""status":"#));
        assert!(serialized.contains(r#""id":"#));
        assert!(serialized.contains(r#""statusInfo":"#));
        assert!(serialized.contains(r#""customData":"#));
        assert!(serialized.contains(r#""reasonCode":"#));
        assert!(serialized.contains(r#""vendorId":"#));

        // Ensure no snake_case field names are present
        assert!(!serialized.contains(r#""status_info":"#));
        assert!(!serialized.contains(r#""custom_data":"#));
        assert!(!serialized.contains(r#""reason_code":"#));
        assert!(!serialized.contains(r#""vendor_id":"#));
    }
}
