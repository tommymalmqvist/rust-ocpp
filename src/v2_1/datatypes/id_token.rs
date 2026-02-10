use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{additional_info::AdditionalInfoType, custom_data::CustomDataType};

/// Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdTokenType {
    /// Optional. Additional information about the identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub additional_info: Option<Vec<AdditionalInfoType>>,

    /// Required. IdToken is case insensitive. Might hold the hidden id of an RFID tag, but can for example also contain a UUID.
    #[validate(length(max = 255))]
    pub id_token: String,

    /// Required. Type of identification used to authorize charging.
    /// Allowed values: "Central", "DirectPayment", "eMAID", "EVCCID", "ISO14443", "ISO15693",
    /// "KeyCode", "Local", "MacAddress", "NoAuthorization", "VIN"
    #[serde(rename = "type")]
    #[validate(length(max = 20))]
    pub type_: String,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl IdTokenType {
    /// Creates a new `IdTokenType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID token string, case insensitive
    /// * `type_` - Type of identification used to authorize charging
    ///
    /// # Returns
    ///
    /// A new instance of `IdTokenType` with optional fields set to `None`
    pub fn new(id_token: String, type_: String) -> Self {
        Self {
            id_token,
            type_,
            additional_info: None,
            custom_data: None,
        }
    }

    /// Sets the additional information.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Vector of additional information about the identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_additional_info(mut self, additional_info: Vec<AdditionalInfoType>) -> Self {
        self.additional_info = Some(additional_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this ID token
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ID token.
    ///
    /// # Returns
    ///
    /// The ID token string
    pub fn id_token(&self) -> &str {
        &self.id_token
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID token string, case insensitive
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: String) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Gets the token type.
    ///
    /// # Returns
    ///
    /// The token type string
    pub fn type_(&self) -> &str {
        &self.type_
    }

    /// Sets the token type.
    ///
    /// # Arguments
    ///
    /// * `type_` - Type of identification used to authorize charging
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_type(&mut self, type_: String) -> &mut Self {
        self.type_ = type_;
        self
    }

    /// Gets the additional information.
    ///
    /// # Returns
    ///
    /// An optional reference to the vector of additional information
    pub fn additional_info(&self) -> Option<&Vec<AdditionalInfoType>> {
        self.additional_info.as_ref()
    }

    /// Sets the additional information.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Vector of additional information about the identifier, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_additional_info(
        &mut self,
        additional_info: Option<Vec<AdditionalInfoType>>,
    ) -> &mut Self {
        self.additional_info = additional_info;
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
    /// * `custom_data` - Custom data for this ID token, or None to clear
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
    fn test_new_id_token() {
        let id = "4F62C4E0123456789".to_string();
        let token_type = "ISO14443".to_string();

        let token = IdTokenType::new(id.clone(), token_type.clone());

        assert_eq!(token.id_token(), &id);
        assert_eq!(token.type_(), &token_type);
        assert_eq!(token.additional_info(), None);
        assert_eq!(token.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = "4F62C4E0123456789".to_string();
        let token_type = "ISO14443".to_string();

        let additional_info = vec![AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        }];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let token = IdTokenType::new(id.clone(), token_type.clone())
            .with_additional_info(additional_info.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(token.id_token(), &id);
        assert_eq!(token.type_(), &token_type);
        assert_eq!(token.additional_info(), Some(&additional_info));
        assert_eq!(token.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = "4F62C4E0123456789".to_string();
        let id2 = "ABCDEF0123456789".to_string();
        let token_type1 = "ISO14443".to_string();
        let token_type2 = "RFID".to_string();

        let additional_info = vec![AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        }];

        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut token = IdTokenType::new(id1.clone(), token_type1.clone());

        token
            .set_id_token(id2.clone())
            .set_type(token_type2.clone())
            .set_additional_info(Some(additional_info.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(token.id_token(), &id2);
        assert_eq!(token.type_(), &token_type2);
        assert_eq!(token.additional_info(), Some(&additional_info));
        assert_eq!(token.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        token.set_additional_info(None).set_custom_data(None);

        assert_eq!(token.additional_info(), None);
        assert_eq!(token.custom_data(), None);
    }

    #[test]
    fn test_validate() {
        // Valid token
        let token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string());
        assert!(token.validate().is_ok());

        // Test with valid additional info
        let additional_info = vec![AdditionalInfoType {
            additional_id_token: "Card123".to_string(),
            type_: "CardType".to_string(),
            custom_data: None,
        }];
        let token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string())
            .with_additional_info(additional_info);
        assert!(token.validate().is_ok());

        // Test with invalid id_token (too long)
        let token = IdTokenType::new("A".repeat(256), "ISO14443".to_string());
        assert!(token.validate().is_err());

        // Test with invalid type_ (too long)
        let token = IdTokenType::new("4F62C4E0123456789".to_string(), "A".repeat(21));
        assert!(token.validate().is_err());

        // Test with empty additional_info vector (should fail validation)
        let token = IdTokenType::new("4F62C4E0123456789".to_string(), "ISO14443".to_string())
            .with_additional_info(vec![]);
        assert!(token.validate().is_err());
    }
}
