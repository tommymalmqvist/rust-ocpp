use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Element providing more information about the status.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    /// Required. A predefined code for the reason why the status is returned in this response.
    /// The string is case-insensitive.
    #[validate(length(max = 20))]
    pub reason_code: String,

    /// Optional. Additional text to provide detailed information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 1024))]
    pub additional_info: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl StatusInfoType {
    /// Creates a new `StatusInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `reason_code` - A predefined code for the reason why the status is returned
    ///
    /// # Returns
    ///
    /// A new instance of `StatusInfoType` with optional fields set to `None`
    pub fn new(reason_code: String) -> Self {
        Self {
            reason_code,
            additional_info: None,
            custom_data: None,
        }
    }

    /// Sets the additional info.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Additional text to provide detailed information
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_additional_info(mut self, additional_info: String) -> Self {
        self.additional_info = Some(additional_info);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this status info
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the reason code.
    ///
    /// # Returns
    ///
    /// A reference to the predefined code for the reason why the status is returned
    pub fn reason_code(&self) -> &str {
        &self.reason_code
    }

    /// Sets the reason code.
    ///
    /// # Arguments
    ///
    /// * `reason_code` - A predefined code for the reason why the status is returned
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reason_code(&mut self, reason_code: String) -> &mut Self {
        self.reason_code = reason_code;
        self
    }

    /// Gets the additional info.
    ///
    /// # Returns
    ///
    /// An optional reference to the additional text providing detailed information
    pub fn additional_info(&self) -> Option<&str> {
        self.additional_info.as_deref()
    }

    /// Sets the additional info.
    ///
    /// # Arguments
    ///
    /// * `additional_info` - Additional text to provide detailed information, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_additional_info(&mut self, additional_info: Option<String>) -> &mut Self {
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
    /// * `custom_data` - Custom data for this status info, or None to clear
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
    fn test_new_status_info() {
        let status_info = StatusInfoType::new("SomeReason".to_string());

        assert_eq!(status_info.reason_code(), "SomeReason");
        assert_eq!(status_info.additional_info(), None);
        assert_eq!(status_info.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let status_info = StatusInfoType::new("SomeReason".to_string())
            .with_additional_info("Additional details".to_string())
            .with_custom_data(custom_data.clone());

        assert_eq!(status_info.reason_code(), "SomeReason");
        assert_eq!(status_info.additional_info(), Some("Additional details"));
        assert_eq!(status_info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut status_info = StatusInfoType::new("SomeReason".to_string());

        status_info
            .set_reason_code("OtherReason".to_string())
            .set_additional_info(Some("Additional details".to_string()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(status_info.reason_code(), "OtherReason");
        assert_eq!(status_info.additional_info(), Some("Additional details"));
        assert_eq!(status_info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        status_info.set_additional_info(None).set_custom_data(None);

        assert_eq!(status_info.additional_info(), None);
        assert_eq!(status_info.custom_data(), None);
    }
}
