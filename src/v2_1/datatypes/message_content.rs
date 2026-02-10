use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::MessageFormatEnumType;

/// Contains message details, for a message to be displayed on a Charging Station.
///
/// This type is used to display messages on a Charging Station's screen.
/// The message content can be formatted in different ways (ASCII, HTML, etc.)
/// and supports internationalization through the language field.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageContentType {
    /// Required. Message contents.
    ///
    /// Maximum length is 1024 characters as defined in the OCPP 2.1 specification.
    #[validate(length(max = 1024))]
    pub content: String,

    /// Required. Format of the message.
    ///
    /// Defines how the message should be rendered on the Charging Station's display.
    pub format: MessageFormatEnumType,

    /// Required. Language identifier of the message content.
    ///
    /// Contains a language code as defined in RFC5646.
    #[validate(length(max = 8))]
    pub language: String,

    /// Custom data from the Charging Station.
    ///
    /// This field can be used to add vendor-specific information to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MessageContentType {
    /// Creates a new `MessageContentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `content` - Message contents (max 1024 characters)
    /// * `format` - Format of the message (ASCII, HTML, etc.)
    /// * `language` - Language identifier of the message content (RFC5646 language code, max 8 characters)
    ///
    /// # Returns
    ///
    /// A new instance of `MessageContentType` with optional fields set to `None`
    ///
    /// # Example
    ///
    /// ```
    /// use rust_ocpp::v2_1::datatypes::message_content::MessageContentType;
    /// use rust_ocpp::v2_1::enumerations::MessageFormatEnumType;
    ///
    /// let message = MessageContentType::new(
    ///     "Please plug in your vehicle.".to_string(),
    ///     MessageFormatEnumType::ASCII,
    ///     "en".to_string()
    /// );
    /// ```
    pub fn new(content: String, format: MessageFormatEnumType, language: String) -> Self {
        Self {
            content,
            format,
            language,
            custom_data: None,
        }
    }

    /// Creates a builder for `MessageContentType` with required fields.
    ///
    /// This is an alternative to using `new()` followed by `with_*` methods.
    ///
    /// # Arguments
    ///
    /// * `content` - Message contents
    /// * `format` - Format of the message
    /// * `language` - Language identifier of the message content
    ///
    /// # Returns
    ///
    /// A new instance of `MessageContentType` with optional fields set to `None`
    pub fn builder(content: String, format: MessageFormatEnumType, language: String) -> Self {
        Self::new(content, format, language)
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this message content
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    ///
    /// # Example
    ///
    /// ```
    /// use rust_ocpp::v2_1::datatypes::message_content::MessageContentType;
    /// use rust_ocpp::v2_1::datatypes::custom_data::CustomDataType;
    /// use rust_ocpp::v2_1::enumerations::MessageFormatEnumType;
    ///
    /// let custom_data = CustomDataType::new("VendorX".to_string());
    /// let message = MessageContentType::new(
    ///     "Please plug in your vehicle.".to_string(),
    ///     MessageFormatEnumType::ASCII,
    ///     "en".to_string()
    /// ).with_custom_data(custom_data);
    /// ```
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the message content.
    ///
    /// # Returns
    ///
    /// The message contents as a string slice
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets the message content.
    ///
    /// # Arguments
    ///
    /// * `content` - Message contents (max 1024 characters)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    /// Gets the format of the message.
    ///
    /// # Returns
    ///
    /// A reference to the format of the message
    pub fn format(&self) -> &MessageFormatEnumType {
        &self.format
    }

    /// Sets the format of the message.
    ///
    /// # Arguments
    ///
    /// * `format` - Format of the message (ASCII, HTML, etc.)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_format(&mut self, format: MessageFormatEnumType) -> &mut Self {
        self.format = format;
        self
    }

    /// Gets the language identifier.
    ///
    /// # Returns
    ///
    /// The language identifier of the message content as a string slice
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Sets the language identifier.
    ///
    /// # Arguments
    ///
    /// * `language` - Language identifier of the message content (RFC5646 language code, max 8 characters)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_language(&mut self, language: String) -> &mut Self {
        self.language = language;
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
    /// * `custom_data` - Custom data for this message content, or None to clear
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
    /// Ok(()) if the instance is valid, otherwise an error with validation details
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Validate::validate(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_new_message_content() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();

        let message = MessageContentType::new(content.clone(), format.clone(), language.clone());

        assert_eq!(message.content(), content);
        assert_eq!(message.format(), &format);
        assert_eq!(message.language(), language);
        assert_eq!(message.custom_data(), None);
    }

    #[test]
    fn test_builder() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();

        let message =
            MessageContentType::builder(content.clone(), format.clone(), language.clone());

        assert_eq!(message.content(), content);
        assert_eq!(message.format(), &format);
        assert_eq!(message.language(), language);
        assert_eq!(message.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let message = MessageContentType::new(content.clone(), format.clone(), language.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(message.content(), content);
        assert_eq!(message.format(), &format);
        assert_eq!(message.language(), language);
        assert_eq!(message.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let content1 = "Please plug in your vehicle.".to_string();
        let content2 = "Charging session complete.".to_string();
        let format1 = MessageFormatEnumType::ASCII;
        let format2 = MessageFormatEnumType::HTML;
        let language1 = "en".to_string();
        let language2 = "fr".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut message =
            MessageContentType::new(content1.clone(), format1.clone(), language1.clone());

        message
            .set_content(content2.clone())
            .set_format(format2.clone())
            .set_language(language2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(message.content(), content2);
        assert_eq!(message.format(), &format2);
        assert_eq!(message.language(), language2);
        assert_eq!(message.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        message.set_custom_data(None);
        assert_eq!(message.custom_data(), None);
    }

    #[test]
    fn test_validation_success() {
        // Valid message content
        let message = MessageContentType::new(
            "Valid message".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        // Validation should pass
        assert!(message.validate().is_ok());
    }

    #[test]
    fn test_validation_content_length() {
        // Create a message with content that exceeds the maximum length (1024 characters)
        let long_content = "a".repeat(1025);
        let message =
            MessageContentType::new(long_content, MessageFormatEnumType::ASCII, "en".to_string());

        // Validation should fail
        let result = message.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("content"));
    }

    #[test]
    fn test_validation_language_length() {
        // Create a message with language that exceeds the maximum length (8 characters)
        let long_language = "language_too_long".to_string();
        let message = MessageContentType::new(
            "Valid message".to_string(),
            MessageFormatEnumType::ASCII,
            long_language,
        );

        // Validation should fail
        let result = message.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("language"));
    }

    #[test]
    fn test_validation_custom_data() {
        // Create a message with invalid custom data (vendor_id too long)
        let invalid_vendor_id = "a".repeat(256); // Max length is 255
        let invalid_custom_data = CustomDataType::new(invalid_vendor_id);

        let message = MessageContentType::new(
            "Valid message".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        )
        .with_custom_data(invalid_custom_data);

        // Validation should fail
        let result = message.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let content = "Please plug in your vehicle.".to_string();
        let format = MessageFormatEnumType::ASCII;
        let language = "en".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let message = MessageContentType::new(content.clone(), format.clone(), language.clone())
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();

        // Check that fields are correctly serialized
        assert_eq!(deserialized["content"], content);
        assert_eq!(deserialized["format"], "ASCII");
        assert_eq!(deserialized["language"], language);
        assert_eq!(deserialized["customData"]["vendorId"], "VendorX");
        assert_eq!(deserialized["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization() {
        // Create JSON with all fields
        let json_with_all_fields = json!({
            "content": "Please plug in your vehicle.",
            "format": "HTML",
            "language": "fr",
            "customData": {
                "vendorId": "VendorY",
                "extraInfo": "Additional information"
            }
        });

        // Deserialize
        let message: MessageContentType = serde_json::from_value(json_with_all_fields).unwrap();

        // Check fields
        assert_eq!(message.content(), "Please plug in your vehicle.");
        assert_eq!(message.format(), &MessageFormatEnumType::HTML);
        assert_eq!(message.language(), "fr");
        assert!(message.custom_data().is_some());
        assert_eq!(message.custom_data().unwrap().vendor_id(), "VendorY");

        // Create JSON with only required fields
        let json_required_only = json!({
            "content": "Required message",
            "format": "ASCII",
            "language": "en"
        });

        // Deserialize
        let message: MessageContentType = serde_json::from_value(json_required_only).unwrap();

        // Check fields
        assert_eq!(message.content(), "Required message");
        assert_eq!(message.format(), &MessageFormatEnumType::ASCII);
        assert_eq!(message.language(), "en");
        assert!(message.custom_data().is_none());
    }

    #[test]
    fn test_all_message_formats() {
        // Test with all possible message formats
        let formats = vec![
            MessageFormatEnumType::ASCII,
            MessageFormatEnumType::HTML,
            MessageFormatEnumType::URI,
            MessageFormatEnumType::UTF8,
            MessageFormatEnumType::QRCODE,
        ];

        for format in formats {
            let message = MessageContentType::new(
                "Test message".to_string(),
                format.clone(),
                "en".to_string(),
            );

            assert_eq!(message.format(), &format);
        }
    }
}
