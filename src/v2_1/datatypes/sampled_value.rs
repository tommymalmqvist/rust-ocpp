use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, signed_meter_value::SignedMeterValueType,
    unit_of_measure::UnitOfMeasureType,
};
use crate::v2_1::enumerations::{
    LocationEnumType, MeasurandEnumType, PhaseEnumType, ReadingContextEnumType,
};

/// Single sampled value in MeterValues. Each value can be accompanied by optional fields.
///
/// To save on mobile data usage, default values of all of the optional fields are such that.
/// The value without any additional fields will be interpreted, as a register reading of active
/// import energy in Wh (Watt-hour) units.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SampledValueType {
    /// Required. Indicates the measured value.
    pub value: f64,

    /// Optional. Type of measurement value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnumType>,

    /// Optional. Type of detail value: start, end or sample.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnumType>,

    /// Optional. Phase as measured or assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnumType>,

    /// Optional. Location of measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnumType>,

    /// Optional. Contains the signed version of the meter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub signed_meter_value: Option<SignedMeterValueType>,

    /// Optional. Unit of the measured value.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub unit_of_measure: Option<UnitOfMeasureType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SampledValueType {
    /// Creates a new `SampledValueType` with the required value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value as a floating-point number
    ///
    /// # Returns
    ///
    /// A new instance of `SampledValueType` with optional fields set to `None`
    pub fn new(value: f64) -> Self {
        Self {
            value,
            measurand: None,
            context: None,
            phase: None,
            location: None,
            signed_meter_value: None,
            unit_of_measure: None,
            custom_data: None,
        }
    }

    /// Gets the value.
    ///
    /// # Returns
    ///
    /// The value as a floating-point number
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the value.
    ///
    /// # Arguments
    ///
    /// * `value` - Value as a floating-point number
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_value(&mut self, value: f64) -> &mut Self {
        self.value = value;
        self
    }

    /// Gets the measurand.
    ///
    /// # Returns
    ///
    /// An optional type of measurement value
    pub fn measurand(&self) -> Option<&MeasurandEnumType> {
        self.measurand.as_ref()
    }

    /// Sets the measurand.
    ///
    /// # Arguments
    ///
    /// * `measurand` - Type of measurement value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_measurand(&mut self, measurand: Option<MeasurandEnumType>) -> &mut Self {
        self.measurand = measurand;
        self
    }

    /// Sets the measurand.
    ///
    /// # Arguments
    ///
    /// * `measurand` - Type of measurement value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_measurand(mut self, measurand: MeasurandEnumType) -> Self {
        self.measurand = Some(measurand);
        self
    }

    /// Gets the context.
    ///
    /// # Returns
    ///
    /// An optional type of detail value
    pub fn context(&self) -> Option<&ReadingContextEnumType> {
        self.context.as_ref()
    }

    /// Sets the context.
    ///
    /// # Arguments
    ///
    /// * `context` - Type of detail value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_context(&mut self, context: Option<ReadingContextEnumType>) -> &mut Self {
        self.context = context;
        self
    }

    /// Sets the context.
    ///
    /// # Arguments
    ///
    /// * `context` - Type of detail value: start, end or sample
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_context(mut self, context: ReadingContextEnumType) -> Self {
        self.context = Some(context);
        self
    }

    /// Gets the phase.
    ///
    /// # Returns
    ///
    /// An optional phase as measured or assumed
    pub fn phase(&self) -> Option<&PhaseEnumType> {
        self.phase.as_ref()
    }

    /// Sets the phase.
    ///
    /// # Arguments
    ///
    /// * `phase` - Phase as measured or assumed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_phase(&mut self, phase: Option<PhaseEnumType>) -> &mut Self {
        self.phase = phase;
        self
    }

    /// Sets the phase.
    ///
    /// # Arguments
    ///
    /// * `phase` - Phase as measured or assumed
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_phase(mut self, phase: PhaseEnumType) -> Self {
        self.phase = Some(phase);
        self
    }

    /// Gets the location.
    ///
    /// # Returns
    ///
    /// An optional location of measurement
    pub fn location(&self) -> Option<&LocationEnumType> {
        self.location.as_ref()
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of measurement, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_location(&mut self, location: Option<LocationEnumType>) -> &mut Self {
        self.location = location;
        self
    }

    /// Sets the location.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of measurement
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_location(mut self, location: LocationEnumType) -> Self {
        self.location = Some(location);
        self
    }

    /// Gets the signed meter value.
    ///
    /// # Returns
    ///
    /// An optional signed version of the meter value
    pub fn signed_meter_value(&self) -> Option<&SignedMeterValueType> {
        self.signed_meter_value.as_ref()
    }

    /// Sets the signed meter value.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_value` - Signed version of the meter value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_signed_meter_value(
        &mut self,
        signed_meter_value: Option<SignedMeterValueType>,
    ) -> &mut Self {
        self.signed_meter_value = signed_meter_value;
        self
    }

    /// Sets the signed meter value.
    ///
    /// # Arguments
    ///
    /// * `signed_meter_value` - Contains the signed version of the meter value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_signed_meter_value(mut self, signed_meter_value: SignedMeterValueType) -> Self {
        self.signed_meter_value = Some(signed_meter_value);
        self
    }

    /// Gets the unit of measure.
    ///
    /// # Returns
    ///
    /// An optional unit of the measured value
    pub fn unit_of_measure(&self) -> Option<&UnitOfMeasureType> {
        self.unit_of_measure.as_ref()
    }

    /// Sets the unit of measure.
    ///
    /// # Arguments
    ///
    /// * `unit_of_measure` - Unit of the measured value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_unit_of_measure(&mut self, unit_of_measure: Option<UnitOfMeasureType>) -> &mut Self {
        self.unit_of_measure = unit_of_measure;
        self
    }

    /// Sets the unit of measure.
    ///
    /// # Arguments
    ///
    /// * `unit_of_measure` - Unit of the measured value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_unit_of_measure(mut self, unit_of_measure: UnitOfMeasureType) -> Self {
        self.unit_of_measure = Some(unit_of_measure);
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
    /// * `custom_data` - Custom data for this sampled value, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sampled value
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_sampled_value() {
        let value = 42.5;
        let sampled_value = SampledValueType::new(value);

        assert_eq!(sampled_value.value(), value);
        assert_eq!(sampled_value.measurand(), None);
        assert_eq!(sampled_value.context(), None);
        assert_eq!(sampled_value.phase(), None);
        assert_eq!(sampled_value.location(), None);
        assert_eq!(sampled_value.signed_meter_value(), None);
        assert_eq!(sampled_value.unit_of_measure(), None);
        assert_eq!(sampled_value.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let value = 42.5;
        let measurand = MeasurandEnumType::CurrentImport;
        let context = ReadingContextEnumType::SamplePeriodic;
        let phase = PhaseEnumType::L1;
        let location = LocationEnumType::Outlet;

        let signed_meter_value =
            SignedMeterValueType::new("signed_data".to_string(), "encoding_method".to_string())
                .with_signing_method("signing_method".to_string())
                .with_public_key("public_key".to_string());

        let unit_of_measure = UnitOfMeasureType::new_with_unit("Wh".to_string());

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let sampled_value = SampledValueType::new(value)
            .with_measurand(measurand.clone())
            .with_context(context.clone())
            .with_phase(phase.clone())
            .with_location(location.clone())
            .with_signed_meter_value(signed_meter_value.clone())
            .with_unit_of_measure(unit_of_measure.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(sampled_value.value(), value);
        assert_eq!(sampled_value.measurand(), Some(&measurand));
        assert_eq!(sampled_value.context(), Some(&context));
        assert_eq!(sampled_value.phase(), Some(&phase));
        assert_eq!(sampled_value.location(), Some(&location));
        assert_eq!(
            sampled_value.signed_meter_value(),
            Some(&signed_meter_value)
        );
        assert_eq!(sampled_value.unit_of_measure(), Some(&unit_of_measure));
        assert_eq!(sampled_value.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let value1 = 42.5;
        let mut sampled_value = SampledValueType::new(value1);

        let value2 = 84.0;
        let measurand = MeasurandEnumType::CurrentImport;
        let context = ReadingContextEnumType::SamplePeriodic;
        let phase = PhaseEnumType::L1;
        let location = LocationEnumType::Outlet;

        let signed_meter_value =
            SignedMeterValueType::new("signed_data".to_string(), "encoding_method".to_string())
                .with_signing_method("signing_method".to_string())
                .with_public_key("public_key".to_string());

        let unit_of_measure = UnitOfMeasureType::new_with_unit("Wh".to_string());

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        sampled_value
            .set_value(value2)
            .set_measurand(Some(measurand.clone()))
            .set_context(Some(context.clone()))
            .set_phase(Some(phase.clone()))
            .set_location(Some(location.clone()))
            .set_signed_meter_value(Some(signed_meter_value.clone()))
            .set_unit_of_measure(Some(unit_of_measure.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(sampled_value.value(), value2);
        assert_eq!(sampled_value.measurand(), Some(&measurand));
        assert_eq!(sampled_value.context(), Some(&context));
        assert_eq!(sampled_value.phase(), Some(&phase));
        assert_eq!(sampled_value.location(), Some(&location));
        assert_eq!(
            sampled_value.signed_meter_value(),
            Some(&signed_meter_value)
        );
        assert_eq!(sampled_value.unit_of_measure(), Some(&unit_of_measure));
        assert_eq!(sampled_value.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        sampled_value
            .set_measurand(None)
            .set_context(None)
            .set_phase(None)
            .set_location(None)
            .set_signed_meter_value(None)
            .set_unit_of_measure(None)
            .set_custom_data(None);

        assert_eq!(sampled_value.measurand(), None);
        assert_eq!(sampled_value.context(), None);
        assert_eq!(sampled_value.phase(), None);
        assert_eq!(sampled_value.location(), None);
        assert_eq!(sampled_value.signed_meter_value(), None);
        assert_eq!(sampled_value.unit_of_measure(), None);
        assert_eq!(sampled_value.custom_data(), None);
    }
}
