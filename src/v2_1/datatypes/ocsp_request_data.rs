use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::HashAlgorithmEnumType;

/// Information about a certificate for an OCSP check.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,

    /// Required. The hash of the issuer's distinguished name (DN), that must be calculated over the DER encoding of the issuer's name field in the certificate being checked.
    #[validate(length(max = 128))]
    pub issuer_name_hash: String,

    /// Required. The hash of the DER encoded public key: the value (excluding tag and length) of the subject public key field in the issuer's certificate.
    #[validate(length(max = 128))]
    pub issuer_key_hash: String,

    /// Required. The string representation of the hexadecimal value of the serial number without the prefix "0x" and without leading zeroes.
    #[validate(length(max = 40))]
    pub serial_number: String,

    /// Required. This contains the responder URL (Case insensitive).
    #[validate(length(max = 2000))]
    pub responder_url: String,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl OCSPRequestDataType {
    /// Creates a new `OCSPRequestDataType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `hash_algorithm` - The hash algorithm used to calculate HashValue
    /// * `issuer_name_hash` - The hash value of the Issuer DN
    /// * `issuer_key_hash` - The hash value of the Issuer Public Key
    /// * `serial_number` - The serial number of the certificate
    /// * `responder_url` - The responder URL
    ///
    /// # Returns
    ///
    /// A new instance of `OCSPRequestDataType` with optional fields set to `None`
    pub fn new(
        hash_algorithm: HashAlgorithmEnumType,
        issuer_name_hash: String,
        issuer_key_hash: String,
        serial_number: String,
        responder_url: String,
    ) -> Self {
        Self {
            hash_algorithm,
            issuer_name_hash,
            issuer_key_hash,
            serial_number,
            responder_url,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this OCSP request data
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
    /// The hash algorithm used to calculate HashValue
    pub fn hash_algorithm(&self) -> &HashAlgorithmEnumType {
        &self.hash_algorithm
    }

    /// Sets the hash algorithm.
    ///
    /// # Arguments
    ///
    /// * `hash_algorithm` - The hash algorithm used to calculate HashValue
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
    /// The hash value of the Issuer DN
    pub fn issuer_name_hash(&self) -> &str {
        &self.issuer_name_hash
    }

    /// Sets the issuer name hash.
    ///
    /// # Arguments
    ///
    /// * `issuer_name_hash` - The hash value of the Issuer DN
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
    /// The hash value of the Issuer Public Key
    pub fn issuer_key_hash(&self) -> &str {
        &self.issuer_key_hash
    }

    /// Sets the issuer key hash.
    ///
    /// # Arguments
    ///
    /// * `issuer_key_hash` - The hash value of the Issuer Public Key
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
    /// The serial number of the certificate
    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    /// Sets the serial number.
    ///
    /// # Arguments
    ///
    /// * `serial_number` - The serial number of the certificate
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_serial_number(&mut self, serial_number: String) -> &mut Self {
        self.serial_number = serial_number;
        self
    }

    /// Gets the responder URL.
    ///
    /// # Returns
    ///
    /// The responder URL
    pub fn responder_url(&self) -> &str {
        &self.responder_url
    }

    /// Sets the responder URL.
    ///
    /// # Arguments
    ///
    /// * `responder_url` - The responder URL
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_responder_url(&mut self, responder_url: String) -> &mut Self {
        self.responder_url = responder_url;
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
    /// * `custom_data` - Custom data for this OCSP request data, or None to clear
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
    fn test_new_ocsp_request_data() {
        let hash_algorithm = HashAlgorithmEnumType::SHA256;
        let issuer_name_hash = "1234567890abcdef1234567890abcdef".to_string();
        let issuer_key_hash = "abcdef1234567890abcdef1234567890".to_string();
        let serial_number = "0123456789".to_string();
        let responder_url = "https://ocsp.example.com".to_string();

        let ocsp_data = OCSPRequestDataType::new(
            hash_algorithm.clone(),
            issuer_name_hash.clone(),
            issuer_key_hash.clone(),
            serial_number.clone(),
            responder_url.clone(),
        );

        assert_eq!(ocsp_data.hash_algorithm(), &hash_algorithm);
        assert_eq!(ocsp_data.issuer_name_hash(), issuer_name_hash);
        assert_eq!(ocsp_data.issuer_key_hash(), issuer_key_hash);
        assert_eq!(ocsp_data.serial_number(), serial_number);
        assert_eq!(ocsp_data.responder_url(), responder_url);
        assert_eq!(ocsp_data.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let hash_algorithm = HashAlgorithmEnumType::SHA256;
        let issuer_name_hash = "1234567890abcdef1234567890abcdef".to_string();
        let issuer_key_hash = "abcdef1234567890abcdef1234567890".to_string();
        let serial_number = "0123456789".to_string();
        let responder_url = "https://ocsp.example.com".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let ocsp_data = OCSPRequestDataType::new(
            hash_algorithm.clone(),
            issuer_name_hash.clone(),
            issuer_key_hash.clone(),
            serial_number.clone(),
            responder_url.clone(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(ocsp_data.hash_algorithm(), &hash_algorithm);
        assert_eq!(ocsp_data.issuer_name_hash(), issuer_name_hash);
        assert_eq!(ocsp_data.issuer_key_hash(), issuer_key_hash);
        assert_eq!(ocsp_data.serial_number(), serial_number);
        assert_eq!(ocsp_data.responder_url(), responder_url);
        assert_eq!(ocsp_data.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let hash_algorithm1 = HashAlgorithmEnumType::SHA256;
        let hash_algorithm2 = HashAlgorithmEnumType::SHA384;
        let issuer_name_hash1 = "1234567890abcdef1234567890abcdef".to_string();
        let issuer_name_hash2 = "fedcba0987654321fedcba0987654321".to_string();
        let issuer_key_hash1 = "abcdef1234567890abcdef1234567890".to_string();
        let issuer_key_hash2 = "0987654321fedcba0987654321fedcba".to_string();
        let serial_number1 = "0123456789".to_string();
        let serial_number2 = "9876543210".to_string();
        let responder_url1 = "https://ocsp.example.com".to_string();
        let responder_url2 = "https://ocsp.example.org".to_string();
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut ocsp_data = OCSPRequestDataType::new(
            hash_algorithm1.clone(),
            issuer_name_hash1.clone(),
            issuer_key_hash1.clone(),
            serial_number1.clone(),
            responder_url1.clone(),
        );

        ocsp_data
            .set_hash_algorithm(hash_algorithm2.clone())
            .set_issuer_name_hash(issuer_name_hash2.clone())
            .set_issuer_key_hash(issuer_key_hash2.clone())
            .set_serial_number(serial_number2.clone())
            .set_responder_url(responder_url2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(ocsp_data.hash_algorithm(), &hash_algorithm2);
        assert_eq!(ocsp_data.issuer_name_hash(), issuer_name_hash2);
        assert_eq!(ocsp_data.issuer_key_hash(), issuer_key_hash2);
        assert_eq!(ocsp_data.serial_number(), serial_number2);
        assert_eq!(ocsp_data.responder_url(), responder_url2);
        assert_eq!(ocsp_data.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        ocsp_data.set_custom_data(None);
        assert_eq!(ocsp_data.custom_data(), None);
    }
}
