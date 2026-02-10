use crate::v2_1::datatypes::custom_data::CustomDataType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// A generic address format.
///
/// This type represents a physical address with standard address fields
/// such as name, street address, city, postal code, and country.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddressType {
    /// Name of person/company
    #[validate(length(max = 50))]
    pub name: String,

    /// Address line 1
    ///
    /// Primary street address, building number, etc.
    #[validate(length(max = 100))]
    pub address1: String,

    /// Address line 2
    ///
    /// Additional address information like apartment number, suite, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 100))]
    pub address2: Option<String>,

    /// City
    ///
    /// Name of the city or locality
    #[validate(length(max = 100))]
    pub city: String,

    /// Postal code
    ///
    /// ZIP or postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 20))]
    pub postal_code: Option<String>,

    /// Country name
    ///
    /// Name of the country
    #[validate(length(max = 50))]
    pub country: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl AddressType {
    /// Creates a new `AddressType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of person/company
    /// * `address1` - Primary street address
    /// * `city` - Name of the city or locality
    /// * `country` - Name of the country
    ///
    /// # Returns
    ///
    /// A new instance of `AddressType` with optional fields set to `None`
    pub fn new(name: String, address1: String, city: String, country: String) -> Self {
        Self {
            name,
            address1,
            address2: None,
            city,
            postal_code: None,
            country,
            custom_data: None,
        }
    }

    /// Sets the address line 2.
    ///
    /// # Arguments
    ///
    /// * `address2` - Additional address information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_address2(mut self, address2: String) -> Self {
        self.address2 = Some(address2);
        self
    }

    /// Sets the postal code.
    ///
    /// # Arguments
    ///
    /// * `postal_code` - ZIP or postal code
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_postal_code(mut self, postal_code: String) -> Self {
        self.postal_code = Some(postal_code);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this address
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the name.
    ///
    /// # Returns
    ///
    /// The name of person/company as a string
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of person/company
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    /// Gets the address line 1.
    ///
    /// # Returns
    ///
    /// The primary street address as a string
    pub fn address1(&self) -> &str {
        &self.address1
    }

    /// Sets the address line 1.
    ///
    /// # Arguments
    ///
    /// * `address1` - Primary street address
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_address1(&mut self, address1: String) -> &mut Self {
        self.address1 = address1;
        self
    }

    /// Gets the address line 2.
    ///
    /// # Returns
    ///
    /// An optional reference to the additional address information
    pub fn address2(&self) -> Option<&String> {
        self.address2.as_ref()
    }

    /// Sets the address line 2.
    ///
    /// # Arguments
    ///
    /// * `address2` - Additional address information, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_address2(&mut self, address2: Option<String>) -> &mut Self {
        self.address2 = address2;
        self
    }

    /// Gets the city.
    ///
    /// # Returns
    ///
    /// The name of the city or locality as a string
    pub fn city(&self) -> &str {
        &self.city
    }

    /// Sets the city.
    ///
    /// # Arguments
    ///
    /// * `city` - Name of the city or locality
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_city(&mut self, city: String) -> &mut Self {
        self.city = city;
        self
    }

    /// Gets the postal code.
    ///
    /// # Returns
    ///
    /// An optional reference to the ZIP or postal code
    pub fn postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }

    /// Sets the postal code.
    ///
    /// # Arguments
    ///
    /// * `postal_code` - ZIP or postal code, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_postal_code(&mut self, postal_code: Option<String>) -> &mut Self {
        self.postal_code = postal_code;
        self
    }

    /// Gets the country.
    ///
    /// # Returns
    ///
    /// The name of the country as a string
    pub fn country(&self) -> &str {
        &self.country
    }

    /// Sets the country.
    ///
    /// # Arguments
    ///
    /// * `country` - Name of the country
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = country;
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
    /// * `custom_data` - Custom data for this address, or None to clear
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

    #[test]
    fn test_new_address() {
        let address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );

        assert_eq!(address.name(), "John Doe");
        assert_eq!(address.address1(), "123 Main St");
        assert_eq!(address.address2(), None);
        assert_eq!(address.city(), "Anytown");
        assert_eq!(address.postal_code(), None);
        assert_eq!(address.country(), "USA");
        assert_eq!(address.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        )
        .with_address2("Apt 4B".to_string())
        .with_postal_code("12345".to_string())
        .with_custom_data(custom_data.clone());

        assert_eq!(address.name(), "John Doe");
        assert_eq!(address.address1(), "123 Main St");
        assert_eq!(address.address2(), Some(&"Apt 4B".to_string()));
        assert_eq!(address.city(), "Anytown");
        assert_eq!(address.postal_code(), Some(&"12345".to_string()));
        assert_eq!(address.country(), "USA");
        assert_eq!(address.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );

        address
            .set_name("Jane Smith".to_string())
            .set_address1("456 Oak Ave".to_string())
            .set_address2(Some("Suite 789".to_string()))
            .set_city("Othertown".to_string())
            .set_postal_code(Some("67890".to_string()))
            .set_country("Canada".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(address.name(), "Jane Smith");
        assert_eq!(address.address1(), "456 Oak Ave");
        assert_eq!(address.address2(), Some(&"Suite 789".to_string()));
        assert_eq!(address.city(), "Othertown");
        assert_eq!(address.postal_code(), Some(&"67890".to_string()));
        assert_eq!(address.country(), "Canada");
        assert_eq!(address.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        address
            .set_address2(None)
            .set_postal_code(None)
            .set_custom_data(None);

        assert_eq!(address.address2(), None);
        assert_eq!(address.postal_code(), None);
        assert_eq!(address.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Valid address
        let valid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );
        assert!(valid_address.validate().is_ok());

        // Test name length validation
        let long_name = "A".repeat(51); // 51 characters, exceeds max of 50
        let invalid_address = AddressType::new(
            long_name,
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );
        assert!(invalid_address.validate().is_err());

        // Test address1 length validation
        let long_address1 = "A".repeat(101); // 101 characters, exceeds max of 100
        let invalid_address = AddressType::new(
            "John Doe".to_string(),
            long_address1,
            "Anytown".to_string(),
            "USA".to_string(),
        );
        assert!(invalid_address.validate().is_err());

        // Test address2 length validation
        let long_address2 = "A".repeat(101); // 101 characters, exceeds max of 100
        let mut invalid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );
        invalid_address.set_address2(Some(long_address2));
        assert!(invalid_address.validate().is_err());

        // Test city length validation
        let long_city = "A".repeat(101); // 101 characters, exceeds max of 100
        let invalid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            long_city,
            "USA".to_string(),
        );
        assert!(invalid_address.validate().is_err());

        // Test postal_code length validation
        let long_postal_code = "A".repeat(21); // 21 characters, exceeds max of 20
        let mut invalid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );
        invalid_address.set_postal_code(Some(long_postal_code));
        assert!(invalid_address.validate().is_err());

        // Test country length validation
        let long_country = "A".repeat(51); // 51 characters, exceeds max of 50
        let invalid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            long_country,
        );
        assert!(invalid_address.validate().is_err());

        // Test custom_data nested validation
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255

        let mut invalid_address = AddressType::new(
            "John Doe".to_string(),
            "123 Main St".to_string(),
            "Anytown".to_string(),
            "USA".to_string(),
        );
        invalid_address.set_custom_data(Some(invalid_custom_data));

        assert!(
            invalid_address.validate().is_err(),
            "Address with invalid custom_data should fail validation"
        );
    }
}
