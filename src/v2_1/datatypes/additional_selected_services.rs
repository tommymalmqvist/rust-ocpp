use crate::v2_1::datatypes::{custom_data::CustomDataType, rational_number::RationalNumberType};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Part of ISO 15118-20 price schedule.
///
/// This type represents additional services that can be selected as part of a charging session,
/// including the service name and associated fee.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalSelectedServicesType {
    /// Service fee
    ///
    /// The fee associated with this additional service, represented as a rational number.
    #[validate(nested)]
    pub service_fee: RationalNumberType,

    /// Human-readable string to identify this service.
    ///
    /// A descriptive name for the service that can be displayed to users.
    #[validate(length(max = 80))]
    pub service_name: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AdditionalSelectedServicesType {
    /// Creates a new `AdditionalSelectedServicesType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `service_fee` - The fee associated with this additional service
    /// * `service_name` - Human-readable string to identify this service
    ///
    /// # Returns
    ///
    /// A new instance of `AdditionalSelectedServicesType` with optional fields set to `None`
    ///
    /// # Panics
    ///
    /// Panics if `service_name` is longer than 80 characters
    pub fn new(service_fee: RationalNumberType, service_name: String) -> Self {
        assert!(
            service_name.len() <= 80,
            "service_name must not exceed 80 characters"
        );

        Self {
            service_fee,
            service_name,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this additional selected service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Validates this instance according to the OCPP 2.1 specification.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the instance is valid, otherwise an error
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Validate::validate(self)
    }

    /// Gets the service fee.
    ///
    /// # Returns
    ///
    /// A reference to the service fee as a rational number
    pub fn service_fee(&self) -> &RationalNumberType {
        &self.service_fee
    }

    /// Sets the service fee.
    ///
    /// # Arguments
    ///
    /// * `service_fee` - The fee associated with this additional service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_service_fee(&mut self, service_fee: RationalNumberType) -> &mut Self {
        self.service_fee = service_fee;
        self
    }

    /// Gets the service name.
    ///
    /// # Returns
    ///
    /// The service name as a string
    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Sets the service name.
    ///
    /// # Arguments
    ///
    /// * `service_name` - Human-readable string to identify this service
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    ///
    /// # Panics
    ///
    /// Panics if `service_name` is longer than 80 characters
    pub fn set_service_name(&mut self, service_name: String) -> &mut Self {
        assert!(
            service_name.len() <= 80,
            "service_name must not exceed 80 characters"
        );
        self.service_name = service_name;
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
    /// * `custom_data` - Custom data for this additional selected service, or None to clear
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
    use validator::Validate;

    #[test]
    fn test_new_additional_selected_services() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let services = AdditionalSelectedServicesType::new(
            service_fee.clone(),
            "Premium Charging".to_string(),
        );

        assert_eq!(services.service_fee(), &service_fee);
        assert_eq!(services.service_name(), "Premium Charging");
        assert_eq!(services.custom_data(), None);

        // Validation should pass
        assert!(services.validate().is_ok());
    }

    #[test]
    fn test_with_custom_data() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let services = AdditionalSelectedServicesType::new(
            service_fee.clone(),
            "Premium Charging".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(services.service_fee(), &service_fee);
        assert_eq!(services.service_name(), "Premium Charging");
        assert_eq!(services.custom_data(), Some(&custom_data));

        // Validation should pass
        assert!(services.validate().is_ok());
    }

    #[test]
    fn test_setter_methods() {
        let service_fee1 = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let service_fee2 = RationalNumberType {
            exponent: 2,
            value: 2495,
            custom_data: None,
        };

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut services = AdditionalSelectedServicesType::new(
            service_fee1.clone(),
            "Premium Charging".to_string(),
        );

        services
            .set_service_fee(service_fee2.clone())
            .set_service_name("Ultra Premium Charging".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(services.service_fee(), &service_fee2);
        assert_eq!(services.service_name(), "Ultra Premium Charging");
        assert_eq!(services.custom_data(), Some(&custom_data));

        // Validation should pass
        assert!(services.validate().is_ok());

        // Test clearing optional fields
        services.set_custom_data(None);
        assert_eq!(services.custom_data(), None);

        // Validation should still pass
        assert!(services.validate().is_ok());
    }

    #[test]
    #[should_panic(expected = "service_name must not exceed 80 characters")]
    fn test_service_name_length_validation_new() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        // This should panic because service_name is too long
        let _services = AdditionalSelectedServicesType::new(
            service_fee,
            "A".repeat(81), // 81 characters, exceeding the 80 character limit
        );
    }

    #[test]
    #[should_panic(expected = "service_name must not exceed 80 characters")]
    fn test_service_name_length_validation_setter() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let mut services =
            AdditionalSelectedServicesType::new(service_fee, "Premium Charging".to_string());

        // This should panic because service_name is too long
        services.set_service_name("A".repeat(81)); // 81 characters, exceeding the 80 character limit
    }

    #[test]
    fn test_validation_with_validator() {
        let service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let mut services =
            AdditionalSelectedServicesType::new(service_fee, "Premium Charging".to_string());

        // Valid service name
        assert!(services.validate().is_ok());

        // Manually set an invalid service name (bypassing the setter)
        services.service_name = "A".repeat(81);

        // Validation should fail
        assert!(services.validate().is_err());
    }

    #[test]
    fn test_comprehensive_validation() {
        // 1. Create a valid instance
        let valid_service_fee = RationalNumberType {
            exponent: 2,
            value: 1995,
            custom_data: None,
        };

        let valid_custom_data = CustomDataType::new("VendorX".to_string());

        let valid_services = AdditionalSelectedServicesType::new(
            valid_service_fee.clone(),
            "Premium Charging".to_string(),
        )
        .with_custom_data(valid_custom_data.clone());

        // Valid instance should pass validation
        let validation_result = Validate::validate(&valid_services);
        assert!(
            validation_result.is_ok(),
            "Valid services should pass validation"
        );

        // 2. Test invalid service_name (too long)
        let mut invalid_name_services = valid_services.clone();
        // Bypass the setter to avoid the panic
        invalid_name_services.service_name = "A".repeat(81);

        let validation_result = Validate::validate(&invalid_name_services);
        assert!(
            validation_result.is_err(),
            "Services with too long name should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("service_name"),
            "Error should mention service_name: {}",
            error
        );

        // 3. Test invalid custom_data (nested validation failure)
        let mut invalid_services_custom_data = valid_services.clone();
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255
        invalid_services_custom_data.custom_data = Some(invalid_custom_data);

        let validation_result = Validate::validate(&invalid_services_custom_data);
        assert!(
            validation_result.is_err(),
            "Services with invalid custom_data should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }
}
