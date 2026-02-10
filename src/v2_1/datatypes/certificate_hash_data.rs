use crate::v2_1::datatypes::custom_data::CustomDataType;
use crate::v2_1::enumerations::HashAlgorithmEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Certificate hash data for validating certificates through OCSP.
///
/// This type contains the necessary hash data to validate a certificate using
/// the Online Certificate Status Protocol (OCSP).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataType {
    /// Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,

    /// The hash of the issuer's distinguished name (DN), that must be calculated over the DER
    /// encoding of the issuer's name field in the certificate being checked.
    #[validate(length(max = 128))]
    pub issuer_name_hash: String,

    /// The hash of the DER encoded public key: the value (excluding tag and length) of the subject
    /// public key field in the issuer's certificate.
    #[validate(length(max = 128))]
    pub issuer_key_hash: String,

    /// The string representation of the hexadecimal value of the serial number without the
    /// prefix "0x" and without leading zeroes.
    #[validate(length(max = 40))]
    pub serial_number: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateHashDataType {
    /// Creates a new `CertificateHashDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `hash_algorithm` - Algorithm used for the hashes
    /// * `issuer_name_hash` - Hash of the issuer's distinguished name
    /// * `issuer_key_hash` - Hash of the DER encoded public key
    /// * `serial_number` - Hexadecimal value of the serial number
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateHashDataType` with optional fields set to `None`
    pub fn new(
        hash_algorithm: HashAlgorithmEnumType,
        issuer_name_hash: String,
        issuer_key_hash: String,
        serial_number: String,
    ) -> Self {
        Self {
            hash_algorithm,
            issuer_name_hash,
            issuer_key_hash,
            serial_number,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate hash data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the hash algorithm.
    ///
    /// # Returns
    ///
    /// The hash algorithm used for the hashes
    pub fn hash_algorithm(&self) -> &HashAlgorithmEnumType {
        &self.hash_algorithm
    }

    /// Sets the hash algorithm.
    ///
    /// # Arguments
    ///
    /// * `hash_algorithm` - Algorithm used for the hashes
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_hash_algorithm(&mut self, hash_algorithm: HashAlgorithmEnumType) -> &mut Self {
        self.hash_algorithm = hash_algorithm;
        self
    }

    /// Gets the issuer name hash.
    ///
    /// # Returns
    ///
    /// The hash of the issuer's distinguished name
    pub fn issuer_name_hash(&self) -> &str {
        &self.issuer_name_hash
    }

    /// Sets the issuer name hash.
    ///
    /// # Arguments
    ///
    /// * `issuer_name_hash` - Hash of the issuer's distinguished name
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_issuer_name_hash(&mut self, issuer_name_hash: String) -> &mut Self {
        self.issuer_name_hash = issuer_name_hash;
        self
    }

    /// Gets the issuer key hash.
    ///
    /// # Returns
    ///
    /// The hash of the DER encoded public key
    pub fn issuer_key_hash(&self) -> &str {
        &self.issuer_key_hash
    }

    /// Sets the issuer key hash.
    ///
    /// # Arguments
    ///
    /// * `issuer_key_hash` - Hash of the DER encoded public key
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_issuer_key_hash(&mut self, issuer_key_hash: String) -> &mut Self {
        self.issuer_key_hash = issuer_key_hash;
        self
    }

    /// Gets the serial number.
    ///
    /// # Returns
    ///
    /// The hexadecimal value of the serial number
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - Hexadecimal value of the serial number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_serial_number(&mut self, serial_number: String) -> &mut Self {
        self.serial_number = serial_number;
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
    /// * `custom_data` - Custom data for this certificate hash data, or None to clear
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
    use serde_json::json;

    #[test]
    fn test_new_certificate_hash_data() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        assert_eq!(
            cert_hash_data.hash_algorithm(),
            &HashAlgorithmEnumType::SHA256
        );
        assert_eq!(cert_hash_data.issuer_name_hash(), "a1b2c3d4e5f6");
        assert_eq!(cert_hash_data.issuer_key_hash(), "f6e5d4c3b2a1");
        assert_eq!(cert_hash_data.serial_number(), "1234567890abcdef");
        assert_eq!(cert_hash_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(
            cert_hash_data.hash_algorithm(),
            &HashAlgorithmEnumType::SHA256
        );
        assert_eq!(cert_hash_data.issuer_name_hash(), "a1b2c3d4e5f6");
        assert_eq!(cert_hash_data.issuer_key_hash(), "f6e5d4c3b2a1");
        assert_eq!(cert_hash_data.serial_number(), "1234567890abcdef");
        assert_eq!(cert_hash_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        cert_hash_data
            .set_hash_algorithm(HashAlgorithmEnumType::SHA512)
            .set_issuer_name_hash("newnamehash".to_string())
            .set_issuer_key_hash("newkeyhash".to_string())
            .set_serial_number("newserial".to_string())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            cert_hash_data.hash_algorithm(),
            &HashAlgorithmEnumType::SHA512
        );
        assert_eq!(cert_hash_data.issuer_name_hash(), "newnamehash");
        assert_eq!(cert_hash_data.issuer_key_hash(), "newkeyhash");
        assert_eq!(cert_hash_data.serial_number(), "newserial");
        assert_eq!(cert_hash_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cert_hash_data.set_custom_data(None);
        assert_eq!(cert_hash_data.custom_data(), None);
    }

    #[test]
    fn test_length_validation() {
        // Valid certificate hash data
        let valid_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );
        assert!(valid_data.validate().is_ok());

        // Test issuer_name_hash exceeds max length (128 chars)
        let mut invalid_data = valid_data.clone();
        invalid_data.set_issuer_name_hash("a".repeat(129));
        assert!(invalid_data.validate().is_err());

        // Test issuer_key_hash exceeds max length (128 chars)
        let mut invalid_data = valid_data.clone();
        invalid_data.set_issuer_key_hash("b".repeat(129));
        assert!(invalid_data.validate().is_err());

        // Test serial_number exceeds max length (40 chars)
        let mut invalid_data = valid_data.clone();
        invalid_data.set_serial_number("c".repeat(41));
        assert!(invalid_data.validate().is_err());
    }

    #[test]
    fn test_validation_with_custom_data() {
        // Create valid custom data
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        )
        .with_custom_data(custom_data);

        // Validate certificate hash data with custom data
        assert!(cert_hash_data.validate().is_ok());
    }

    #[test]
    fn test_invalid_custom_data_validation() {
        // Create invalid custom data (vendor_id too long)
        let too_long_vendor_id = "V".repeat(256); // Exceeds 255 character limit
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        )
        .with_custom_data(invalid_custom_data);

        // Validation should fail because custom data is invalid
        assert!(cert_hash_data.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        // Serialize to JSON
        let serialized = serde_json::to_string(&cert_hash_data).unwrap();

        // Deserialize back
        let deserialized: CertificateHashDataType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(cert_hash_data, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_edge_case_validations() {
        // Test with maximum allowed lengths
        let max_length_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "a".repeat(128),
            "b".repeat(128),
            "c".repeat(40),
        );
        assert!(max_length_data.validate().is_ok());

        // Test with empty strings (which may be invalid in real use)
        let empty_strings_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "".to_string(),
            "".to_string(),
            "".to_string(),
        );
        // Empty strings should pass validation as there's no explicit min length
        assert!(empty_strings_data.validate().is_ok());
    }
}
