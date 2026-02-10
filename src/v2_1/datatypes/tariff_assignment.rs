use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;

/// Tariff assignment to a charging profile.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffAssignmentType {
    /// Required. Unique identifier used to identify one or more tariffs.
    #[validate(length(max = 60))]
    pub tariff_id: String,

    /// Required. Start date and time of the tariff assignment.
    pub start_date_time: DateTime<Utc>,

    /// Optional. End date and time of the tariff assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<DateTime<Utc>>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffAssignmentType {
    /// Creates a new `TariffAssignmentType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier used to identify one or more tariffs
    /// * `start_date_time` - Start date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// A new instance of `TariffAssignmentType` with optional fields set to `None`
    pub fn new(tariff_id: String, start_date_time: DateTime<Utc>) -> Self {
        Self {
            tariff_id,
            start_date_time,
            expiry_date_time: None,
            custom_data: None,
        }
    }

    /// Sets the expiry date and time.
    ///
    /// # Arguments
    ///
    /// * `expiry_date_time` - End date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_expiry_date_time(mut self, expiry_date_time: DateTime<Utc>) -> Self {
        self.expiry_date_time = Some(expiry_date_time);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the tariff identifier.
    ///
    /// # Returns
    ///
    /// The unique identifier used to identify one or more tariffs
    pub fn tariff_id(&self) -> &str {
        &self.tariff_id
    }

    /// Sets the tariff identifier.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique identifier used to identify one or more tariffs
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: String) -> &mut Self {
        self.tariff_id = tariff_id;
        self
    }

    /// Gets the start date and time.
    ///
    /// # Returns
    ///
    /// The start date and time of the tariff assignment
    pub fn start_date_time(&self) -> &DateTime<Utc> {
        &self.start_date_time
    }

    /// Sets the start date and time.
    ///
    /// # Arguments
    ///
    /// * `start_date_time` - Start date and time of the tariff assignment
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_date_time(&mut self, start_date_time: DateTime<Utc>) -> &mut Self {
        self.start_date_time = start_date_time;
        self
    }

    /// Gets the expiry date and time.
    ///
    /// # Returns
    ///
    /// An optional reference to the end date and time of the tariff assignment
    pub fn expiry_date_time(&self) -> Option<&DateTime<Utc>> {
        self.expiry_date_time.as_ref()
    }

    /// Sets the expiry date and time.
    ///
    /// # Arguments
    ///
    /// * `expiry_date_time` - End date and time of the tariff assignment, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_expiry_date_time(&mut self, expiry_date_time: Option<DateTime<Utc>>) -> &mut Self {
        self.expiry_date_time = expiry_date_time;
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
    /// * `custom_data` - Custom data for this tariff assignment, or None to clear
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
    use chrono::TimeZone;

    #[test]
    fn test_new_tariff_assignment() {
        let tariff_id = "tariff-123".to_string();
        let start_date_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

        let tariff_assignment = TariffAssignmentType::new(tariff_id.clone(), start_date_time);

        assert_eq!(tariff_assignment.tariff_id(), tariff_id);
        assert_eq!(tariff_assignment.start_date_time(), &start_date_time);
        assert_eq!(tariff_assignment.expiry_date_time(), None);
        assert_eq!(tariff_assignment.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let tariff_id = "tariff-123".to_string();
        let start_date_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let expiry_date_time = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff_assignment = TariffAssignmentType::new(tariff_id.clone(), start_date_time)
            .with_expiry_date_time(expiry_date_time)
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff_assignment.tariff_id(), tariff_id);
        assert_eq!(tariff_assignment.start_date_time(), &start_date_time);
        assert_eq!(
            tariff_assignment.expiry_date_time().unwrap(),
            &expiry_date_time
        );
        assert_eq!(tariff_assignment.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let tariff_id1 = "tariff-123".to_string();
        let start_date_time1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let tariff_id2 = "tariff-456".to_string();
        let start_date_time2 = Utc.with_ymd_and_hms(2023, 2, 1, 0, 0, 0).unwrap();
        let expiry_date_time = Utc.with_ymd_and_hms(2023, 12, 31, 23, 59, 59).unwrap();
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff_assignment = TariffAssignmentType::new(tariff_id1, start_date_time1);

        tariff_assignment
            .set_tariff_id(tariff_id2.clone())
            .set_start_date_time(start_date_time2)
            .set_expiry_date_time(Some(expiry_date_time))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff_assignment.tariff_id(), tariff_id2);
        assert_eq!(tariff_assignment.start_date_time(), &start_date_time2);
        assert_eq!(
            tariff_assignment.expiry_date_time().unwrap(),
            &expiry_date_time
        );
        assert_eq!(tariff_assignment.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        tariff_assignment
            .set_expiry_date_time(None)
            .set_custom_data(None);

        assert_eq!(tariff_assignment.expiry_date_time(), None);
        assert_eq!(tariff_assignment.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Test with valid tariff assignment
        let tariff_id = "tariff-123".to_string();
        let start_date_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let tariff_assignment = TariffAssignmentType::new(tariff_id, start_date_time);

        assert!(tariff_assignment.validate().is_ok());

        // Test with tariff_id that exceeds max length (60 characters)
        let long_id = "a".repeat(61);
        let invalid_assignment = TariffAssignmentType::new(long_id, start_date_time);

        assert!(invalid_assignment.validate().is_err());
    }
}
