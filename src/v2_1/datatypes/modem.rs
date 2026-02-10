use super::custom_data::CustomDataType;
use crate::v2_1::helpers::validator::validate_identifier_string;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Defines parameters required for initiating and maintaining wireless communication with other devices.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ModemType {
    /// Required. This contains the ICCID of the modem's SIM card.
    #[validate(length(max = 20), custom(function = "validate_identifier_string"))]
    pub iccid: String,

    /// Required. This contains the IMSI of the modem's SIM card.
    #[validate(length(max = 20), custom(function = "validate_identifier_string"))]
    pub imsi: String,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ModemType {
    /// Creates a new `ModemType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `iccid` - ICCID of the modem's SIM card
    /// * `imsi` - IMSI of the modem's SIM card
    ///
    /// # Returns
    ///
    /// A new instance of `ModemType` with optional fields set to `None`
    pub fn new(iccid: String, imsi: String) -> Self {
        Self {
            iccid,
            imsi,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this modem
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the ICCID.
    ///
    /// # Returns
    ///
    /// A reference to the ICCID of the modem's SIM card
    pub fn iccid(&self) -> &str {
        &self.iccid
    }

    /// Sets the ICCID.
    ///
    /// # Arguments
    ///
    /// * `iccid` - ICCID of the modem's SIM card
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_iccid(&mut self, iccid: &str) -> &mut Self {
        self.iccid = iccid.to_string();
        self
    }

    /// Gets the IMSI.
    ///
    /// # Returns
    ///
    /// A reference to the IMSI of the modem's SIM card
    pub fn imsi(&self) -> &str {
        &self.imsi
    }

    /// Sets the IMSI.
    ///
    /// # Arguments
    ///
    /// * `imsi` - IMSI of the modem's SIM card
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_imsi(&mut self, imsi: &str) -> &mut Self {
        self.imsi = imsi.to_string();
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
    /// * `custom_data` - Custom data for this modem, or None to clear
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_modem() {
        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        assert_eq!(modem.iccid(), "12345678901234567890");
        assert_eq!(modem.imsi(), "123456789012345");
        assert_eq!(modem.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        )
        .with_custom_data(custom_data.clone());

        assert_eq!(modem.iccid(), "12345678901234567890");
        assert_eq!(modem.imsi(), "123456789012345");
        assert_eq!(modem.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );

        modem
            .set_iccid("09876543210987654321")
            .set_imsi("543210987654321")
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(modem.iccid(), "09876543210987654321");
        assert_eq!(modem.imsi(), "543210987654321");
        assert_eq!(modem.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        modem.set_custom_data(None);
        assert_eq!(modem.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // 有效的ModemType实例
        let valid_modem = ModemType::new(
            "12345678901234567890".to_string(),
            "123456789012345".to_string(),
        );
        assert!(valid_modem.validate().is_ok(), "有效的ModemType应通过验证");

        // 测试ICCID长度验证（过长）
        let mut invalid_modem = valid_modem.clone();
        invalid_modem.iccid = "a".repeat(21); // 超过最大长度20
        assert!(
            invalid_modem.validate().is_err(),
            "ICCID过长的ModemType应验证失败"
        );

        // 测试IMSI长度验证（过长）
        let mut invalid_modem = valid_modem.clone();
        invalid_modem.imsi = "a".repeat(21); // 超过最大长度20
        assert!(
            invalid_modem.validate().is_err(),
            "IMSI过长的ModemType应验证失败"
        );

        // 测试ICCID标识符字符串验证
        let mut invalid_modem = valid_modem.clone();
        invalid_modem.iccid = "invalid/character".to_string(); // 包含无效字符'/'
        assert!(
            invalid_modem.validate().is_err(),
            "包含无效字符的ICCID应验证失败"
        );

        // 测试IMSI标识符字符串验证
        let mut invalid_modem = valid_modem.clone();
        invalid_modem.imsi = "invalid character".to_string(); // 包含空格
        assert!(
            invalid_modem.validate().is_err(),
            "包含空格的IMSI应验证失败"
        );

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let mut modem_with_invalid_custom_data = valid_modem.clone();
        modem_with_invalid_custom_data.custom_data = Some(invalid_custom_data);
        assert!(
            modem_with_invalid_custom_data.validate().is_err(),
            "包含无效CustomData的ModemType应验证失败"
        );
    }
}
