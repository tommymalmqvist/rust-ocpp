use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::{CertificateStatusEnumType, CertificateStatusSourceEnumType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Revocation status of certificate
///
/// This type represents the status of a certificate, including its revocation status,
/// source of the status information, and when the next update is expected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateStatusType {
    /// Certificate hash data needed for validating certificates through OCSP.
    #[validate(nested)]
    pub certificate_hash_data: CertificateHashDataType,

    /// Source of status: OCSP, CRL
    pub source: CertificateStatusSourceEnumType,

    /// Status of certificate: good, revoked or unknown.
    pub status: CertificateStatusEnumType,

    /// The date and time at which the next update of the certificate status MAY be expected.
    pub next_update: DateTime<Utc>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateStatusType {
    /// Creates a new `CertificateStatusType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Certificate hash data for validation
    /// * `source` - Source of the certificate status information
    /// * `status` - Status of the certificate
    /// * `next_update` - Expected time of next status update
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateStatusType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        source: CertificateStatusSourceEnumType,
        status: CertificateStatusEnumType,
        next_update: DateTime<Utc>,
    ) -> Self {
        Self {
            certificate_hash_data,
            source,
            status,
            next_update,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate status
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

    /// Gets the status of the certificate.
    ///
    /// # Returns
    ///
    /// The status of the certificate
    pub fn status(&self) -> &CertificateStatusEnumType {
        &self.status
    }

    /// Sets the status of the certificate.
    ///
    /// # Arguments
    ///
    /// * `status` - Status of the certificate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_status(&mut self, status: CertificateStatusEnumType) -> &mut Self {
        self.status = status;
        self
    }

    /// Gets the next update time.
    ///
    /// # Returns
    ///
    /// The expected time of the next status update
    pub fn next_update(&self) -> &DateTime<Utc> {
        &self.next_update
    }

    /// Sets the next update time.
    ///
    /// # Arguments
    ///
    /// * `next_update` - Expected time of next status update
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_next_update(&mut self, next_update: DateTime<Utc>) -> &mut Self {
        self.next_update = next_update;
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
    /// * `custom_data` - Custom data for this certificate status, or None to clear
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
    use chrono::TimeZone;
    use serde_json::json;

    #[test]
    fn test_new_certificate_status() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let next_update = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let cert_status = CertificateStatusType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update,
        );

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Good);
        assert_eq!(cert_status.next_update(), &next_update);
        assert_eq!(cert_status.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string());
        let next_update = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let cert_status = CertificateStatusType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update,
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::OCSP);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Good);
        assert_eq!(cert_status.next_update(), &next_update);
        assert_eq!(cert_status.custom_data(), Some(&custom_data));
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

        let custom_data = CustomDataType::new("VendorX".to_string());
        let next_update1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let next_update2 = Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();

        let mut cert_status = CertificateStatusType::new(
            cert_hash_data1.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update1,
        );

        cert_status
            .set_certificate_hash_data(cert_hash_data2.clone())
            .set_source(CertificateStatusSourceEnumType::CRL)
            .set_status(CertificateStatusEnumType::Revoked)
            .set_next_update(next_update2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(cert_status.certificate_hash_data(), &cert_hash_data2);
        assert_eq!(cert_status.source(), &CertificateStatusSourceEnumType::CRL);
        assert_eq!(cert_status.status(), &CertificateStatusEnumType::Revoked);
        assert_eq!(cert_status.next_update(), &next_update2);
        assert_eq!(cert_status.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cert_status.set_custom_data(None);
        assert_eq!(cert_status.custom_data(), None);
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

        // Valid certificate status
        let next_update = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let valid_cert_status = CertificateStatusType::new(
            cert_hash_data.clone(),
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update,
        );

        // Validation should pass
        assert!(valid_cert_status.validate().is_ok());

        // Test with invalid certificate hash data (nested validation)
        let mut invalid_cert_hash_data = cert_hash_data.clone();
        invalid_cert_hash_data.set_issuer_name_hash("a".repeat(129)); // Exceeds max length of 128

        let mut invalid_cert_status = valid_cert_status.clone();
        invalid_cert_status.set_certificate_hash_data(invalid_cert_hash_data);
        assert!(invalid_cert_status.validate().is_err());

        // Test with invalid custom data (nested validation)
        let invalid_custom_data = CustomDataType::new("a".repeat(256)); // Exceeds max length of 255

        let mut invalid_cert_status = valid_cert_status.clone();
        invalid_cert_status.set_custom_data(Some(invalid_custom_data));
        assert!(invalid_cert_status.validate().is_err());
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

        let next_update = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let cert_status = CertificateStatusType::new(
            cert_hash_data,
            CertificateStatusSourceEnumType::OCSP,
            CertificateStatusEnumType::Good,
            next_update,
        )
        .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&cert_status).unwrap();

        // Deserialize back
        let deserialized: CertificateStatusType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(cert_status, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }
}
