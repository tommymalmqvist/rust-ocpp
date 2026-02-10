use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::CertificateStatusSourceEnumType;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

/// Validates that each URL in the list does not exceed the maximum length.
///
/// # Arguments
///
/// * `urls` - The list of URLs to validate
///
/// # Returns
///
/// Returns Ok(()) if all URLs are valid, otherwise returns Err
pub fn validate_urls(urls: &[String]) -> Result<(), ValidationError> {
    const MAX_URL_LENGTH: usize = 2000;

    for url in urls {
        if url.len() > MAX_URL_LENGTH {
            return Err(ValidationError::new("url_too_long"));
        }
    }

    Ok(())
}

/// Data necessary to request the revocation status of a certificate.
///
/// This type contains the information needed to request the revocation status
/// of a certificate from a certificate status source like OCSP or CRL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusRequestInfoType {
    /// Certificate hash data needed for validating certificates through OCSP.
    #[validate(nested)]
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// URL(s) of _source_.
    #[validate(length(min = 1, max = 5), custom(function = "validate_urls"))]
    pub urls: Vec<String>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateStatusRequestInfoType {
    /// Creates a new `CertificateStatusRequestInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Certificate hash data for validation
    /// * `source` - Source of the certificate status information
    /// * `urls` - URLs of the certificate status source
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateStatusRequestInfoType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        source: CertificateStatusSourceEnumType,
        urls: Vec<String>,
    ) -> Self {
        Self {
            certificate_hash_data,
            source,
            urls,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate status request
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the certificate hash data.
    ///
    /// # Returns
    ///
    /// A reference to the certificate hash data
    pub fn certificate_hash_data(&self) -> &CertificateHashDataType {
        &self.certificate_hash_data
    }

    /// Sets the certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Certificate hash data for validation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_certificate_hash_data(
        &mut self,
        certificate_hash_data: CertificateHashDataType,
    ) -> &mut Self {
        self.certificate_hash_data = certificate_hash_data;
        self
    }

    /// Gets the source of the certificate status.
    ///
    /// # Returns
    ///
    /// The source of the certificate status information
    pub fn source(&self) -> &CertificateStatusSourceEnumType {
        &self.source
    }

    /// Sets the source of the certificate status.
    ///
    /// # Arguments
    ///
    /// * `source` - Source of the certificate status information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_source(&mut self, source: CertificateStatusSourceEnumType) -> &mut Self {
        self.source = source;
        self
    }

    /// Gets the URLs of the certificate status source.
    ///
    /// # Returns
    ///
    /// A reference to the URLs of the certificate status source
    pub fn urls(&self) -> &Vec<String> {
        &self.urls
    }

    /// Sets the URLs of the certificate status source.
    ///
    /// # Arguments
    ///
    /// * `urls` - URLs of the certificate status source
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_urls(&mut self, urls: Vec<String>) -> &mut Self {
        self.urls = urls;
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
    /// * `custom_data` - Custom data for this certificate status request, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2_1::enumerations::HashAlgorithmEnumType;
    use serde_json::json;

    #[test]
    fn test_new_certificate_status_request_info() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let urls = vec!["https://ocsp.example.com".to_string()];

        let request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls.clone(),
        );

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data);
        assert_eq!(
            request_info.source(),
            &CertificateStatusSourceEnumType::OCSP
        );
        assert_eq!(request_info.urls(), &urls);
        assert_eq!(request_info.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let urls = vec!["https://ocsp.example.com".to_string()];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data);
        assert_eq!(
            request_info.source(),
            &CertificateStatusSourceEnumType::OCSP
        );
        assert_eq!(request_info.urls(), &urls);
        assert_eq!(request_info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let cert_hash_data1 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let cert_hash_data2 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "new_name_hash".to_string(),
            "new_key_hash".to_string(),
            "new_serial".to_string(),
        );

        let urls1 = vec!["https://ocsp.example.com".to_string()];
        let urls2 = vec![
            "https://crl.example.com".to_string(),
            "https://crl2.example.com".to_string(),
        ];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data1.clone(),
            CertificateStatusSourceEnumType::OCSP,
            urls1.clone(),
        );

        request_info
            .set_certificate_hash_data(cert_hash_data2.clone())
            .set_source(CertificateStatusSourceEnumType::CRL)
            .set_urls(urls2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(request_info.certificate_hash_data(), &cert_hash_data2);
        assert_eq!(request_info.source(), &CertificateStatusSourceEnumType::CRL);
        assert_eq!(request_info.urls(), &urls2);
        assert_eq!(request_info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        request_info.set_custom_data(None);
        assert_eq!(request_info.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Create valid certificate hash data
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        // Valid request info
        let valid_request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            vec!["https://ocsp.example.com".to_string()],
        );

        // Validation should pass
        assert!(valid_request_info.validate().is_ok());

        // Test with empty urls array (violates min length)
        let mut invalid_request_info = valid_request_info.clone();
        invalid_request_info.set_urls(vec![]);
        assert!(invalid_request_info.validate().is_err());

        // Test with too many urls (violates max length)
        let mut invalid_request_info = valid_request_info.clone();
        invalid_request_info.set_urls(vec![
            "https://url1.example.com".to_string(),
            "https://url2.example.com".to_string(),
            "https://url3.example.com".to_string(),
            "https://url4.example.com".to_string(),
            "https://url5.example.com".to_string(),
            "https://url6.example.com".to_string(), // Exceeds max of 5
        ]);
        assert!(invalid_request_info.validate().is_err());

        // Test with URL that exceeds max length (2000 chars)
        let mut invalid_request_info = valid_request_info.clone();
        let long_url = format!("https://example.com/{}", "a".repeat(2000)); // URL longer than 2000 chars
        invalid_request_info.set_urls(vec![long_url]);
        assert!(invalid_request_info.validate().is_err());

        // Test with invalid certificate hash data (nested validation)
        let mut invalid_cert_hash_data = cert_hash_data.clone();
        invalid_cert_hash_data.set_issuer_name_hash("a".repeat(129)); // Exceeds max length of 128

        let mut invalid_request_info = valid_request_info.clone();
        invalid_request_info.set_certificate_hash_data(invalid_cert_hash_data);
        assert!(invalid_request_info.validate().is_err());

        // Test with invalid custom data (nested validation)
        let invalid_custom_data = CustomDataType::new("a".repeat(256)); // Exceeds max length of 255

        let mut invalid_request_info = valid_request_info.clone();
        invalid_request_info.set_custom_data(Some(invalid_custom_data));
        assert!(invalid_request_info.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let request_info = CertificateStatusRequestInfoType::new(
            cert_hash_data,
            CertificateStatusSourceEnumType::OCSP,
            vec!["https://ocsp.example.com".to_string()],
        )
        .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&request_info).unwrap();

        // Deserialize back
        let deserialized: CertificateStatusRequestInfoType =
            serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(request_info, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }

    #[test]
    fn test_edge_case_validations() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        // Test with exactly 5 URLs (max allowed)
        let max_urls = vec![
            "https://url1.example.com".to_string(),
            "https://url2.example.com".to_string(),
            "https://url3.example.com".to_string(),
            "https://url4.example.com".to_string(),
            "https://url5.example.com".to_string(),
        ];

        let max_urls_request = CertificateStatusRequestInfoType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::CRL,
            max_urls,
        );

        // Validation should pass with exactly 5 URLs
        assert!(max_urls_request.validate().is_ok());

        // Test with URL at exactly 2000 chars (max allowed)
        let max_length_url = format!("https://example.com/{}", "a".repeat(1980)); // Total length is 2000
        assert_eq!(max_length_url.len(), 2000);

        let max_length_url_request = CertificateStatusRequestInfoType::new(
            cert_hash_data,
            CertificateStatusSourceEnumType::OCSP,
            vec![max_length_url],
        );

        // Validation should pass with URL at exactly 2000 chars
        assert!(max_length_url_request.validate().is_ok());
    }
}
