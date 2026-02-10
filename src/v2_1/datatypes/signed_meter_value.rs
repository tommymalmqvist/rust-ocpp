use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Represent a signed version of the meter value.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignedMeterValueType {
    /// Required. Base64 encoded, contains the signed data from the meter in the format specified in _encodingMethod_,
    /// which might contain more then just the meter value. It can contain information like timestamps,
    /// reference to a customer etc.
    #[validate(length(max = 32768))]
    pub signed_meter_data: String,

    /// Required. Format used by the energy meter to encode the meter data. For example: OCMF or EDL.
    #[validate(length(max = 50))]
    pub encoding_method: String,

    /// Optional. *(2.1)* Method used to create the digital signature. Optional, if already included in _signedMeterData_.
    /// Standard values for this are defined in Appendix as SigningMethodEnumStringType.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub signing_method: Option<String>,

    /// Optional. *(2.1)* Base64 encoded, sending depends on configuration variable _PublicKeyWithSignedMeterValue_.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 2500))]
    pub public_key: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SignedMeterValueType {
    /// Creates a new `SignedMeterValueType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_data` - Base64 encoded, contains the signed data from the meter
    /// * `encoding_method` - Format used by the energy meter to encode the meter data
    ///
    /// # Returns
    ///
    /// A new instance of `SignedMeterValueType` with optional fields set to `None`
    pub fn new(signed_meter_data: String, encoding_method: String) -> Self {
        Self {
            signed_meter_data,
            encoding_method,
            signing_method: None,
            public_key: None,
            custom_data: None,
        }
    }

    /// Sets the signing method.
    ///
    /// # Arguments
    ///
    /// * `signing_method` - Method used to create the digital signature
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_signing_method(mut self, signing_method: String) -> Self {
        self.signing_method = Some(signing_method);
        self
    }

    /// Sets the public key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - Base64 encoded public key
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_public_key(mut self, public_key: String) -> Self {
        self.public_key = Some(public_key);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this signed meter value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the signed meter data.
    ///
    /// # Returns
    ///
    /// The Base64 encoded signed data from the meter
    pub fn signed_meter_data(&self) -> &str {
        &self.signed_meter_data
    }

    /// Sets the signed meter data.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_data` - Base64 encoded, contains the signed data from the meter
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signed_meter_data(&mut self, signed_meter_data: String) -> &mut Self {
        self.signed_meter_data = signed_meter_data;
        self
    }

    /// Gets the encoding method.
    ///
    /// # Returns
    ///
    /// The format used by the energy meter to encode the meter data
    pub fn encoding_method(&self) -> &str {
        &self.encoding_method
    }

    /// Sets the encoding method.
    ///
    /// # Arguments
    ///
    /// * `encoding_method` - Format used by the energy meter to encode the meter data
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_encoding_method(&mut self, encoding_method: String) -> &mut Self {
        self.encoding_method = encoding_method;
        self
    }

    /// Gets the signing method.
    ///
    /// # Returns
    ///
    /// An optional reference to the method used to create the digital signature
    pub fn signing_method(&self) -> Option<&str> {
        self.signing_method.as_deref()
    }

    /// Sets the signing method.
    ///
    /// # Arguments
    ///
    /// * `signing_method` - Method used to create the digital signature, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signing_method(&mut self, signing_method: Option<String>) -> &mut Self {
        self.signing_method = signing_method;
        self
    }

    /// Gets the public key.
    ///
    /// # Returns
    ///
    /// An optional reference to the Base64 encoded public key
    pub fn public_key(&self) -> Option<&str> {
        self.public_key.as_deref()
    }

    /// Sets the public key.
    ///
    /// # Arguments
    ///
    /// * `public_key` - Base64 encoded public key, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_public_key(&mut self, public_key: Option<String>) -> &mut Self {
        self.public_key = public_key;
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
    /// * `custom_data` - Custom data for this signed meter value, or None to clear
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
    fn test_new_signed_meter_value() {
        let signed_meter_data = "signed_data".to_string();
        let encoding_method = "OCMF".to_string();

        let value = SignedMeterValueType::new(signed_meter_data.clone(), encoding_method.clone());

        assert_eq!(value.signed_meter_data(), signed_meter_data);
        assert_eq!(value.encoding_method(), encoding_method);
        assert_eq!(value.signing_method(), None);
        assert_eq!(value.public_key(), None);
        assert_eq!(value.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let signed_meter_data = "signed_data".to_string();
        let encoding_method = "OCMF".to_string();
        let signing_method = "ECDSA-secp256r1-SHA256".to_string();
        let public_key = "public_key_base64".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let value = SignedMeterValueType::new(signed_meter_data.clone(), encoding_method.clone())
            .with_signing_method(signing_method.clone())
            .with_public_key(public_key.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(value.signed_meter_data(), signed_meter_data);
        assert_eq!(value.encoding_method(), encoding_method);
        assert_eq!(value.signing_method(), Some(signing_method.as_str()));
        assert_eq!(value.public_key(), Some(public_key.as_str()));
        assert_eq!(value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let signed_meter_data1 = "signed_data1".to_string();
        let encoding_method1 = "OCMF".to_string();

        let mut value = SignedMeterValueType::new(signed_meter_data1, encoding_method1);

        let signed_meter_data2 = "signed_data2".to_string();
        let encoding_method2 = "EDL".to_string();
        let signing_method = "ECDSA-secp256r1-SHA256".to_string();
        let public_key = "public_key_base64".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        value
            .set_signed_meter_data(signed_meter_data2.clone())
            .set_encoding_method(encoding_method2.clone())
            .set_signing_method(Some(signing_method.clone()))
            .set_public_key(Some(public_key.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(value.signed_meter_data(), signed_meter_data2);
        assert_eq!(value.encoding_method(), encoding_method2);
        assert_eq!(value.signing_method(), Some(signing_method.as_str()));
        assert_eq!(value.public_key(), Some(public_key.as_str()));
        assert_eq!(value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        value
            .set_signing_method(None)
            .set_public_key(None)
            .set_custom_data(None);

        assert_eq!(value.signing_method(), None);
        assert_eq!(value.public_key(), None);
        assert_eq!(value.custom_data(), None);
    }
}
