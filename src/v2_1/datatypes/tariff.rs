use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, message_content::MessageContentType, price::PriceType,
    tariff_energy::TariffEnergyType, tariff_fixed::TariffFixedType, tariff_time::TariffTimeType,
};

/// A tariff is described by fields with prices for:
/// energy,
/// charging time,
/// idle time,
/// fixed fee,
/// reservation time,
/// reservation fixed fee.
///
/// Each of these fields may have (optional) conditions that specify when a price is applicable.
/// The _description_ contains a human-readable explanation of the tariff to be shown to the user.
/// The other fields are parameters that define the tariff. These are used by the charging station to calculate the price.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TariffType {
    /// Required. Unique id of tariff
    #[validate(length(max = 60))]
    pub tariff_id: String,

    /// Required. Currency code according to ISO 4217
    #[validate(length(max = 3))]
    pub currency: String,

    /// Optional. Description of the tariff in different languages
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 10), nested)]
    pub description: Option<Vec<MessageContentType>>,

    /// Optional. Energy costs of the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub energy: Option<TariffEnergyType>,

    /// Optional. Time when this tariff becomes active. When absent, it is immediately active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,

    /// Optional. Charging time costs of the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub charging_time: Option<TariffTimeType>,

    /// Optional. Idle time costs of the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub idle_time: Option<TariffTimeType>,

    /// Optional. Fixed costs of the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub fixed_fee: Option<TariffFixedType>,

    /// Optional. Reservation time costs of the tariff
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub reservation_time: Option<TariffTimeType>,

    /// Optional. Fixed costs for a reservation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub reservation_fixed: Option<TariffFixedType>,

    /// Optional. Minimum cost for a charging session
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub min_cost: Option<PriceType>,

    /// Optional. Maximum cost for a charging session
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub max_cost: Option<PriceType>,

    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl TariffType {
    /// Creates a new `TariffType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique id of tariff
    /// * `currency` - Currency code according to ISO 4217
    ///
    /// # Returns
    ///
    /// A new instance of `TariffType` with optional fields set to `None`
    pub fn new(tariff_id: String, currency: String) -> Self {
        Self {
            tariff_id,
            currency,
            description: None,
            energy: None,
            valid_from: None,
            charging_time: None,
            idle_time: None,
            fixed_fee: None,
            reservation_time: None,
            reservation_fixed: None,
            min_cost: None,
            max_cost: None,
            custom_data: None,
        }
    }

    /// Sets the description of the tariff in different languages.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the tariff in different languages
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_description(mut self, description: Vec<MessageContentType>) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the energy costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_energy(mut self, energy: TariffEnergyType) -> Self {
        self.energy = Some(energy);
        self
    }

    /// Sets the time when this tariff becomes active.
    ///
    /// # Arguments
    ///
    /// * `valid_from` - Time when this tariff becomes active
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_valid_from(mut self, valid_from: DateTime<Utc>) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    /// Sets the charging time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `charging_time` - Charging time costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_charging_time(mut self, charging_time: TariffTimeType) -> Self {
        self.charging_time = Some(charging_time);
        self
    }

    /// Sets the idle time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `idle_time` - Idle time costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_idle_time(mut self, idle_time: TariffTimeType) -> Self {
        self.idle_time = Some(idle_time);
        self
    }

    /// Sets the fixed costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `fixed_fee` - Fixed costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_fixed_fee(mut self, fixed_fee: TariffFixedType) -> Self {
        self.fixed_fee = Some(fixed_fee);
        self
    }

    /// Sets the reservation time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `reservation_time` - Reservation time costs of the tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_reservation_time(mut self, reservation_time: TariffTimeType) -> Self {
        self.reservation_time = Some(reservation_time);
        self
    }

    /// Sets the fixed costs for a reservation.
    ///
    /// # Arguments
    ///
    /// * `reservation_fixed` - Fixed costs for a reservation
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_reservation_fixed(mut self, reservation_fixed: TariffFixedType) -> Self {
        self.reservation_fixed = Some(reservation_fixed);
        self
    }

    /// Sets the minimum cost for a charging session.
    ///
    /// # Arguments
    ///
    /// * `min_cost` - Minimum cost for a charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_min_cost(mut self, min_cost: PriceType) -> Self {
        self.min_cost = Some(min_cost);
        self
    }

    /// Sets the maximum cost for a charging session.
    ///
    /// # Arguments
    ///
    /// * `max_cost` - Maximum cost for a charging session
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_max_cost(mut self, max_cost: PriceType) -> Self {
        self.max_cost = Some(max_cost);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the tariff ID.
    ///
    /// # Returns
    ///
    /// The unique id of tariff
    pub fn tariff_id(&self) -> &str {
        &self.tariff_id
    }

    /// Sets the tariff ID.
    ///
    /// # Arguments
    ///
    /// * `tariff_id` - Unique id of tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_tariff_id(&mut self, tariff_id: String) -> &mut Self {
        self.tariff_id = tariff_id;
        self
    }

    /// Gets the currency.
    ///
    /// # Returns
    ///
    /// The currency code
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Sets the currency.
    ///
    /// # Arguments
    ///
    /// * `currency` - Currency code
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_currency(&mut self, currency: String) -> &mut Self {
        self.currency = currency;
        self
    }

    /// Gets the description.
    ///
    /// # Returns
    ///
    /// An optional reference to the description of the tariff in different languages
    pub fn description(&self) -> Option<&[MessageContentType]> {
        self.description.as_deref()
    }

    /// Sets the description of the tariff in different languages.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the tariff in different languages, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_description(&mut self, description: Option<Vec<MessageContentType>>) -> &mut Self {
        self.description = description;
        self
    }

    /// Gets the energy costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the energy costs of the tariff
    pub fn energy(&self) -> Option<&TariffEnergyType> {
        self.energy.as_ref()
    }

    /// Sets the energy costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `energy` - Energy costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_energy(&mut self, energy: Option<TariffEnergyType>) -> &mut Self {
        self.energy = energy;
        self
    }

    /// Gets the valid from time.
    ///
    /// # Returns
    ///
    /// An optional time when this tariff becomes active
    pub fn valid_from(&self) -> Option<DateTime<Utc>> {
        self.valid_from
    }

    /// Sets the time when this tariff becomes active.
    ///
    /// # Arguments
    ///
    /// * `valid_from` - Time when this tariff becomes active, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_valid_from(&mut self, valid_from: Option<DateTime<Utc>>) -> &mut Self {
        self.valid_from = valid_from;
        self
    }

    /// Gets the charging time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the charging time costs of the tariff
    pub fn charging_time(&self) -> Option<&TariffTimeType> {
        self.charging_time.as_ref()
    }

    /// Sets the charging time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `charging_time` - Charging time costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_charging_time(&mut self, charging_time: Option<TariffTimeType>) -> &mut Self {
        self.charging_time = charging_time;
        self
    }

    /// Gets the idle time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the idle time costs of the tariff
    pub fn idle_time(&self) -> Option<&TariffTimeType> {
        self.idle_time.as_ref()
    }

    /// Sets the idle time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `idle_time` - Idle time costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_idle_time(&mut self, idle_time: Option<TariffTimeType>) -> &mut Self {
        self.idle_time = idle_time;
        self
    }

    /// Gets the fixed costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed costs of the tariff
    pub fn fixed_fee(&self) -> Option<&TariffFixedType> {
        self.fixed_fee.as_ref()
    }

    /// Sets the fixed costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `fixed_fee` - Fixed costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_fixed_fee(&mut self, fixed_fee: Option<TariffFixedType>) -> &mut Self {
        self.fixed_fee = fixed_fee;
        self
    }

    /// Gets the reservation time costs.
    ///
    /// # Returns
    ///
    /// An optional reference to the reservation time costs of the tariff
    pub fn reservation_time(&self) -> Option<&TariffTimeType> {
        self.reservation_time.as_ref()
    }

    /// Sets the reservation time costs of the tariff.
    ///
    /// # Arguments
    ///
    /// * `reservation_time` - Reservation time costs of the tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reservation_time(&mut self, reservation_time: Option<TariffTimeType>) -> &mut Self {
        self.reservation_time = reservation_time;
        self
    }

    /// Gets the fixed costs for a reservation.
    ///
    /// # Returns
    ///
    /// An optional reference to the fixed costs for a reservation
    pub fn reservation_fixed(&self) -> Option<&TariffFixedType> {
        self.reservation_fixed.as_ref()
    }

    /// Sets the fixed costs for a reservation.
    ///
    /// # Arguments
    ///
    /// * `reservation_fixed` - Fixed costs for a reservation, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_reservation_fixed(
        &mut self,
        reservation_fixed: Option<TariffFixedType>,
    ) -> &mut Self {
        self.reservation_fixed = reservation_fixed;
        self
    }

    /// Gets the minimum cost for a charging session.
    ///
    /// # Returns
    ///
    /// An optional reference to the minimum cost for a charging session
    pub fn min_cost(&self) -> Option<&PriceType> {
        self.min_cost.as_ref()
    }

    /// Sets the minimum cost for a charging session.
    ///
    /// # Arguments
    ///
    /// * `min_cost` - Minimum cost for a charging session, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_min_cost(&mut self, min_cost: Option<PriceType>) -> &mut Self {
        self.min_cost = min_cost;
        self
    }

    /// Gets the maximum cost for a charging session.
    ///
    /// # Returns
    ///
    /// An optional reference to the maximum cost for a charging session
    pub fn max_cost(&self) -> Option<&PriceType> {
        self.max_cost.as_ref()
    }

    /// Sets the maximum cost for a charging session.
    ///
    /// # Arguments
    ///
    /// * `max_cost` - Maximum cost for a charging session, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_max_cost(&mut self, max_cost: Option<PriceType>) -> &mut Self {
        self.max_cost = max_cost;
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
    /// * `custom_data` - Custom data for this tariff, or None to clear
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
    use rust_decimal::Decimal;

    use super::*;
    use crate::v2_1::{
        datatypes::{
            message_content::MessageContentType, tariff_energy_price::TariffEnergyPriceType,
            tariff_fixed_price::TariffFixedPriceType, tariff_time_price::TariffTimePriceType,
        },
        enumerations::message_format::MessageFormatEnumType,
    };

    #[test]
    fn test_with_methods() {
        let tariff_id = "tariff-123".to_string();
        let currency = "EUR".to_string();

        let description = vec![MessageContentType::new(
            "Standard Tariff".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        )];
        let energy = TariffEnergyType::new(vec![TariffEnergyPriceType::new(Decimal::new(25, 2))]); // 0.25
        let valid_from = Utc::now();
        let charging_time =
            TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(50, 1))]); // 5.0
        let idle_time = TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(100, 1))]); // 10.0

        // Create a fixed price
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
                                                                           // Create a fixed fee with the fixed price
        #[allow(deprecated)]
        let fixed_fee = TariffFixedType::from_single_price(fixed_price.clone());

        let reservation_time =
            TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(20, 1))]); // 2.0

        // Create a reservation fixed price
        let reservation_price = TariffFixedPriceType::new(Decimal::new(50, 1)); // 5.0
                                                                                // Create a reservation fixed fee with the reservation price
        #[allow(deprecated)]
        let reservation_fixed = TariffFixedType::from_single_price(reservation_price.clone());

        let min_cost = PriceType::new(Decimal::new(50, 1), false); // 5.0 excl tax
        let max_cost = PriceType::new(Decimal::new(500, 1), false); // 50.0 excl tax
        let custom_data = CustomDataType::new("VendorX".to_string());

        let tariff = TariffType::new(tariff_id.clone(), currency.clone())
            .with_description(description.clone())
            .with_energy(energy.clone())
            .with_valid_from(valid_from)
            .with_charging_time(charging_time.clone())
            .with_idle_time(idle_time.clone())
            .with_fixed_fee(fixed_fee.clone())
            .with_reservation_time(reservation_time.clone())
            .with_reservation_fixed(reservation_fixed.clone())
            .with_min_cost(min_cost.clone())
            .with_max_cost(max_cost.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(tariff.tariff_id(), tariff_id);
        assert_eq!(tariff.currency(), currency);
        assert_eq!(tariff.description().unwrap().len(), 1);
        assert_eq!(
            tariff.description().unwrap()[0].content(),
            "Standard Tariff"
        );
        assert_eq!(
            tariff.energy().unwrap().prices()[0].price_kwh,
            Decimal::new(25, 2)
        );
        assert!(tariff.valid_from().is_some());
        assert_eq!(
            tariff.charging_time().unwrap().prices()[0].price_minute,
            Decimal::new(50, 1)
        );
        assert_eq!(
            tariff.idle_time().unwrap().prices()[0].price_minute,
            Decimal::new(100, 1)
        );

        #[allow(deprecated)]
        {
            assert_eq!(
                tariff.fixed_fee().unwrap().fixed_price().price_fixed,
                Decimal::new(100, 1)
            );
            assert_eq!(
                tariff
                    .reservation_fixed()
                    .unwrap()
                    .fixed_price()
                    .price_fixed,
                Decimal::new(50, 1)
            );
        }

        assert_eq!(
            tariff.reservation_time().unwrap().prices()[0].price_minute,
            Decimal::new(20, 1)
        );
        assert_eq!(
            tariff.min_cost().unwrap().excl_tax(),
            Some(Decimal::new(50, 1))
        );
        assert_eq!(
            tariff.max_cost().unwrap().excl_tax(),
            Some(Decimal::new(500, 1))
        );
        assert_eq!(tariff.custom_data().unwrap().vendor_id(), "VendorX");
    }

    #[test]
    fn test_setter_methods() {
        let tariff_id1 = "tariff-123".to_string();
        let currency1 = "EUR".to_string();
        let tariff_id2 = "tariff-456".to_string();
        let currency2 = "USD".to_string();

        let description = vec![MessageContentType::new(
            "Standard Tariff".to_string(),
            MessageFormatEnumType::ASCII,
            "en".to_string(),
        )];
        let energy = TariffEnergyType::new(vec![TariffEnergyPriceType::new(Decimal::new(25, 2))]); // 0.25
        let valid_from = Utc::now();
        let charging_time =
            TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(50, 1))]); // 5.0
        let idle_time = TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(100, 1))]); // 10.0

        // Create a fixed price
        let fixed_price = TariffFixedPriceType::new(Decimal::new(100, 1)); // 10.0
                                                                           // Create a fixed fee with the fixed price
        #[allow(deprecated)]
        let fixed_fee = TariffFixedType::from_single_price(fixed_price.clone());

        let reservation_time =
            TariffTimeType::new(vec![TariffTimePriceType::new(Decimal::new(20, 1))]); // 2.0

        // Create a reservation fixed price
        let reservation_price = TariffFixedPriceType::new(Decimal::new(50, 1)); // 5.0
                                                                                // Create a reservation fixed fee with the reservation price
        #[allow(deprecated)]
        let reservation_fixed = TariffFixedType::from_single_price(reservation_price.clone());

        let min_cost = PriceType::new(Decimal::new(50, 1), false); // 5.0 excl tax
        let max_cost = PriceType::new(Decimal::new(500, 1), false); // 50.0 excl tax
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut tariff = TariffType::new(tariff_id1, currency1);

        tariff
            .set_tariff_id(tariff_id2.clone())
            .set_currency(currency2.clone())
            .set_description(Some(description.clone()))
            .set_energy(Some(energy.clone()))
            .set_valid_from(Some(valid_from))
            .set_charging_time(Some(charging_time.clone()))
            .set_idle_time(Some(idle_time.clone()))
            .set_fixed_fee(Some(fixed_fee.clone()))
            .set_reservation_time(Some(reservation_time.clone()))
            .set_reservation_fixed(Some(reservation_fixed.clone()))
            .set_min_cost(Some(min_cost.clone()))
            .set_max_cost(Some(max_cost.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(tariff.tariff_id(), tariff_id2);
        assert_eq!(tariff.currency(), currency2);
        assert_eq!(tariff.description().unwrap().len(), 1);
        assert_eq!(
            tariff.description().unwrap()[0].content(),
            "Standard Tariff"
        );
        assert_eq!(
            tariff.energy().unwrap().prices()[0].price_kwh,
            Decimal::new(25, 2)
        );
        assert!(tariff.valid_from().is_some());
        assert_eq!(
            tariff.charging_time().unwrap().prices()[0].price_minute,
            Decimal::new(50, 1)
        );
        assert_eq!(
            tariff.idle_time().unwrap().prices()[0].price_minute,
            Decimal::new(100, 1)
        );

        #[allow(deprecated)]
        {
            assert_eq!(
                tariff.fixed_fee().unwrap().fixed_price().price_fixed,
                Decimal::new(100, 1)
            );
            assert_eq!(
                tariff
                    .reservation_fixed()
                    .unwrap()
                    .fixed_price()
                    .price_fixed,
                Decimal::new(50, 1)
            );
        }

        assert_eq!(
            tariff.reservation_time().unwrap().prices()[0].price_minute,
            Decimal::new(20, 1)
        );
        assert_eq!(
            tariff.min_cost().unwrap().excl_tax(),
            Some(Decimal::new(50, 1))
        );
        assert_eq!(
            tariff.max_cost().unwrap().excl_tax(),
            Some(Decimal::new(500, 1))
        );
        assert_eq!(tariff.custom_data().unwrap().vendor_id(), "VendorX");

        // Test clearing optional fields
        tariff
            .set_description(None)
            .set_energy(None)
            .set_valid_from(None)
            .set_charging_time(None)
            .set_idle_time(None)
            .set_fixed_fee(None)
            .set_reservation_time(None)
            .set_reservation_fixed(None)
            .set_min_cost(None)
            .set_max_cost(None)
            .set_custom_data(None);

        assert_eq!(tariff.description(), None);
        assert_eq!(tariff.energy(), None);
        assert_eq!(tariff.valid_from(), None);
        assert_eq!(tariff.charging_time(), None);
        assert_eq!(tariff.idle_time(), None);
        assert_eq!(tariff.fixed_fee(), None);
        assert_eq!(tariff.reservation_time(), None);
        assert_eq!(tariff.reservation_fixed(), None);
        assert_eq!(tariff.min_cost(), None);
        assert_eq!(tariff.max_cost(), None);
        assert_eq!(tariff.custom_data(), None);
    }
}
