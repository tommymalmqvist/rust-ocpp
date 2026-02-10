use serde::{Deserialize, Serialize};
use validator::Validate;

use super::custom_data::CustomDataType;
use crate::v2_1::datatypes::rational_number::RationalNumberType;

/// Part of ISO 15118-20 price schedule.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceRuleType {
    /// The duration of the parking fee period (in seconds).
    /// When the time enters into a ParkingFeePeriod, the ParkingFee will apply to the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parking_fee_period: Option<i32>,

    /// Number of grams of CO2 per kWh.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub carbon_dioxide_emission: Option<i32>,

    /// Percentage of the power that is created by renewable resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0, max = 100))]
    pub renewable_generation_percentage: Option<i32>,

    /// Required. Energy fee for this price rule.
    #[validate(nested)]
    pub energy_fee: RationalNumberType,

    /// Parking fee for this price rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub parking_fee: Option<RationalNumberType>,

    /// Required. Start of the power range for this price rule.
    #[validate(nested)]
    pub power_range_start: RationalNumberType,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PriceRuleType {
    /// Creates a new `PriceRuleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee for this price rule
    /// * `power_range_start` - Start of the power range for this price rule
    ///
    /// # Returns
    ///
    /// A new instance of `PriceRuleType` with optional fields set to `None`
    pub fn new(energy_fee: RationalNumberType, power_range_start: RationalNumberType) -> Self {
        Self {
            custom_data: None,
            parking_fee_period: None,
            carbon_dioxide_emission: None,
            renewable_generation_percentage: None,
            energy_fee,
            parking_fee: None,
            power_range_start,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
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
    /// * `custom_data` - Custom data for this price rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_custom_data(&mut self, custom_data: Option<CustomDataType>) -> &mut Self {
        self.custom_data = custom_data;
        self
    }

    /// Gets the parking fee period.
    ///
    /// # Returns
    ///
    /// The duration of the parking fee period in seconds, if set
    pub fn parking_fee_period(&self) -> Option<i32> {
        self.parking_fee_period
    }

    /// Sets the parking fee period.
    ///
    /// # Arguments
    ///
    /// * `parking_fee_period` - Duration of the parking fee period in seconds, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_parking_fee_period(&mut self, parking_fee_period: Option<i32>) -> &mut Self {
        self.parking_fee_period = parking_fee_period;
        self
    }

    /// Gets the carbon dioxide emission.
    ///
    /// # Returns
    ///
    /// The number of grams of CO2 per kWh, if set
    pub fn carbon_dioxide_emission(&self) -> Option<i32> {
        self.carbon_dioxide_emission
    }

    /// Sets the carbon dioxide emission.
    ///
    /// # Arguments
    ///
    /// * `carbon_dioxide_emission` - Number of grams of CO2 per kWh, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_carbon_dioxide_emission(
        &mut self,
        carbon_dioxide_emission: Option<i32>,
    ) -> &mut Self {
        self.carbon_dioxide_emission = carbon_dioxide_emission;
        self
    }

    /// Gets the renewable generation percentage.
    ///
    /// # Returns
    ///
    /// The percentage of power from renewable resources, if set
    pub fn renewable_generation_percentage(&self) -> Option<i32> {
        self.renewable_generation_percentage
    }

    /// Sets the renewable generation percentage.
    ///
    /// # Arguments
    ///
    /// * `renewable_generation_percentage` - Percentage of power from renewable resources, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_renewable_generation_percentage(
        &mut self,
        renewable_generation_percentage: Option<i32>,
    ) -> &mut Self {
        self.renewable_generation_percentage = renewable_generation_percentage;
        self
    }

    /// Gets the energy fee.
    ///
    /// # Returns
    ///
    /// The energy fee for this price rule
    pub fn energy_fee(&self) -> &RationalNumberType {
        &self.energy_fee
    }

    /// Sets the energy fee.
    ///
    /// # Arguments
    ///
    /// * `energy_fee` - Energy fee for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy_fee(&mut self, energy_fee: RationalNumberType) -> &mut Self {
        self.energy_fee = energy_fee;
        self
    }

    /// Gets the parking fee.
    ///
    /// # Returns
    ///
    /// The parking fee for this price rule, if set
    pub fn parking_fee(&self) -> Option<&RationalNumberType> {
        self.parking_fee.as_ref()
    }

    /// Sets the parking fee.
    ///
    /// # Arguments
    ///
    /// * `parking_fee` - Parking fee for this price rule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_parking_fee(&mut self, parking_fee: Option<RationalNumberType>) -> &mut Self {
        self.parking_fee = parking_fee;
        self
    }

    /// Gets the power range start.
    ///
    /// # Returns
    ///
    /// The start of the power range for this price rule
    pub fn power_range_start(&self) -> &RationalNumberType {
        &self.power_range_start
    }

    /// Sets the power range start.
    ///
    /// # Arguments
    ///
    /// * `power_range_start` - Start of the power range for this price rule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_power_range_start(&mut self, power_range_start: RationalNumberType) -> &mut Self {
        self.power_range_start = power_range_start;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_price_rule() {
        let energy_fee = RationalNumberType::new(0, 10);
        let power_range_start = RationalNumberType::new(0, 0);

        let price_rule = PriceRuleType::new(energy_fee.clone(), power_range_start.clone());

        assert_eq!(price_rule.energy_fee().value(), energy_fee.value());
        assert_eq!(price_rule.energy_fee().exponent(), energy_fee.exponent());
        assert_eq!(
            price_rule.power_range_start().value(),
            power_range_start.value()
        );
        assert_eq!(
            price_rule.power_range_start().exponent(),
            power_range_start.exponent()
        );
        assert_eq!(price_rule.custom_data(), None);
        assert_eq!(price_rule.parking_fee_period(), None);
        assert_eq!(price_rule.carbon_dioxide_emission(), None);
        assert_eq!(price_rule.renewable_generation_percentage(), None);
        assert_eq!(price_rule.parking_fee(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let energy_fee = RationalNumberType::new(0, 10);
        let power_range_start = RationalNumberType::new(0, 0);
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let price_rule = PriceRuleType::new(energy_fee.clone(), power_range_start.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(price_rule.energy_fee().value(), energy_fee.value());
        assert_eq!(price_rule.energy_fee().exponent(), energy_fee.exponent());
        assert_eq!(
            price_rule.power_range_start().value(),
            power_range_start.value()
        );
        assert_eq!(
            price_rule.power_range_start().exponent(),
            power_range_start.exponent()
        );
        assert_eq!(price_rule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let energy_fee1 = RationalNumberType::new(0, 10);
        let energy_fee2 = RationalNumberType::new(0, 15);
        let power_range_start1 = RationalNumberType::new(0, 0);
        let power_range_start2 = RationalNumberType::new(0, 5);
        let parking_fee = RationalNumberType::new(0, 2);
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };
        let parking_fee_period = 3600;
        let carbon_dioxide_emission = 50;
        let renewable_generation_percentage = 75;

        let mut price_rule = PriceRuleType::new(energy_fee1.clone(), power_range_start1.clone());

        price_rule
            .set_custom_data(Some(custom_data.clone()))
            .set_parking_fee_period(Some(parking_fee_period))
            .set_carbon_dioxide_emission(Some(carbon_dioxide_emission))
            .set_renewable_generation_percentage(Some(renewable_generation_percentage))
            .set_energy_fee(energy_fee2.clone())
            .set_parking_fee(Some(parking_fee.clone()))
            .set_power_range_start(power_range_start2.clone());

        assert_eq!(price_rule.custom_data(), Some(&custom_data));
        assert_eq!(price_rule.parking_fee_period(), Some(parking_fee_period));
        assert_eq!(
            price_rule.carbon_dioxide_emission(),
            Some(carbon_dioxide_emission)
        );
        assert_eq!(
            price_rule.renewable_generation_percentage(),
            Some(renewable_generation_percentage)
        );
        assert_eq!(price_rule.energy_fee().value(), energy_fee2.value());
        assert_eq!(price_rule.energy_fee().exponent(), energy_fee2.exponent());
        assert_eq!(
            price_rule.parking_fee().unwrap().value(),
            parking_fee.value()
        );
        assert_eq!(
            price_rule.parking_fee().unwrap().exponent(),
            parking_fee.exponent()
        );
        assert_eq!(
            price_rule.power_range_start().value(),
            power_range_start2.value()
        );
        assert_eq!(
            price_rule.power_range_start().exponent(),
            power_range_start2.exponent()
        );

        // Test clearing optional fields
        price_rule.set_custom_data(None);
        price_rule.set_parking_fee_period(None);
        price_rule.set_carbon_dioxide_emission(None);
        price_rule.set_renewable_generation_percentage(None);
        price_rule.set_parking_fee(None);

        assert_eq!(price_rule.custom_data(), None);
        assert_eq!(price_rule.parking_fee_period(), None);
        assert_eq!(price_rule.carbon_dioxide_emission(), None);
        assert_eq!(price_rule.renewable_generation_percentage(), None);
        assert_eq!(price_rule.parking_fee(), None);
    }
}
