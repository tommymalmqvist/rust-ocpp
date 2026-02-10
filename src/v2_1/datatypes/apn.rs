use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::enumerations::APNAuthenticationEnumType;

/// Collection of configuration data needed to make a data-connection over a cellular network.
///
/// NOTE: When asking a GSM modem to dial in, it is possible to specify which mobile operator should be used.
/// This can be done with the mobile country code (MCC) in combination with a mobile network code (MNC).
/// Example: If your preferred network is Vodafone Netherlands, the MCC=204 and the MNC=04 which means
/// the key PreferredNetwork = 20404 Some modems allows to specify a preferred network, which means,
/// if this network is not available, a different network is used. If you specify UseOnlyPreferredNetwork
/// and this network is not available, the modem will not dial in.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct APNType {
    /// Required. The Access Point Name as an URL.
    #[validate(length(max = 2000))]
    pub apn: String,

    /// APN username.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 50))]
    pub apn_user_name: Option<String>,

    /// APN Password.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 64))]
    pub apn_password: Option<String>,

    /// SIM card pin code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sim_pin: Option<i32>,

    /// Preferred network, written as MCC and MNC concatenated. See note.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 6))]
    pub preferred_network: Option<String>,

    /// Default: false. Use only the preferred Network, do not dial in when not available. See Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_only_preferred_network: Option<bool>,

    /// Required. Authentication method.
    pub apn_authentication: APNAuthenticationEnumType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl APNType {
    /// Creates a new `APNType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `apn` - The Access Point Name as an URL
    /// * `apn_authentication` - Authentication method
    ///
    /// # Returns
    ///
    /// A new instance of `APNType` with optional fields set to `None`
    pub fn new(apn: String, apn_authentication: APNAuthenticationEnumType) -> Self {
        Self {
            custom_data: None,
            apn,
            apn_user_name: None,
            apn_password: None,
            sim_pin: None,
            preferred_network: None,
            use_only_preferred_network: None,
            apn_authentication,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this APN configuration
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the APN username.
    ///
    /// # Arguments
    ///
    /// * `apn_user_name` - APN username
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_apn_user_name(mut self, apn_user_name: String) -> Self {
        self.apn_user_name = Some(apn_user_name);
        self
    }

    /// Sets the APN password.
    ///
    /// # Arguments
    ///
    /// * `apn_password` - APN password
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_apn_password(mut self, apn_password: String) -> Self {
        self.apn_password = Some(apn_password);
        self
    }

    /// Sets the SIM card PIN code.
    ///
    /// # Arguments
    ///
    /// * `sim_pin` - SIM card PIN code
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_sim_pin(mut self, sim_pin: i32) -> Self {
        self.sim_pin = Some(sim_pin);
        self
    }

    /// Sets the preferred network.
    ///
    /// # Arguments
    ///
    /// * `preferred_network` - Preferred network, written as MCC and MNC concatenated
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_preferred_network(mut self, preferred_network: String) -> Self {
        self.preferred_network = Some(preferred_network);
        self
    }

    /// Sets whether to use only the preferred network.
    ///
    /// # Arguments
    ///
    /// * `use_only_preferred_network` - Whether to use only the preferred network
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_use_only_preferred_network(mut self, use_only_preferred_network: bool) -> Self {
        self.use_only_preferred_network = Some(use_only_preferred_network);
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
    /// * `custom_data` - Custom data for this APN configuration, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the APN.
    ///
    /// # Returns
    ///
    /// The Access Point Name as a string
    pub fn apn(&self) -> &str {
        &self.apn
    }

    /// Sets the APN.
    ///
    /// # Arguments
    ///
    /// * `apn` - The Access Point Name as an URL
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_apn(&mut self, apn: String) -> &mut Self {
        self.apn = apn;
        self
    }

    /// Gets the APN username.
    ///
    /// # Returns
    ///
    /// An optional reference to the APN username
    pub fn apn_user_name(&self) -> Option<&String> {
        self.apn_user_name.as_ref()
    }

    /// Sets the APN username.
    ///
    /// # Arguments
    ///
    /// * `apn_user_name` - APN username, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_apn_user_name(&mut self, apn_user_name: Option<String>) -> &mut Self {
        self.apn_user_name = apn_user_name;
        self
    }

    /// Gets the APN password.
    ///
    /// # Returns
    ///
    /// An optional reference to the APN password
    pub fn apn_password(&self) -> Option<&String> {
        self.apn_password.as_ref()
    }

    /// Sets the APN password.
    ///
    /// # Arguments
    ///
    /// * `apn_password` - APN password, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_apn_password(&mut self, apn_password: Option<String>) -> &mut Self {
        self.apn_password = apn_password;
        self
    }

    /// Gets the SIM card PIN code.
    ///
    /// # Returns
    ///
    /// An optional SIM card PIN code
    pub fn sim_pin(&self) -> Option<i32> {
        self.sim_pin
    }

    /// Sets the SIM card PIN code.
    ///
    /// # Arguments
    ///
    /// * `sim_pin` - SIM card PIN code, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sim_pin(&mut self, sim_pin: Option<i32>) -> &mut Self {
        self.sim_pin = sim_pin;
        self
    }

    /// Gets the preferred network.
    ///
    /// # Returns
    ///
    /// An optional reference to the preferred network
    pub fn preferred_network(&self) -> Option<&String> {
        self.preferred_network.as_ref()
    }

    /// Sets the preferred network.
    ///
    /// # Arguments
    ///
    /// * `preferred_network` - Preferred network, written as MCC and MNC concatenated, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_preferred_network(&mut self, preferred_network: Option<String>) -> &mut Self {
        self.preferred_network = preferred_network;
        self
    }

    /// Gets whether to use only the preferred network.
    ///
    /// # Returns
    ///
    /// An optional boolean indicating whether to use only the preferred network
    pub fn use_only_preferred_network(&self) -> Option<bool> {
        self.use_only_preferred_network
    }

    /// Sets whether to use only the preferred network.
    ///
    /// # Arguments
    ///
    /// * `use_only_preferred_network` - Whether to use only the preferred network, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_use_only_preferred_network(
        &mut self,
        use_only_preferred_network: Option<bool>,
    ) -> &mut Self {
        self.use_only_preferred_network = use_only_preferred_network;
        self
    }

    /// Gets the APN authentication method.
    ///
    /// # Returns
    ///
    /// The APN authentication method
    pub fn apn_authentication(&self) -> APNAuthenticationEnumType {
        self.apn_authentication.clone()
    }

    /// Sets the APN authentication method.
    ///
    /// # Arguments
    ///
    /// * `apn_authentication` - Authentication method
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_apn_authentication(
        &mut self,
        apn_authentication: APNAuthenticationEnumType,
    ) -> &mut Self {
        self.apn_authentication = apn_authentication;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_new_apn() {
        let apn = APNType::new(
            "internet.provider.com".to_string(),
            APNAuthenticationEnumType::CHAP,
        );

        assert_eq!(apn.apn(), "internet.provider.com");
        assert_eq!(apn.apn_authentication(), APNAuthenticationEnumType::CHAP);
        assert_eq!(apn.apn_user_name(), None);
        assert_eq!(apn.apn_password(), None);
        assert_eq!(apn.sim_pin(), None);
        assert_eq!(apn.preferred_network(), None);
        assert_eq!(apn.use_only_preferred_network(), None);
        assert_eq!(apn.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let apn = APNType::new(
            "internet.provider.com".to_string(),
            APNAuthenticationEnumType::CHAP,
        )
        .with_apn_user_name("username".to_string())
        .with_apn_password("password".to_string())
        .with_sim_pin(1234)
        .with_preferred_network("20404".to_string())
        .with_use_only_preferred_network(true)
        .with_custom_data(custom_data.clone());

        assert_eq!(apn.apn(), "internet.provider.com");
        assert_eq!(apn.apn_authentication(), APNAuthenticationEnumType::CHAP);
        assert_eq!(apn.apn_user_name(), Some(&"username".to_string()));
        assert_eq!(apn.apn_password(), Some(&"password".to_string()));
        assert_eq!(apn.sim_pin(), Some(1234));
        assert_eq!(apn.preferred_network(), Some(&"20404".to_string()));
        assert_eq!(apn.use_only_preferred_network(), Some(true));
        assert_eq!(apn.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut apn = APNType::new(
            "internet.provider.com".to_string(),
            APNAuthenticationEnumType::CHAP,
        );

        apn.set_apn("mobile.provider.com".to_string())
            .set_apn_authentication(APNAuthenticationEnumType::PAP)
            .set_apn_user_name(Some("new_username".to_string()))
            .set_apn_password(Some("new_password".to_string()))
            .set_sim_pin(Some(5678))
            .set_preferred_network(Some("31015".to_string()))
            .set_use_only_preferred_network(Some(false))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(apn.apn(), "mobile.provider.com");
        assert_eq!(apn.apn_authentication(), APNAuthenticationEnumType::PAP);
        assert_eq!(apn.apn_user_name(), Some(&"new_username".to_string()));
        assert_eq!(apn.apn_password(), Some(&"new_password".to_string()));
        assert_eq!(apn.sim_pin(), Some(5678));
        assert_eq!(apn.preferred_network(), Some(&"31015".to_string()));
        assert_eq!(apn.use_only_preferred_network(), Some(false));
        assert_eq!(apn.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        apn.set_apn_user_name(None)
            .set_apn_password(None)
            .set_sim_pin(None)
            .set_preferred_network(None)
            .set_use_only_preferred_network(None)
            .set_custom_data(None);

        assert_eq!(apn.apn_user_name(), None);
        assert_eq!(apn.apn_password(), None);
        assert_eq!(apn.sim_pin(), None);
        assert_eq!(apn.preferred_network(), None);
        assert_eq!(apn.use_only_preferred_network(), None);
        assert_eq!(apn.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // 1. Valid APN - should pass validation
        let valid_apn = APNType::new(
            "internet.provider.com".to_string(),
            APNAuthenticationEnumType::CHAP,
        )
        .with_apn_user_name("username".to_string())
        .with_apn_password("password".to_string())
        .with_preferred_network("20404".to_string());

        assert!(
            valid_apn.validate().is_ok(),
            "Valid APN should pass validation"
        );

        // 2. Test apn length validation (too long)
        let long_apn = "a".repeat(2001); // 2001 characters, exceeds max of 2000
        let mut invalid_apn = valid_apn.clone();
        invalid_apn.apn = long_apn;

        assert!(
            invalid_apn.validate().is_err(),
            "APN with too long apn should fail validation"
        );
        let error = invalid_apn.validate().unwrap_err();
        assert!(
            error.to_string().contains("apn"),
            "Error should mention apn: {}",
            error
        );

        // 3. Test apn_user_name length validation (too long)
        let long_username = "a".repeat(51); // 51 characters, exceeds max of 50
        let mut invalid_apn = valid_apn.clone();
        invalid_apn.apn_user_name = Some(long_username);

        assert!(
            invalid_apn.validate().is_err(),
            "APN with too long username should fail validation"
        );
        let error = invalid_apn.validate().unwrap_err();
        assert!(
            error.to_string().contains("apn_user_name"),
            "Error should mention apn_user_name: {}",
            error
        );

        // 4. Test apn_password length validation (too long)
        let long_password = "a".repeat(65); // 65 characters, exceeds max of 64
        let mut invalid_apn = valid_apn.clone();
        invalid_apn.apn_password = Some(long_password);

        assert!(
            invalid_apn.validate().is_err(),
            "APN with too long password should fail validation"
        );
        let error = invalid_apn.validate().unwrap_err();
        assert!(
            error.to_string().contains("apn_password"),
            "Error should mention apn_password: {}",
            error
        );

        // 5. Test preferred_network length validation (too long)
        let long_network = "1234567".to_string(); // 7 characters, exceeds max of 6
        let mut invalid_apn = valid_apn.clone();
        invalid_apn.preferred_network = Some(long_network);

        assert!(
            invalid_apn.validate().is_err(),
            "APN with too long preferred_network should fail validation"
        );
        let error = invalid_apn.validate().unwrap_err();
        assert!(
            error.to_string().contains("preferred_network"),
            "Error should mention preferred_network: {}",
            error
        );

        // 6. Test custom_data nested validation
        let mut invalid_custom_data = CustomDataType::new("VendorX".to_string());
        // Set an invalid vendor_id (too long) by bypassing the setter
        invalid_custom_data.vendor_id = "A".repeat(256); // Max length is 255

        let mut invalid_apn = valid_apn.clone();
        invalid_apn.custom_data = Some(invalid_custom_data);

        assert!(
            invalid_apn.validate().is_err(),
            "APN with invalid custom_data should fail validation"
        );
        let error = invalid_apn.validate().unwrap_err();
        assert!(
            error.to_string().contains("custom_data"),
            "Error should mention custom_data: {}",
            error
        );
    }
}
