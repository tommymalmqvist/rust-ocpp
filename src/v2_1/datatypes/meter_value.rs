use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sampled_value::SampledValueType};

/// Collection of one or more sampled values in MeterValuesRequest and StopTransactionRequest.
/// All sampled values in a MeterValue are sampled at the same point in time.
///
/// This type is used to represent meter readings from a Charging Station. Each meter value
/// contains a timestamp and one or more sampled values, all taken at the same point in time.
/// The sampled values can represent different types of measurements (energy, power, voltage, etc.)
/// from different locations within the Charging Station.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MeterValueType {
    /// Required. Timestamp for measured value(s).
    ///
    /// This is the exact time when the meter values were sampled.
    pub timestamp: DateTime<Utc>,

    /// Required. One or more measured values.
    ///
    /// This vector must contain at least one sampled value as per the OCPP 2.1 specification.
    /// All values in this vector are sampled at the same point in time (specified by the timestamp).
    #[validate(length(min = 1))]
    #[validate(nested)]
    pub sampled_value: Vec<SampledValueType>,

    /// Custom data from the Charging Station.
    ///
    /// This field can be used to include vendor-specific information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MeterValueType {
    /// Creates a new `MeterValueType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp for measured value(s)
    /// * `sampled_value` - One or more measured values (must contain at least one element)
    ///
    /// # Returns
    ///
    /// A new instance of `MeterValueType` with optional fields set to `None`
    ///
    /// # Example
    ///
    /// ```
    /// use rust_ocpp::v2_1::datatypes::meter_value::MeterValueType;
    /// use rust_ocpp::v2_1::datatypes::sampled_value::SampledValueType;
    /// use chrono::Utc;
    ///
    /// let sampled_value = vec![SampledValueType::new(42.0)];
    /// let meter_value = MeterValueType::new(Utc::now(), sampled_value);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if `sampled_value` is empty, as the OCPP 2.1 specification
    /// requires at least one sampled value.
    pub fn new(timestamp: DateTime<Utc>, sampled_value: Vec<SampledValueType>) -> Self {
        assert!(
            !sampled_value.is_empty(),
            "sampled_value must contain at least one element"
        );
        Self {
            timestamp,
            sampled_value,
            custom_data: None,
        }
    }

    /// Creates a builder for `MeterValueType` with required fields.
    ///
    /// This is an alternative to using `new()` followed by `with_*` methods.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp for measured value(s)
    /// * `sampled_value` - One or more measured values (must contain at least one element)
    ///
    /// # Returns
    ///
    /// A new instance of `MeterValueType` with optional fields set to `None`
    ///
    /// # Panics
    ///
    /// This function will panic if `sampled_value` is empty, as the OCPP 2.1 specification
    /// requires at least one sampled value.
    pub fn builder(timestamp: DateTime<Utc>, sampled_value: Vec<SampledValueType>) -> Self {
        Self::new(timestamp, sampled_value)
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this MeterValue
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    ///
    /// # Example
    ///
    /// ```
    /// use rust_ocpp::v2_1::datatypes::meter_value::MeterValueType;
    /// use rust_ocpp::v2_1::datatypes::sampled_value::SampledValueType;
    /// use rust_ocpp::v2_1::datatypes::custom_data::CustomDataType;
    /// use chrono::Utc;
    ///
    /// let sampled_value = vec![SampledValueType::new(42.0)];
    /// let custom_data = CustomDataType::new("VendorX".to_string());
    /// let meter_value = MeterValueType::new(Utc::now(), sampled_value)
    ///     .with_custom_data(custom_data);
    /// ```
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the timestamp.
    ///
    /// # Returns
    ///
    /// The timestamp for measured value(s)
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Sets the timestamp.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - Timestamp for measured value(s)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_timestamp(&mut self, timestamp: DateTime<Utc>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    /// Gets the sampled values.
    ///
    /// # Returns
    ///
    /// Reference to the vector of sampled values
    pub fn sampled_value(&self) -> &[SampledValueType] {
        &self.sampled_value
    }

    /// Sets the sampled values.
    ///
    /// # Arguments
    ///
    /// * `sampled_value` - Vector of sampled values (must contain at least one element)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    ///
    /// # Panics
    ///
    /// This function will panic if `sampled_value` is empty, as the OCPP 2.1 specification
    /// requires at least one sampled value.
    pub fn set_sampled_value(&mut self, sampled_value: Vec<SampledValueType>) -> &mut Self {
        assert!(
            !sampled_value.is_empty(),
            "sampled_value must contain at least one element"
        );
        self.sampled_value = sampled_value;
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
    /// * `custom_data` - Custom data for this MeterValue, or None to clear
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
    use crate::v2_1::enumerations::{
        LocationEnumType, MeasurandEnumType, PhaseEnumType, ReadingContextEnumType,
    };
    use serde_json::{json, Value};

    #[test]
    fn test_new_meter_value() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)];

        let meter_value = MeterValueType::new(timestamp, sampled_value.clone());

        assert_eq!(meter_value.timestamp(), timestamp);
        assert_eq!(meter_value.sampled_value(), &sampled_value);
        assert_eq!(meter_value.custom_data(), None);
    }

    #[test]
    #[should_panic(expected = "sampled_value must contain at least one element")]
    fn test_new_meter_value_empty_sampled_value() {
        let timestamp = Utc::now();
        let sampled_value = vec![];

        // This should panic because sampled_value is empty
        let _meter_value = MeterValueType::new(timestamp, sampled_value);
    }

    #[test]
    fn test_builder() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)];

        let meter_value = MeterValueType::builder(timestamp, sampled_value.clone());

        assert_eq!(meter_value.timestamp(), timestamp);
        assert_eq!(meter_value.sampled_value(), &sampled_value);
        assert_eq!(meter_value.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let meter_value = MeterValueType::new(timestamp, sampled_value.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(meter_value.timestamp(), timestamp);
        assert_eq!(meter_value.sampled_value(), &sampled_value);
        assert_eq!(meter_value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let timestamp1 = Utc::now();
        let timestamp2 = timestamp1 + chrono::Duration::seconds(60);

        let sampled_value1 = vec![SampledValueType::new(42.0)];
        let sampled_value2 = vec![SampledValueType::new(50.0)];
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut meter_value = MeterValueType::new(timestamp1, sampled_value1.clone());

        meter_value
            .set_timestamp(timestamp2)
            .set_sampled_value(sampled_value2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(meter_value.timestamp(), timestamp2);
        assert_eq!(meter_value.sampled_value(), &sampled_value2);
        assert_eq!(meter_value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        meter_value.set_custom_data(None);
        assert_eq!(meter_value.custom_data(), None);
    }

    #[test]
    #[should_panic(expected = "sampled_value must contain at least one element")]
    fn test_set_sampled_value_empty() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)];
        let mut meter_value = MeterValueType::new(timestamp, sampled_value);

        // This should panic because sampled_value is empty
        meter_value.set_sampled_value(vec![]);
    }

    #[test]
    fn test_validation_success() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)];
        let meter_value = MeterValueType::new(timestamp, sampled_value);

        // Validation should pass
        assert!(meter_value.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_sampled_value() {
        // Create a meter value with an empty sampled_value vector
        // We need to bypass the constructor's assertion to test validation
        let meter_value = MeterValueType {
            timestamp: Utc::now(),
            sampled_value: vec![],
            custom_data: None,
        };

        // Validation should fail
        let result = meter_value.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("sampled_value"));
    }

    #[test]
    fn test_validation_nested_fields() {
        // Create a meter value with invalid sampled value (empty array)
        let meter_value = MeterValueType {
            timestamp: Utc::now(),
            sampled_value: vec![],
            custom_data: None,
        };

        // Validation should fail due to empty sampled_value array
        let result = meter_value.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("sampled_value"));
    }

    #[test]
    fn test_serialization() {
        let timestamp = Utc::now();
        let sampled_value = vec![SampledValueType::new(42.0)
            .with_context(ReadingContextEnumType::SamplePeriodic)
            .with_measurand(MeasurandEnumType::CurrentImport)
            .with_phase(PhaseEnumType::L1)
            .with_location(LocationEnumType::Outlet)];
        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let meter_value =
            MeterValueType::new(timestamp, sampled_value).with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&meter_value).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();

        // Check that fields are correctly serialized
        assert!(deserialized["timestamp"].is_string());
        assert!(deserialized["sampledValue"].is_array());
        assert_eq!(deserialized["sampledValue"][0]["value"], 42.0);
        assert_eq!(
            deserialized["sampledValue"][0]["context"],
            "Sample.Periodic"
        );
        assert_eq!(
            deserialized["sampledValue"][0]["measurand"],
            "Current.Import"
        );
        assert_eq!(deserialized["sampledValue"][0]["phase"], "L1");
        assert_eq!(deserialized["sampledValue"][0]["location"], "Outlet");
        assert_eq!(deserialized["customData"]["vendorId"], "VendorX");
        assert_eq!(deserialized["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization() {
        // Create JSON with all fields
        let json_with_all_fields = json!({
            "timestamp": "2023-01-01T12:00:00Z",
            "sampledValue": [
                {
                    "value": 42.0,
                    "context": "Sample.Periodic",
                    "measurand": "Current.Import",
                    "phase": "L1",
                    "location": "Outlet"
                },
                {
                    "value": 50.0,
                    "measurand": "Voltage"
                }
            ],
            "customData": {
                "vendorId": "VendorX"
            }
        });

        // Deserialize
        let meter_value: MeterValueType = serde_json::from_value(json_with_all_fields).unwrap();

        // Check fields
        assert_eq!(
            meter_value.timestamp().to_rfc3339(),
            "2023-01-01T12:00:00+00:00"
        );
        assert_eq!(meter_value.sampled_value().len(), 2);
        assert_eq!(meter_value.sampled_value()[0].value(), 42.0);
        assert_eq!(meter_value.sampled_value()[1].value(), 50.0);
        assert!(meter_value.custom_data().is_some());
        assert_eq!(meter_value.custom_data().unwrap().vendor_id(), "VendorX");

        // Create JSON with only required fields
        let json_required_only = json!({
            "timestamp": "2023-01-01T12:00:00Z",
            "sampledValue": [
                {
                    "value": 42.0
                }
            ]
        });

        // Deserialize
        let meter_value: MeterValueType = serde_json::from_value(json_required_only).unwrap();

        // Check fields
        assert_eq!(
            meter_value.timestamp().to_rfc3339(),
            "2023-01-01T12:00:00+00:00"
        );
        assert_eq!(meter_value.sampled_value().len(), 1);
        assert_eq!(meter_value.sampled_value()[0].value(), 42.0);
        assert!(meter_value.custom_data().is_none());
    }

    #[test]
    fn test_multiple_sampled_values() {
        let timestamp = Utc::now();
        let sampled_values = vec![
            SampledValueType::new(42.0)
                .with_measurand(MeasurandEnumType::EnergyActiveImportRegister),
            SampledValueType::new(230.0).with_measurand(MeasurandEnumType::Voltage),
            SampledValueType::new(10.5).with_measurand(MeasurandEnumType::CurrentImport),
        ];

        let meter_value = MeterValueType::new(timestamp, sampled_values.clone());

        assert_eq!(meter_value.sampled_value().len(), 3);
        assert_eq!(meter_value.sampled_value()[0].value(), 42.0);
        assert_eq!(meter_value.sampled_value()[1].value(), 230.0);
        assert_eq!(meter_value.sampled_value()[2].value(), 10.5);
    }
}
