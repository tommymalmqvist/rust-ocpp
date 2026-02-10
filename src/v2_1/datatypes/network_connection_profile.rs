use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{apn::APNType, custom_data::CustomDataType, vpn::VPNType};

/// The NetworkConnectionProfile defines the functional and technical parameters of a communication link.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    /// Optional. APN configuration, when using GSM.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub apn: Option<APNType>,

    /// Required. URL of the CSMS(s) that this Charging Station communicates with, without the Charging Station identity part.
    /// The SecurityCtrlr.Identity field is appended to this URL to provide the full websocket URL.
    #[validate(length(max = 2000))]
    pub ocpp_csms_url: String,

    /// Required. Applicable Network Interface. Charging Station is allowed to use a different network interface to connect if the given one does not work.
    /// Allowed values: "Wired0", "Wired1", "Wired2", "Wired3", "Wireless0", "Wireless1", "Wireless2", "Wireless3", "Any"
    #[validate(length(max = 20))]
    pub ocpp_interface: String,

    /// Required. Duration in seconds before a message sent by the Charging Station via this network connection times-out.
    /// The best setting depends on the underlying network and response times of the CSMS. A starting point could be 30 seconds.
    pub message_timeout: i32,

    /// Required. The security profile used when connecting to the CSMS with this NetworkConnectionProfile.
    pub security_profile: i32,

    /// Required. Defines the transport protocol (e.g. SOAP or JSON). Note: SOAP is not supported in OCPP 2.x, but is supported by earlier versions.
    /// Allowed values: "SOAP", "JSON"
    #[validate(length(max = 20))]
    pub ocpp_transport: String,

    /// Required. This field is ignored, since the OCPP version to use is determined during the websocket handshake.
    /// The field is only kept for backwards compatibility with the OCPP 2.0.1 JSON schema.
    /// Allowed values: "OCPP12", "OCPP15", "OCPP16", "OCPP20", "OCPP201", "OCPP21"
    #[validate(length(max = 20))]
    pub ocpp_version: String,

    /// Optional. Charging Station identity to be used as the basic authentication username (specific to OCPP 2.1).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 48))]
    pub identity: Option<String>,

    /// Optional. BasicAuthPassword to use for security profile 1 or 2 (specific to OCPP 2.1).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub basic_auth_password: Option<String>,

    /// Optional. VPN configuration, when using VPN.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub vpn: Option<VPNType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl NetworkConnectionProfileType {
    /// Creates a new `NetworkConnectionProfileType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ocpp_interface` - Applicable Network Interface
    /// * `ocpp_transport` - Transport protocol used by OCPP
    /// * `ocpp_version` - OCPP version used (ignored, determined during websocket handshake)
    /// * `ocpp_csms_url` - URL of the CSMS that this Charging Station communicates with
    /// * `message_timeout` - Duration in seconds before a message times-out
    /// * `security_profile` - The security profile used when connecting to the CSMS
    ///
    /// # Returns
    ///
    /// A new instance of `NetworkConnectionProfileType` with optional fields set to `None`
    pub fn new(
        ocpp_interface: String,
        ocpp_transport: String,
        ocpp_version: String,
        ocpp_csms_url: String,
        message_timeout: i32,
        security_profile: i32,
    ) -> Self {
        Self {
            apn: None,
            ocpp_csms_url,
            ocpp_interface,
            message_timeout,
            security_profile,
            ocpp_transport,
            ocpp_version,
            identity: None,
            basic_auth_password: None,
            vpn: None,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this network connection profile
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the APN configuration.
    ///
    /// # Arguments
    ///
    /// * `apn` - APN configuration, when using GSM
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_apn(mut self, apn: APNType) -> Self {
        self.apn = Some(apn);
        self
    }

    /// Sets the VPN configuration.
    ///
    /// # Arguments
    ///
    /// * `vpn` - VPN configuration, when using VPN
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_vpn(mut self, vpn: VPNType) -> Self {
        self.vpn = Some(vpn);
        self
    }

    /// Sets the identity for basic authentication.
    ///
    /// # Arguments
    ///
    /// * `identity` - Charging Station identity for basic authentication username
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_identity(mut self, identity: String) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Sets the basic authentication password.
    ///
    /// # Arguments
    ///
    /// * `basic_auth_password` - Password for security profile 1 or 2
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_basic_auth_password(mut self, basic_auth_password: String) -> Self {
        self.basic_auth_password = Some(basic_auth_password);
        self
    }

    /// Gets the APN configuration.
    ///
    /// # Returns
    ///
    /// An optional reference to the APN configuration
    pub fn apn(&self) -> Option<&APNType> {
        self.apn.as_ref()
    }

    /// Sets the APN configuration.
    ///
    /// # Arguments
    ///
    /// * `apn` - APN configuration, when using GSM, or None to clear
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_apn(&mut self, apn: Option<APNType>) -> &mut Self {
        self.apn = apn;
        self
    }

    /// Gets the OCPP CSMS URL.
    ///
    /// # Returns
    ///
    /// The URL of the CSMS that this Charging Station communicates with
    pub fn ocpp_csms_url(&self) -> &str {
        &self.ocpp_csms_url
    }

    /// Sets the OCPP CSMS URL.
    ///
    /// # Arguments
    ///
    /// * `ocpp_csms_url` - URL of the CSMS that this Charging Station communicates with
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_ocpp_csms_url(&mut self, ocpp_csms_url: &str) -> &mut Self {
        self.ocpp_csms_url = ocpp_csms_url.to_string();
        self
    }

    /// Gets the OCPP interface.
    ///
    /// # Returns
    ///
    /// The applicable network interface used by OCPP
    pub fn ocpp_interface(&self) -> &str {
        &self.ocpp_interface
    }

    /// Sets the OCPP interface.
    ///
    /// # Arguments
    ///
    /// * `ocpp_interface` - Applicable network interface used by OCPP
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_ocpp_interface(&mut self, ocpp_interface: &str) -> &mut Self {
        self.ocpp_interface = ocpp_interface.to_string();
        self
    }

    /// Gets the message timeout.
    ///
    /// # Returns
    ///
    /// Duration in seconds before a message times-out
    pub fn message_timeout(&self) -> i32 {
        self.message_timeout
    }

    /// Sets the message timeout.
    ///
    /// # Arguments
    ///
    /// * `message_timeout` - Duration in seconds before a message times-out
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_message_timeout(&mut self, message_timeout: i32) -> &mut Self {
        self.message_timeout = message_timeout;
        self
    }

    /// Gets the security profile.
    ///
    /// # Returns
    ///
    /// The security profile used when connecting to the CSMS
    pub fn security_profile(&self) -> i32 {
        self.security_profile
    }

    /// Sets the security profile.
    ///
    /// # Arguments
    ///
    /// * `security_profile` - The security profile used when connecting to the CSMS
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_security_profile(&mut self, security_profile: i32) -> &mut Self {
        self.security_profile = security_profile;
        self
    }

    /// Gets the OCPP transport.
    ///
    /// # Returns
    ///
    /// The transport protocol used by OCPP
    pub fn ocpp_transport(&self) -> &str {
        &self.ocpp_transport
    }

    /// Sets the OCPP transport.
    ///
    /// # Arguments
    ///
    /// * `ocpp_transport` - Transport protocol used by OCPP
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_ocpp_transport(&mut self, ocpp_transport: &str) -> &mut Self {
        self.ocpp_transport = ocpp_transport.to_string();
        self
    }

    /// Gets the OCPP version.
    ///
    /// # Returns
    ///
    /// The OCPP version used (ignored, determined during websocket handshake)
    pub fn ocpp_version(&self) -> &str {
        &self.ocpp_version
    }

    /// Sets the OCPP version.
    ///
    /// # Arguments
    ///
    /// * `ocpp_version` - OCPP version used
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_ocpp_version(&mut self, ocpp_version: &str) -> &mut Self {
        self.ocpp_version = ocpp_version.to_string();
        self
    }

    /// Gets the identity for basic authentication.
    ///
    /// # Returns
    ///
    /// An optional reference to the identity string
    pub fn identity(&self) -> Option<&str> {
        self.identity.as_deref()
    }

    /// Sets the identity for basic authentication.
    ///
    /// # Arguments
    ///
    /// * `identity` - Charging Station identity for basic authentication username, or None to clear
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_identity(&mut self, identity: Option<String>) -> &mut Self {
        self.identity = identity;
        self
    }

    /// Gets the basic authentication password.
    ///
    /// # Returns
    ///
    /// An optional reference to the basic authentication password
    pub fn basic_auth_password(&self) -> Option<&str> {
        self.basic_auth_password.as_deref()
    }

    /// Sets the basic authentication password.
    ///
    /// # Arguments
    ///
    /// * `basic_auth_password` - Password for security profile 1 or 2, or None to clear
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_basic_auth_password(&mut self, basic_auth_password: Option<String>) -> &mut Self {
        self.basic_auth_password = basic_auth_password;
        self
    }

    /// Gets the VPN configuration.
    ///
    /// # Returns
    ///
    /// An optional reference to the VPN configuration
    pub fn vpn(&self) -> Option<&VPNType> {
        self.vpn.as_ref()
    }

    /// Sets the VPN configuration.
    ///
    /// # Arguments
    ///
    /// * `vpn` - VPN configuration, when using VPN, or None to clear
    ///
    /// # Returns
    ///
    /// Mutable reference to self for method chaining
    pub fn set_vpn(&mut self, vpn: Option<VPNType>) -> &mut Self {
        self.vpn = vpn;
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
    /// * `custom_data` - Custom data for this network connection profile, or None to clear
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
    use crate::v2_1::enumerations::vpn::VPNEnumType;
    use crate::v2_1::enumerations::APNAuthenticationEnumType;
    use validator::Validate;
    #[test]
    fn test_new_network_connection_profile() {
        let ocpp_interface = "Wired0".to_string();
        let ocpp_transport = "JSON".to_string();
        let ocpp_version = "OCPP20".to_string();
        let ocpp_csms_url = "https://example.com/csms".to_string();
        let message_timeout = 30;
        let security_profile = 1;

        let profile = NetworkConnectionProfileType::new(
            ocpp_interface.clone(),
            ocpp_transport.clone(),
            ocpp_version.clone(),
            ocpp_csms_url.clone(),
            message_timeout,
            security_profile,
        );

        assert_eq!(profile.ocpp_interface(), ocpp_interface);
        assert_eq!(profile.ocpp_transport(), ocpp_transport);
        assert_eq!(profile.ocpp_version(), ocpp_version);
        assert_eq!(profile.ocpp_csms_url(), ocpp_csms_url);
        assert_eq!(profile.message_timeout(), message_timeout);
        assert_eq!(profile.security_profile(), security_profile);
        assert_eq!(profile.identity(), None);
        assert_eq!(profile.basic_auth_password(), None);
        assert_eq!(profile.custom_data(), None);
        assert_eq!(profile.apn(), None);
        assert_eq!(profile.vpn(), None);
    }

    #[test]
    fn test_with_optional_fields() {
        let ocpp_interface = "Wired0".to_string();
        let ocpp_transport = "JSON".to_string();
        let ocpp_version = "OCPP20".to_string();
        let ocpp_csms_url = "https://example.com/csms".to_string();
        let message_timeout = 30;
        let security_profile = 1;
        let identity = "Station123".to_string();
        let basic_auth_password = "Pass123!".to_string();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let apn = APNType::new("internet".to_string(), APNAuthenticationEnumType::CHAP)
            .with_apn_user_name("user".to_string())
            .with_apn_password("password".to_string())
            .with_sim_pin(1234)
            .with_preferred_network("network".to_string())
            .with_use_only_preferred_network(true);

        let vpn = VPNType::new(
            "vpn.example.com".to_string(),
            "vpnuser".to_string(),
            "vpnpass".to_string(),
            "vpnkey".to_string(),
            VPNEnumType::IKEv2,
        );

        let profile = NetworkConnectionProfileType::new(
            ocpp_interface.clone(),
            ocpp_transport.clone(),
            ocpp_version.clone(),
            ocpp_csms_url.clone(),
            message_timeout,
            security_profile,
        )
        .with_custom_data(custom_data.clone())
        .with_apn(apn.clone())
        .with_vpn(vpn.clone())
        .with_identity(identity.clone())
        .with_basic_auth_password(basic_auth_password.clone());

        assert_eq!(profile.ocpp_interface(), ocpp_interface);
        assert_eq!(profile.ocpp_transport(), ocpp_transport);
        assert_eq!(profile.ocpp_version(), ocpp_version);
        assert_eq!(profile.ocpp_csms_url(), ocpp_csms_url);
        assert_eq!(profile.message_timeout(), message_timeout);
        assert_eq!(profile.security_profile(), security_profile);
        assert_eq!(profile.identity(), Some(identity.as_str()));
        assert_eq!(
            profile.basic_auth_password(),
            Some(basic_auth_password.as_str())
        );
        assert_eq!(profile.custom_data(), Some(&custom_data));
        assert_eq!(profile.apn(), Some(&apn));
        assert_eq!(profile.vpn(), Some(&vpn));
    }

    #[test]
    fn test_setter_methods() {
        let ocpp_interface1 = "Wired0".to_string();
        let ocpp_interface2 = "Wired1".to_string();
        let ocpp_transport1 = "JSON".to_string();
        let ocpp_transport2 = "SOAP".to_string();
        let ocpp_version1 = "OCPP20".to_string();
        let ocpp_version2 = "OCPP16".to_string();
        let ocpp_csms_url1 = "https://example.com/csms".to_string();
        let ocpp_csms_url2 = "https://example.org/csms".to_string();
        let message_timeout1 = 30;
        let message_timeout2 = 60;
        let security_profile1 = 1;
        let security_profile2 = 2;
        let identity = "Station123".to_string();
        let basic_auth_password = "Pass123!".to_string();

        let custom_data = CustomDataType::new("VendorX".to_string());

        let apn = APNType::new("internet".to_string(), APNAuthenticationEnumType::CHAP);

        let vpn = VPNType::new(
            "vpn.example.com".to_string(),
            "vpnuser".to_string(),
            "vpnpass".to_string(),
            "vpnkey".to_string(),
            VPNEnumType::IKEv2,
        );

        let mut profile = NetworkConnectionProfileType::new(
            ocpp_interface1,
            ocpp_transport1,
            ocpp_version1,
            ocpp_csms_url1,
            message_timeout1,
            security_profile1,
        );

        profile
            .set_ocpp_interface(&ocpp_interface2)
            .set_ocpp_transport(&ocpp_transport2)
            .set_ocpp_version(&ocpp_version2)
            .set_ocpp_csms_url(&ocpp_csms_url2)
            .set_message_timeout(message_timeout2)
            .set_security_profile(security_profile2)
            .set_custom_data(Some(custom_data.clone()))
            .set_apn(Some(apn.clone()))
            .set_vpn(Some(vpn.clone()))
            .set_identity(Some(identity.clone()))
            .set_basic_auth_password(Some(basic_auth_password.clone()));

        assert_eq!(profile.ocpp_interface(), ocpp_interface2);
        assert_eq!(profile.ocpp_transport(), ocpp_transport2);
        assert_eq!(profile.ocpp_version(), ocpp_version2);
        assert_eq!(profile.ocpp_csms_url(), ocpp_csms_url2);
        assert_eq!(profile.message_timeout(), message_timeout2);
        assert_eq!(profile.security_profile(), security_profile2);
        assert_eq!(profile.identity(), Some(identity.as_str()));
        assert_eq!(
            profile.basic_auth_password(),
            Some(basic_auth_password.as_str())
        );
        assert_eq!(profile.custom_data(), Some(&custom_data));
        assert_eq!(profile.apn(), Some(&apn));
        assert_eq!(profile.vpn(), Some(&vpn));

        // Test clearing optional fields
        profile
            .set_custom_data(None)
            .set_apn(None)
            .set_vpn(None)
            .set_identity(None)
            .set_basic_auth_password(None);

        assert_eq!(profile.custom_data(), None);
        assert_eq!(profile.apn(), None);
        assert_eq!(profile.vpn(), None);
        assert_eq!(profile.identity(), None);
        assert_eq!(profile.basic_auth_password(), None);
    }

    #[test]
    fn test_validation() {
        // 有效的NetworkConnectionProfileType实例
        let valid_profile = NetworkConnectionProfileType::new(
            "Wired0".to_string(),
            "JSON".to_string(),
            "OCPP20".to_string(),
            "https://example.com/csms".to_string(),
            30,
            1,
        );
        assert!(
            valid_profile.validate().is_ok(),
            "有效的NetworkConnectionProfileType应通过验证"
        );

        // 测试ocpp_csms_url长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.ocpp_csms_url = "a".repeat(2001); // 超过最大长度2000
        assert!(
            invalid_profile.validate().is_err(),
            "ocpp_csms_url过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试ocpp_interface长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.ocpp_interface = "a".repeat(21); // 超过最大长度20
        assert!(
            invalid_profile.validate().is_err(),
            "ocpp_interface过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试ocpp_transport长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.ocpp_transport = "a".repeat(21); // 超过最大长度20
        assert!(
            invalid_profile.validate().is_err(),
            "ocpp_transport过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试ocpp_version长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.ocpp_version = "a".repeat(21); // 超过最大长度20
        assert!(
            invalid_profile.validate().is_err(),
            "ocpp_version过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试identity长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.identity = Some("a".repeat(49)); // 超过最大长度48
        assert!(
            invalid_profile.validate().is_err(),
            "identity过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试basic_auth_password长度验证（过长）
        let mut invalid_profile = valid_profile.clone();
        invalid_profile.basic_auth_password = Some("a".repeat(65)); // 超过最大长度64
        assert!(
            invalid_profile.validate().is_err(),
            "basic_auth_password过长的NetworkConnectionProfileType应验证失败"
        );

        // 测试嵌套验证 - 使用无效的APNType
        let invalid_apn = APNType::new(
            "a".repeat(2001), // 超过最大长度2000
            APNAuthenticationEnumType::CHAP,
        );

        let mut profile_with_invalid_apn = valid_profile.clone();
        profile_with_invalid_apn.apn = Some(invalid_apn);
        assert!(
            profile_with_invalid_apn.validate().is_err(),
            "包含无效APNType的NetworkConnectionProfileType应验证失败"
        );

        // 测试嵌套验证 - 使用无效的VPNType
        let invalid_vpn = VPNType::new(
            "a".repeat(2001), // 超过最大长度2000
            "user".to_string(),
            "password".to_string(),
            "key".to_string(),
            VPNEnumType::IKEv2,
        );

        let mut profile_with_invalid_vpn = valid_profile.clone();
        profile_with_invalid_vpn.vpn = Some(invalid_vpn);
        assert!(
            profile_with_invalid_vpn.validate().is_err(),
            "包含无效VPNType的NetworkConnectionProfileType应验证失败"
        );

        // 测试嵌套验证 - 使用无效的CustomDataType
        let too_long_vendor_id = "X".repeat(256); // 超过255字符限制
        let invalid_custom_data = CustomDataType::new(too_long_vendor_id);

        let mut profile_with_invalid_custom_data = valid_profile.clone();
        profile_with_invalid_custom_data.custom_data = Some(invalid_custom_data);
        assert!(
            profile_with_invalid_custom_data.validate().is_err(),
            "包含无效CustomData的NetworkConnectionProfileType应验证失败"
        );
    }
}
