use crate::v2_1::datatypes::{
    certificate_hash_data::CertificateHashDataType, custom_data::CustomDataType,
};
use crate::v2_1::enumerations::get_certificate_id_use::GetCertificateIdUseEnumType;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Certificate hash data chain for validating certificates through OCSP.
///
/// This type represents a chain of certificate hash data used for certificate validation
/// through the Online Certificate Status Protocol (OCSP).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    /// Information to identify a certificate
    #[validate(nested)]
    pub certificate_hash_data: CertificateHashDataType,

    /// Indicates the type of the requested certificate(s).
    pub certificate_type: GetCertificateIdUseEnumType,

    /// Information to identify the child certificate(s).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4), nested)]
    pub child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl CertificateHashDataChainType {
    /// Creates a new `CertificateHashDataChainType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `certificate_hash_data` - Information to identify a certificate
    /// * `certificate_type` - Type of the requested certificate(s)
    ///
    /// # Returns
    ///
    /// A new instance of `CertificateHashDataChainType` with optional fields set to `None`
    pub fn new(
        certificate_hash_data: CertificateHashDataType,
        certificate_type: GetCertificateIdUseEnumType,
    ) -> Self {
        Self {
            certificate_hash_data,
            certificate_type,
            child_certificate_hash_data: None,
            custom_data: None,
        }
    }

    /// Sets the child certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `child_certificate_hash_data` - Information to identify the child certificate(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_child_certificate_hash_data(
        mut self,
        child_certificate_hash_data: Vec<CertificateHashDataType>,
    ) -> Self {
        self.child_certificate_hash_data = Some(child_certificate_hash_data);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this certificate hash data chain
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
    /// * `certificate_hash_data` - Information to identify a certificate
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

    /// Gets the certificate type.
    ///
    /// # Returns
    ///
    /// The type of the requested certificate(s)
    pub fn certificate_type(&self) -> &GetCertificateIdUseEnumType {
        &self.certificate_type
    }

    /// Sets the certificate type.
    ///
    /// # Arguments
    ///
    /// * `certificate_type` - Type of the requested certificate(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_certificate_type(
        &mut self,
        certificate_type: GetCertificateIdUseEnumType,
    ) -> &mut Self {
        self.certificate_type = certificate_type;
        self
    }

    /// Gets the child certificate hash data.
    ///
    /// # Returns
    ///
    /// An optional reference to the child certificate hash data
    pub fn child_certificate_hash_data(&self) -> Option<&Vec<CertificateHashDataType>> {
        self.child_certificate_hash_data.as_ref()
    }

    /// Sets the child certificate hash data.
    ///
    /// # Arguments
    ///
    /// * `child_certificate_hash_data` - Information to identify the child certificate(s), or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_child_certificate_hash_data(
        &mut self,
        child_certificate_hash_data: Option<Vec<CertificateHashDataType>>,
    ) -> &mut Self {
        self.child_certificate_hash_data = child_certificate_hash_data;
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
    /// * `custom_data` - Custom data for this certificate hash data chain, or None to clear
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
    use crate::v2_1::enumerations::HashAlgorithmEnumType;

    #[test]
    fn test_new_certificate_hash_data_chain() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let cert_chain = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        );

        assert_eq!(cert_chain.certificate_hash_data(), &cert_hash_data);
        assert_eq!(
            cert_chain.certificate_type(),
            &GetCertificateIdUseEnumType::CSMSRootCertificate
        );
        assert_eq!(cert_chain.child_certificate_hash_data(), None);
        assert_eq!(cert_chain.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        let child_cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string());

        let cert_chain = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        )
        .with_child_certificate_hash_data(vec![child_cert_hash_data.clone()])
        .with_custom_data(custom_data.clone());

        assert_eq!(cert_chain.certificate_hash_data(), &cert_hash_data);
        assert_eq!(
            cert_chain.certificate_type(),
            &GetCertificateIdUseEnumType::CSMSRootCertificate
        );
        assert_eq!(
            cert_chain.child_certificate_hash_data(),
            Some(&vec![child_cert_hash_data])
        );
        assert_eq!(cert_chain.custom_data(), Some(&custom_data));
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

        let child_cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut cert_chain = CertificateHashDataChainType::new(
            cert_hash_data1.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        );

        cert_chain
            .set_certificate_hash_data(cert_hash_data2.clone())
            .set_certificate_type(GetCertificateIdUseEnumType::V2GRootCertificate)
            .set_child_certificate_hash_data(Some(vec![child_cert_hash_data.clone()]))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(cert_chain.certificate_hash_data(), &cert_hash_data2);
        assert_eq!(
            cert_chain.certificate_type(),
            &GetCertificateIdUseEnumType::V2GRootCertificate
        );
        assert_eq!(
            cert_chain.child_certificate_hash_data(),
            Some(&vec![child_cert_hash_data])
        );
        assert_eq!(cert_chain.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        cert_chain
            .set_child_certificate_hash_data(None)
            .set_custom_data(None);

        assert_eq!(cert_chain.child_certificate_hash_data(), None);
        assert_eq!(cert_chain.custom_data(), None);
    }

    #[test]
    fn test_multiple_child_certificates() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "parent_name_hash".to_string(),
            "parent_key_hash".to_string(),
            "parent_serial".to_string(),
        );

        let child1 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child1_name_hash".to_string(),
            "child1_key_hash".to_string(),
            "child1_serial".to_string(),
        );

        let child2 = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "child2_name_hash".to_string(),
            "child2_key_hash".to_string(),
            "child2_serial".to_string(),
        );

        let children = vec![child1.clone(), child2.clone()];

        let cert_chain = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::V2GCertificateChain,
        )
        .with_child_certificate_hash_data(children.clone());

        // Verify children are stored correctly
        assert_eq!(cert_chain.child_certificate_hash_data(), Some(&children));

        // Get and check individual children
        if let Some(stored_children) = cert_chain.child_certificate_hash_data() {
            assert_eq!(stored_children.len(), 2);
            assert_eq!(&stored_children[0], &child1);
            assert_eq!(&stored_children[1], &child2);

            // Check first child properties
            assert_eq!(
                stored_children[0].hash_algorithm(),
                &HashAlgorithmEnumType::SHA384
            );
            assert_eq!(stored_children[0].issuer_name_hash(), "child1_name_hash");
            assert_eq!(stored_children[0].issuer_key_hash(), "child1_key_hash");
            assert_eq!(stored_children[0].serial_number(), "child1_serial");

            // Check second child properties
            assert_eq!(
                stored_children[1].hash_algorithm(),
                &HashAlgorithmEnumType::SHA512
            );
            assert_eq!(stored_children[1].issuer_name_hash(), "child2_name_hash");
            assert_eq!(stored_children[1].issuer_key_hash(), "child2_key_hash");
            assert_eq!(stored_children[1].serial_number(), "child2_serial");
        } else {
            panic!("Child certificates should be present");
        }
    }

    #[test]
    fn test_certificate_chain_with_all_certificate_types() {
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "name_hash".to_string(),
            "key_hash".to_string(),
            "serial_number".to_string(),
        );

        // Test all certificate types
        let certificate_types = [
            GetCertificateIdUseEnumType::V2GRootCertificate,
            GetCertificateIdUseEnumType::MORootCertificate,
            GetCertificateIdUseEnumType::CSMSRootCertificate,
            GetCertificateIdUseEnumType::V2GCertificateChain,
            GetCertificateIdUseEnumType::ManufacturerRootCertificate,
            GetCertificateIdUseEnumType::OEMRootCertificate,
        ];

        for cert_type in &certificate_types {
            let cert_chain =
                CertificateHashDataChainType::new(cert_hash_data.clone(), cert_type.clone());

            assert_eq!(cert_chain.certificate_type(), cert_type);
        }
    }

    #[test]
    fn test_validation_constraints() {
        // Create valid certificate hash data
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        // Create valid child certificate hash data
        let child_cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        // Create valid custom data
        let custom_data = CustomDataType::new("VendorX".to_string());

        // 1. Test valid certificate hash data chain
        let valid_chain = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        )
        .with_child_certificate_hash_data(vec![child_cert_hash_data.clone()])
        .with_custom_data(custom_data.clone());

        // 2. Check empty child_certificate_hash_data array (just check it can be created)
        let _chain_empty_children = valid_chain.clone();
        let empty_vec: Vec<CertificateHashDataType> = vec![];
        let chain_with_empty = CertificateHashDataChainType {
            certificate_hash_data: cert_hash_data.clone(),
            certificate_type: GetCertificateIdUseEnumType::CSMSRootCertificate,
            child_certificate_hash_data: Some(empty_vec),
            custom_data: None,
        };
        assert_eq!(
            chain_with_empty
                .child_certificate_hash_data()
                .unwrap()
                .len(),
            0
        );

        // 3. Test many child certificates (more than 4 should be technically possible)
        let many_children = vec![
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(), // 5 items
        ];
        let chain_many_children = CertificateHashDataChainType {
            certificate_hash_data: cert_hash_data.clone(),
            certificate_type: GetCertificateIdUseEnumType::CSMSRootCertificate,
            child_certificate_hash_data: Some(many_children.clone()),
            custom_data: None,
        };
        assert_eq!(
            chain_many_children
                .child_certificate_hash_data()
                .unwrap()
                .len(),
            5
        );

        // 4. Test custom data usage
        let custom_data = CustomDataType::new("VendorX".to_string());
        let chain_with_custom = CertificateHashDataChainType {
            certificate_hash_data: cert_hash_data.clone(),
            certificate_type: GetCertificateIdUseEnumType::CSMSRootCertificate,
            child_certificate_hash_data: None,
            custom_data: Some(custom_data.clone()),
        };
        assert_eq!(
            chain_with_custom.custom_data().unwrap().vendor_id(),
            "VendorX"
        );
    }

    #[test]
    fn test_child_certificate_variations() {
        // Test with exactly 1 child certificate
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "name_hash".to_string(),
            "key_hash".to_string(),
            "serial".to_string(),
        );

        let child_cert = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA512,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        let chain_min_children = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::V2GCertificateChain,
        )
        .with_child_certificate_hash_data(vec![child_cert.clone()]);

        assert_eq!(
            chain_min_children
                .child_certificate_hash_data()
                .unwrap()
                .len(),
            1,
            "Chain should have 1 child certificate"
        );

        // Test with 4 child certificates
        let child_certs = vec![
            child_cert.clone(),
            child_cert.clone(),
            child_cert.clone(),
            child_cert.clone(),
        ];

        let chain_max_children = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::V2GCertificateChain,
        )
        .with_child_certificate_hash_data(child_certs);

        assert_eq!(
            chain_max_children
                .child_certificate_hash_data()
                .unwrap()
                .len(),
            4,
            "Chain should have 4 child certificates"
        );
    }

    #[test]
    fn test_serde_serialization() {
        use serde_json;

        // Create a certificate chain
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "parent_name_hash".to_string(),
            "parent_key_hash".to_string(),
            "parent_serial".to_string(),
        );

        let child_cert = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string());

        let cert_chain = CertificateHashDataChainType::new(
            cert_hash_data,
            GetCertificateIdUseEnumType::V2GRootCertificate,
        )
        .with_child_certificate_hash_data(vec![child_cert])
        .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&cert_chain).unwrap();

        // Deserialize from JSON
        let deserialized: CertificateHashDataChainType = serde_json::from_str(&serialized).unwrap();

        // Verify data integrity after serialization/deserialization
        assert_eq!(cert_chain, deserialized);

        // Check specific camelCase field names in JSON
        assert!(serialized.contains("\"certificateHashData\""));
        assert!(serialized.contains("\"certificateType\""));
        assert!(serialized.contains("\"childCertificateHashData\""));
        assert!(serialized.contains("\"customData\""));

        // Check specific enum values
        assert!(serialized.contains("V2GRootCertificate"));
        assert!(serialized.contains("SHA256"));
        assert!(serialized.contains("SHA384"));
    }

    #[test]
    fn test_validator_validation() {
        use validator::Validate;

        // Create valid certificate hash data
        let cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA256,
            "a1b2c3d4e5f6".to_string(),
            "f6e5d4c3b2a1".to_string(),
            "1234567890abcdef".to_string(),
        );

        // Create valid child certificate hash data
        let child_cert_hash_data = CertificateHashDataType::new(
            HashAlgorithmEnumType::SHA384,
            "child_name_hash".to_string(),
            "child_key_hash".to_string(),
            "child_serial".to_string(),
        );

        // Create valid custom data
        let custom_data = CustomDataType::new("VendorX".to_string());

        // 1. Test valid certificate hash data chain with all fields
        let valid_chain_all_fields = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        )
        .with_child_certificate_hash_data(vec![child_cert_hash_data.clone()])
        .with_custom_data(custom_data.clone());

        let validation_result = valid_chain_all_fields.validate();
        assert!(
            validation_result.is_ok(),
            "Valid chain with all fields should pass validation"
        );

        // 2. Test valid certificate hash data chain with only required fields
        let valid_chain_required_fields = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        );

        let validation_result = valid_chain_required_fields.validate();
        assert!(
            validation_result.is_ok(),
            "Valid chain with only required fields should pass validation"
        );

        // 3. Test valid certificate hash data chain with multiple child certificates (max allowed)
        let valid_chain_max_children = CertificateHashDataChainType::new(
            cert_hash_data.clone(),
            GetCertificateIdUseEnumType::CSMSRootCertificate,
        )
        .with_child_certificate_hash_data(vec![
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(), // 4 items, max is 4
        ]);

        let validation_result = valid_chain_max_children.validate();
        assert!(
            validation_result.is_ok(),
            "Valid chain with maximum allowed child certificates should pass validation"
        );

        // 4. Test empty child_certificate_hash_data array (should fail validation)
        let mut invalid_chain_empty_children = valid_chain_all_fields.clone();
        invalid_chain_empty_children.child_certificate_hash_data = Some(vec![]);

        let validation_result = invalid_chain_empty_children.validate();
        assert!(
            validation_result.is_err(),
            "Chain with empty child_certificate_hash_data array should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("child_certificate_hash_data"),
            "Error should mention child_certificate_hash_data: {}",
            error
        );

        // 5. Test too many child certificates (more than 4, should fail validation)
        let mut invalid_chain_too_many_children = valid_chain_all_fields.clone();
        let too_many_children = vec![
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(),
            child_cert_hash_data.clone(), // 5 items, max is 4
        ];
        invalid_chain_too_many_children.child_certificate_hash_data = Some(too_many_children);

        let validation_result = invalid_chain_too_many_children.validate();
        assert!(
            validation_result.is_err(),
            "Chain with too many child certificates should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("child_certificate_hash_data"),
            "Error should mention child_certificate_hash_data: {}",
            error
        );

        // 6. Test invalid custom data (nested validation)
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255

        let mut invalid_chain_custom_data = valid_chain_all_fields.clone();
        invalid_chain_custom_data.custom_data = Some(invalid_custom_data);

        let validation_result = invalid_chain_custom_data.validate();
        assert!(
            validation_result.is_err(),
            "Chain with invalid custom_data should fail validation"
        );
        let error = validation_result.unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }
}
