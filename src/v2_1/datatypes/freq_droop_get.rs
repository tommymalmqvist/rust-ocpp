use super::super::helpers::validator::validate_identifier_string;
use super::{custom_data::CustomDataType, freq_droop::FreqDroopType};
use serde::{Deserialize, Serialize};
use std::fmt;
use validator::Validate;

/// Frequency droop get type for retrieving frequency droop settings.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FreqDroopGetType {
    /// The frequency droop settings.
    #[validate(nested)]
    pub freq_droop: FreqDroopType,

    /// Id of the setting.
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub id: String,

    /// True if this setting is superseded by a higher priority setting (i.e. lower value of priority).
    pub is_superseded: bool,

    /// True if this is a default setting.
    pub is_default: bool,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl fmt::Display for FreqDroopGetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FreqDroopGet {{ id: {}, is_default: {}, is_superseded: {} }}",
            self.id, self.is_default, self.is_superseded
        )
    }
}

impl From<FreqDroopType> for FreqDroopGetType {
    fn from(freq_droop: FreqDroopType) -> Self {
        Self {
            freq_droop,
            id: String::new(),
            is_superseded: false,
            is_default: false,
            custom_data: None,
        }
    }
}

impl FreqDroopGetType {
    /// Creates a new `FreqDroopGetType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `freq_droop` - The frequency droop settings
    /// * `id` - Id of the setting
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    /// * `is_default` - True if this is a default setting
    ///
    /// # Returns
    ///
    /// A new instance of `FreqDroopGetType` with optional fields set to `None`
    pub fn new(
        freq_droop: FreqDroopType,
        id: String,
        is_superseded: bool,
        is_default: bool,
    ) -> Self {
        Self {
            freq_droop,
            id,
            is_superseded,
            is_default,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this frequency droop get
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the frequency droop settings.
    ///
    /// # Returns
    ///
    /// A reference to the frequency droop settings
    pub fn freq_droop(&self) -> &FreqDroopType {
        &self.freq_droop
    }

    /// Sets the frequency droop settings.
    ///
    /// # Arguments
    ///
    /// * `freq_droop` - The frequency droop settings
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_freq_droop(&mut self, freq_droop: FreqDroopType) -> &mut Self {
        self.freq_droop = freq_droop;
        self
    }

    /// Gets the ID of the setting.
    ///
    /// # Returns
    ///
    /// A reference to the ID of the setting
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Sets the ID of the setting.
    ///
    /// # Arguments
    ///
    /// * `id` - Id of the setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: String) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets whether this setting is superseded.
    ///
    /// # Returns
    ///
    /// True if this setting is superseded by a higher priority setting
    pub fn is_superseded(&self) -> bool {
        self.is_superseded
    }

    /// Sets whether this setting is superseded.
    ///
    /// # Arguments
    ///
    /// * `is_superseded` - True if this setting is superseded by a higher priority setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_superseded(&mut self, is_superseded: bool) -> &mut Self {
        self.is_superseded = is_superseded;
        self
    }

    /// Gets whether this is a default setting.
    ///
    /// # Returns
    ///
    /// True if this is a default setting
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Sets whether this is a default setting.
    ///
    /// # Arguments
    ///
    /// * `is_default` - True if this is a default setting
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_is_default(&mut self, is_default: bool) -> &mut Self {
        self.is_default = is_default;
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
    /// * `custom_data` - Custom data for this frequency droop get, or None to clear
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
    use rust_decimal::prelude::FromStr;

    #[test]
    fn test_new_freq_droop_get() {
        let over_freq = rust_decimal::Decimal::from_str("50.5").unwrap();
        let under_freq = rust_decimal::Decimal::from_str("49.5").unwrap();
        let over_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let under_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let response_time = rust_decimal::Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            1,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;

        let freq_droop_get =
            FreqDroopGetType::new(freq_droop.clone(), id.clone(), is_superseded, is_default);

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop);
        assert_eq!(freq_droop_get.id(), id);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded);
        assert_eq!(freq_droop_get.is_default(), is_default);
        assert_eq!(freq_droop_get.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let over_freq = rust_decimal::Decimal::from_str("50.5").unwrap();
        let under_freq = rust_decimal::Decimal::from_str("49.5").unwrap();
        let over_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let under_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let response_time = rust_decimal::Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            1,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );
        let id = "setting1".to_string();
        let is_superseded = false;
        let is_default = true;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let freq_droop_get =
            FreqDroopGetType::new(freq_droop.clone(), id.clone(), is_superseded, is_default)
                .with_custom_data(custom_data.clone());

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop);
        assert_eq!(freq_droop_get.id(), id);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded);
        assert_eq!(freq_droop_get.is_default(), is_default);
        assert_eq!(freq_droop_get.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let over_freq1 = rust_decimal::Decimal::from_str("50.5").unwrap();
        let under_freq1 = rust_decimal::Decimal::from_str("49.5").unwrap();
        let over_droop1 = rust_decimal::Decimal::from_str("0.04").unwrap();
        let under_droop1 = rust_decimal::Decimal::from_str("0.04").unwrap();
        let response_time1 = rust_decimal::Decimal::from_str("2.0").unwrap();

        let over_freq2 = rust_decimal::Decimal::from_str("50.6").unwrap();
        let under_freq2 = rust_decimal::Decimal::from_str("49.4").unwrap();
        let over_droop2 = rust_decimal::Decimal::from_str("0.05").unwrap();
        let under_droop2 = rust_decimal::Decimal::from_str("0.05").unwrap();
        let response_time2 = rust_decimal::Decimal::from_str("3.0").unwrap();

        let freq_droop1 = FreqDroopType::new(
            1,
            over_freq1,
            under_freq1,
            over_droop1,
            under_droop1,
            response_time1,
        );
        let freq_droop2 = FreqDroopType::new(
            2,
            over_freq2,
            under_freq2,
            over_droop2,
            under_droop2,
            response_time2,
        );
        let id1 = "setting1".to_string();
        let id2 = "setting2".to_string();
        let is_superseded1 = false;
        let is_superseded2 = true;
        let is_default1 = true;
        let is_default2 = false;
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut freq_droop_get = FreqDroopGetType::new(
            freq_droop1.clone(),
            id1.clone(),
            is_superseded1,
            is_default1,
        );

        freq_droop_get
            .set_freq_droop(freq_droop2.clone())
            .set_id(id2.clone())
            .set_is_superseded(is_superseded2)
            .set_is_default(is_default2)
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop2);
        assert_eq!(freq_droop_get.id(), id2);
        assert_eq!(freq_droop_get.is_superseded(), is_superseded2);
        assert_eq!(freq_droop_get.is_default(), is_default2);
        assert_eq!(freq_droop_get.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        freq_droop_get.set_custom_data(None);
        assert_eq!(freq_droop_get.custom_data(), None);
    }

    #[test]
    fn test_from_freq_droop() {
        let over_freq = rust_decimal::Decimal::from_str("50.5").unwrap();
        let under_freq = rust_decimal::Decimal::from_str("49.5").unwrap();
        let over_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let under_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let response_time = rust_decimal::Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            1,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );
        let freq_droop_get = FreqDroopGetType::from(freq_droop.clone());

        assert_eq!(freq_droop_get.freq_droop(), &freq_droop);
        assert_eq!(freq_droop_get.id(), "");
        assert_eq!(freq_droop_get.is_superseded(), false);
        assert_eq!(freq_droop_get.is_default(), false);
        assert_eq!(freq_droop_get.custom_data(), None);
    }

    #[test]
    fn test_display() {
        let over_freq = rust_decimal::Decimal::from_str("50.5").unwrap();
        let under_freq = rust_decimal::Decimal::from_str("49.5").unwrap();
        let over_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let under_droop = rust_decimal::Decimal::from_str("0.04").unwrap();
        let response_time = rust_decimal::Decimal::from_str("2.0").unwrap();

        let freq_droop = FreqDroopType::new(
            1,
            over_freq,
            under_freq,
            over_droop,
            under_droop,
            response_time,
        );
        let id = "setting1".to_string();
        let is_superseded = true;
        let is_default = false;

        let freq_droop_get = FreqDroopGetType::new(freq_droop, id, is_superseded, is_default);

        let display_string = format!("{}", freq_droop_get);
        assert_eq!(
            display_string,
            "FreqDroopGet { id: setting1, is_default: false, is_superseded: true }"
        );
    }
}
