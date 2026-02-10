use crate::v2_1::{
    datatypes::{
        ACChargingParametersType, CustomDataType, DCChargingParametersType,
        DERChargingParametersType, EVEnergyOfferType, V2XChargingParametersType,
    },
    enumerations::{ControlModeEnumType, EnergyTransferModeEnumType, MobilityNeedsModeEnumType},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents the charging needs of an EV.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ChargingNeedsType {
    /// Mode of energy transfer requested by the EV.
    pub requested_energy_transfer: EnergyTransferModeEnumType,

    /// Modes of energy transfer that are marked as available by EV
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1))]
    pub available_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,

    /// Indicates whether EV wants to operate in Dynamic or Scheduled mode.When absent, Scheduled mode is assumed for backwards compatibility.ISO 15118-20: ServiceSelectionReq(SelectedEnergyTransferService)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_mode: Option<ControlModeEnumType>,

    /// Value of EVCC indicates that EV determines min/target SOC and departure time. +\r\nA value of EVCC_SECC indicates that charging station or CSMS may also update min/target SOC and departure time. +\r\n*ISO 15118-20:* +\r\nServiceSelectionReq(SelectedEnergyTransferService)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobility_needs_mode: Option<MobilityNeedsModeEnumType>,

    /// Estimated departure time of the EV.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_time: Option<DateTime<Utc>>,

    /// Charging parameters for ISO 15118-20, also supporting V2X charging/discharging.+\r\nAll values are greater or equal to zero, with the exception of EVMinEnergyRequest, EVMaxEnergyRequest, EVTargetEnergyRequest, EVMinV2XEnergyRequest and EVMaxV2XEnergyRequest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v2x_charging_parameters: Option<V2XChargingParametersType>,

    /// EV DC charging parameters for ISO 15118-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_charging_parameters: Option<DCChargingParametersType>,

    /// EV AC charging parameters for ISO 15118-2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_charging_parameters: Option<ACChargingParametersType>,

    /// *(2.1)* A schedule of the energy amount over time that EV is willing to discharge. A negative value indicates the willingness to discharge under specific conditions, a positive value indicates that the EV currently is not able to offer energy to discharge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev_energy_offer: Option<EVEnergyOfferType>,

    /// DERChargingParametersType is used in ChargingNeedsType during an ISO 15118-20 session for AC_BPT_DER to report the inverter settings related to DER control that were agreed between EVSE and EV.\r\n\r\nFields starting with \"ev\" contain values from the EV.\r\nOther fields contain a value that is supported by both EV and EVSE.\r\n\r\nDERChargingParametersType type is only relevant in case of an ISO 15118-20 AC_BPT_DER/AC_DER charging session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub der_charging_parameters: Option<DERChargingParametersType>,

    /// Optional custom data
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl ChargingNeedsType {
    /// Creates a new `ChargingNeedsType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `requested_energy_transfer` - Mode of energy transfer requested by the EV
    ///
    /// # Returns
    ///
    /// A new instance of `ChargingNeedsType` with optional fields set to `None`
    pub fn new(requested_energy_transfer: EnergyTransferModeEnumType) -> Self {
        Self {
            requested_energy_transfer,
            available_energy_transfer: None,
            control_mode: None,
            mobility_needs_mode: None,
            departure_time: None,
            v2x_charging_parameters: None,
            dc_charging_parameters: None,
            ac_charging_parameters: None,
            ev_energy_offer: None,
            der_charging_parameters: None,
            custom_data: None,
        }
    }

    /// Sets the departure time.
    ///
    /// # Arguments
    ///
    /// * `departure_time` - Estimated departure time of the EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_departure_time(mut self, departure_time: DateTime<Utc>) -> Self {
        self.departure_time = Some(departure_time);
        self
    }

    /// Sets the AC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `ac_charging_parameters` - EV AC charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_ac_charging_parameters(
        mut self,
        ac_charging_parameters: ACChargingParametersType,
    ) -> Self {
        self.ac_charging_parameters = Some(ac_charging_parameters);
        self
    }

    /// Sets the DC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `dc_charging_parameters` - EV DC charging parameters
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_dc_charging_parameters(
        mut self,
        dc_charging_parameters: DCChargingParametersType,
    ) -> Self {
        self.dc_charging_parameters = Some(dc_charging_parameters);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this charging needs
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the requested energy transfer mode.
    ///
    /// # Returns
    ///
    /// The mode of energy transfer requested by the EV
    pub fn requested_energy_transfer(&self) -> &EnergyTransferModeEnumType {
        &self.requested_energy_transfer
    }

    /// Sets the requested energy transfer mode.
    ///
    /// # Arguments
    ///
    /// * `requested_energy_transfer` - Mode of energy transfer requested by the EV
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_requested_energy_transfer(
        &mut self,
        requested_energy_transfer: EnergyTransferModeEnumType,
    ) -> &mut Self {
        self.requested_energy_transfer = requested_energy_transfer;
        self
    }

    /// Gets the departure time.
    ///
    /// # Returns
    ///
    /// An optional reference to the estimated departure time of the EV
    pub fn departure_time(&self) -> Option<&DateTime<Utc>> {
        self.departure_time.as_ref()
    }

    /// Sets the departure time.
    ///
    /// # Arguments
    ///
    /// * `departure_time` - Estimated departure time of the EV, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_departure_time(&mut self, departure_time: Option<DateTime<Utc>>) -> &mut Self {
        self.departure_time = departure_time;
        self
    }

    /// Gets the AC charging parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the EV AC charging parameters
    pub fn ac_charging_parameters(&self) -> Option<&ACChargingParametersType> {
        self.ac_charging_parameters.as_ref()
    }

    /// Sets the AC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `ac_charging_parameters` - EV AC charging parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ac_charging_parameters(
        &mut self,
        ac_charging_parameters: Option<ACChargingParametersType>,
    ) -> &mut Self {
        self.ac_charging_parameters = ac_charging_parameters;
        self
    }

    /// Gets the DC charging parameters.
    ///
    /// # Returns
    ///
    /// An optional reference to the EV DC charging parameters
    pub fn dc_charging_parameters(&self) -> Option<&DCChargingParametersType> {
        self.dc_charging_parameters.as_ref()
    }

    /// Sets the DC charging parameters.
    ///
    /// # Arguments
    ///
    /// * `dc_charging_parameters` - EV DC charging parameters, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_dc_charging_parameters(
        &mut self,
        dc_charging_parameters: Option<DCChargingParametersType>,
    ) -> &mut Self {
        self.dc_charging_parameters = dc_charging_parameters;
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
    /// * `custom_data` - Custom data for this charging needs, or None to clear
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
    /// `Ok(())` if the instance is valid, otherwise an error
    pub fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Validate::validate(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use rust_decimal::Decimal;
    #[test]
    fn test_new_charging_needs() {
        let needs = ChargingNeedsType::new(EnergyTransferModeEnumType::DC);

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::DC
        );
        assert_eq!(needs.departure_time(), None);
        assert_eq!(needs.ac_charging_parameters(), None);
        assert_eq!(needs.dc_charging_parameters(), None);
        assert_eq!(needs.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(Decimal::from(400), Decimal::from(100));

        let needs = ChargingNeedsType::new(EnergyTransferModeEnumType::ACThreePhase)
            .with_departure_time(departure_time)
            .with_ac_charging_parameters(ac_params.clone())
            .with_dc_charging_parameters(dc_params.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::ACThreePhase
        );
        assert_eq!(needs.departure_time(), Some(&departure_time));
        assert_eq!(needs.ac_charging_parameters(), Some(&ac_params));
        assert_eq!(needs.dc_charging_parameters(), Some(&dc_params));
        assert_eq!(needs.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(Decimal::from(400), Decimal::from(100));

        let mut needs = ChargingNeedsType::new(EnergyTransferModeEnumType::ACSinglePhase);

        needs
            .set_requested_energy_transfer(EnergyTransferModeEnumType::DCBPT)
            .set_departure_time(Some(departure_time))
            .set_ac_charging_parameters(Some(ac_params.clone()))
            .set_dc_charging_parameters(Some(dc_params.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(
            needs.requested_energy_transfer(),
            &EnergyTransferModeEnumType::DCBPT
        );
        assert_eq!(needs.departure_time(), Some(&departure_time));
        assert_eq!(needs.ac_charging_parameters(), Some(&ac_params));
        assert_eq!(needs.dc_charging_parameters(), Some(&dc_params));
        assert_eq!(needs.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        needs
            .set_departure_time(None)
            .set_ac_charging_parameters(None)
            .set_dc_charging_parameters(None)
            .set_custom_data(None);

        assert_eq!(needs.departure_time(), None);
        assert_eq!(needs.ac_charging_parameters(), None);
        assert_eq!(needs.dc_charging_parameters(), None);
        assert_eq!(needs.custom_data(), None);
    }

    #[test]
    fn test_validation() {
        // Create a valid charging needs
        let valid_needs = ChargingNeedsType::new(EnergyTransferModeEnumType::DC);

        // Validation should pass
        assert!(valid_needs.validate().is_ok());

        // Test with valid custom data
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(Decimal::from(400), Decimal::from(100));

        let valid_needs_with_params =
            ChargingNeedsType::new(EnergyTransferModeEnumType::ACThreePhase)
                .with_departure_time(departure_time)
                .with_ac_charging_parameters(ac_params)
                .with_dc_charging_parameters(dc_params)
                .with_custom_data(custom_data);

        // Validation should pass
        assert!(valid_needs_with_params.validate().is_ok());

        // Test with invalid custom data (nested validation)
        let invalid_custom_data = CustomDataType::new("a".repeat(256)); // Exceeds max length of 255
        let invalid_needs = ChargingNeedsType::new(EnergyTransferModeEnumType::DC)
            .with_custom_data(invalid_custom_data);

        // Validation should fail
        assert!(invalid_needs.validate().is_err());
    }

    #[test]
    fn test_serialization_deserialization() {
        let custom_data = CustomDataType::new("VendorX".to_string());
        let departure_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let ac_params = ACChargingParametersType::new_from_f64(10000.0, 10.0, 32.0, 400.0);
        let dc_params = DCChargingParametersType::new(Decimal::from(400), Decimal::from(100));

        let needs = ChargingNeedsType::new(EnergyTransferModeEnumType::ACThreePhase)
            .with_departure_time(departure_time)
            .with_ac_charging_parameters(ac_params)
            .with_dc_charging_parameters(dc_params)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&needs).unwrap();

        // Deserialize back
        let deserialized: ChargingNeedsType = serde_json::from_str(&serialized).unwrap();

        // Verify the result is the same as the original object
        assert_eq!(needs, deserialized);

        // Validate the deserialized object
        assert!(deserialized.validate().is_ok());
    }
}
