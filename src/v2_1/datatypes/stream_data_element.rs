use super::custom_data::CustomDataType;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Class representing a data element for a stream.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StreamDataElementType {
    /// Required. Offset relative to _basetime_ of this message.
    /// _basetime_ + _t_ is timestamp of recorded value.
    #[serde(rename = "t", with = "rust_decimal::serde::arbitrary_precision")]
    pub offset: Decimal,

    /// Required. The value.
    #[serde(rename = "v")]
    #[validate(length(max = 2500))]
    pub value: String,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StreamDataElementType {
    /// Creates a new `StreamDataElementType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset relative to basetime of this message
    /// * `value` - The value
    ///
    /// # Returns
    ///
    /// A new instance of `StreamDataElementType` with optional fields set to `None`
    pub fn new(offset: Decimal, value: String) -> Self {
        Self {
            offset,
            value,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this stream data element
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the offset.
    ///
    /// # Returns
    ///
    /// The offset relative to basetime of this message
    pub fn offset(&self) -> Decimal {
        self.offset
    }

    /// Sets the offset.
    ///
    /// # Arguments
    ///
    /// * `offset` - Offset relative to basetime of this message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_offset(&mut self, offset: Decimal) -> &mut Self {
        self.offset = offset;
        self
    }

    /// Gets the value.
    ///
    /// # Returns
    ///
    /// The value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
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
    /// * `custom_data` - Custom data for this stream data element, or None to clear
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
    fn test_new_stream_data_element() {
        let offset = Decimal::new(425, 1);
        let value = "test_value".to_string();

        let element = StreamDataElementType::new(offset, value.clone());

        assert_eq!(element.offset(), offset);
        assert_eq!(element.value(), value);
        assert_eq!(element.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let offset = Decimal::new(425, 1);
        let value = "test_value".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let element =
            StreamDataElementType::new(offset, value.clone()).with_custom_data(custom_data.clone());

        assert_eq!(element.offset(), offset);
        assert_eq!(element.value(), value);
        assert_eq!(element.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let offset1 = Decimal::new(425, 1);
        let value1 = "test_value1".to_string();

        let mut element = StreamDataElementType::new(offset1, value1);

        let offset2 = Decimal::new(840, 1);
        let value2 = "test_value2".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        element
            .set_offset(offset2)
            .set_value(value2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(element.offset(), offset2);
        assert_eq!(element.value(), value2);
        assert_eq!(element.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        element.set_custom_data(None);

        assert_eq!(element.custom_data(), None);
    }

    #[test]
    fn test_serialization() {
        let offset = Decimal::new(425, 1);
        let value = "test_value".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let element = StreamDataElementType::new(offset, value).with_custom_data(custom_data);

        let json = serde_json::to_string(&element).unwrap();

        // Check field names are correctly serialized
        assert!(json.contains(r#""t":"#));
        assert!(json.contains(r#""v":"#));
        assert!(json.contains(r#""customData":"#));

        // Check field names are not using internal names
        assert!(!json.contains(r#""offset":"#));
        assert!(!json.contains(r#""value":"#));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"t":42.5,"v":"test_value","customData":{"vendorId":"VendorX"}}"#;

        let element: StreamDataElementType = serde_json::from_str(json).unwrap();

        assert_eq!(element.offset(), Decimal::new(425, 1));
        assert_eq!(element.value(), "test_value");
        assert_eq!(element.custom_data().unwrap().vendor_id(), "VendorX");
    }
}
