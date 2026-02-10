use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    custom_data::CustomDataType, ev_absolute_price_schedule::EVAbsolutePriceScheduleType,
    ev_power_schedule::EVPowerScheduleType,
};

/// Energy offer from EV to EVSE.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EVEnergyOfferType {
    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Power schedule of EV energy offer.
    pub ev_power_schedule: EVPowerScheduleType,

    /// Price schedule of EV energy offer.
    pub ev_absolute_price_schedule: EVAbsolutePriceScheduleType,
}

impl EVEnergyOfferType {
    /// Creates a new `EVEnergyOfferType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `ev_power_schedule` - Power schedule of EV energy offer
    /// * `ev_absolute_price_schedule` - Price schedule of EV energy offer
    ///
    /// # Returns
    ///
    /// A new instance of `EVEnergyOfferType` with optional fields set to `None`
    pub fn new(
        ev_power_schedule: EVPowerScheduleType,
        ev_absolute_price_schedule: EVAbsolutePriceScheduleType,
    ) -> Self {
        Self {
            ev_power_schedule,
            ev_absolute_price_schedule,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this energy offer
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the power schedule.
    ///
    /// # Returns
    ///
    /// A reference to the power schedule of EV energy offer
    pub fn ev_power_schedule(&self) -> &EVPowerScheduleType {
        &self.ev_power_schedule
    }

    /// Sets the power schedule.
    ///
    /// # Arguments
    ///
    /// * `ev_power_schedule` - Power schedule of EV energy offer
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_power_schedule(&mut self, ev_power_schedule: EVPowerScheduleType) -> &mut Self {
        self.ev_power_schedule = ev_power_schedule;
        self
    }

    /// Gets the price schedule.
    ///
    /// # Returns
    ///
    /// A reference to the price schedule of EV energy offer
    pub fn ev_absolute_price_schedule(&self) -> &EVAbsolutePriceScheduleType {
        &self.ev_absolute_price_schedule
    }

    /// Sets the price schedule.
    ///
    /// # Arguments
    ///
    /// * `ev_absolute_price_schedule` - Price schedule of EV energy offer
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_ev_absolute_price_schedule(
        &mut self,
        ev_absolute_price_schedule: EVAbsolutePriceScheduleType,
    ) -> &mut Self {
        self.ev_absolute_price_schedule = ev_absolute_price_schedule;
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
    /// * `custom_data` - Custom data for this energy offer, or None to clear
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
    use crate::v2_1::datatypes::ev_absolute_price_schedule_entry::EVAbsolutePriceScheduleEntryType;
    use crate::v2_1::datatypes::ev_power_schedule_entry::EVPowerScheduleEntryType;
    use chrono::Utc;

    #[test]
    fn test_new_ev_energy_offer() {
        let time_anchor = Utc::now();

        // Create power schedule
        let power_entries = vec![
            EVPowerScheduleEntryType::new(3600, 11000.0),
            EVPowerScheduleEntryType::new(7200, 7500.0),
        ];

        let power_schedule = EVPowerScheduleType {
            time_anchor: time_anchor.clone(),
            ev_power_schedule_entries: power_entries,
            custom_data: None,
        }; // Create price schedule
        let price_entries = vec![
            EVAbsolutePriceScheduleEntryType::new_with_single_price(3600, 0.25, 0.0),
            EVAbsolutePriceScheduleEntryType::new_with_single_price(7200, 0.30, 0.0),
        ];

        let price_schedule = EVAbsolutePriceScheduleType {
            time_anchor: time_anchor.clone(),
            currency: "EUR".to_string(),
            price_algorithm: "Power".to_string(),
            ev_absolute_price_schedule_entries: price_entries,
            custom_data: None,
        };

        let offer = EVEnergyOfferType::new(power_schedule.clone(), price_schedule.clone());

        assert_eq!(offer.ev_power_schedule(), &power_schedule);
        assert_eq!(offer.ev_absolute_price_schedule(), &price_schedule);
        assert_eq!(offer.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data() {
        let time_anchor = Utc::now();

        // Create power schedule
        let power_entries = vec![EVPowerScheduleEntryType::new(3600, 11000.0)];

        let power_schedule = EVPowerScheduleType {
            time_anchor: time_anchor.clone(),
            ev_power_schedule_entries: power_entries,
            custom_data: None,
        };

        // Create price schedule
        let price_entries = vec![EVAbsolutePriceScheduleEntryType::new_with_single_price(
            3600, 0.25, 0.0,
        )];

        let price_schedule = EVAbsolutePriceScheduleType {
            time_anchor: time_anchor.clone(),
            currency: "EUR".to_string(),
            price_algorithm: "Power".to_string(),
            ev_absolute_price_schedule_entries: price_entries,
            custom_data: None,
        };

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let offer = EVEnergyOfferType::new(power_schedule.clone(), price_schedule.clone())
            .with_custom_data(custom_data.clone());

        assert_eq!(offer.ev_power_schedule(), &power_schedule);
        assert_eq!(offer.ev_absolute_price_schedule(), &price_schedule);
        assert_eq!(offer.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();

        // Create initial power schedule
        let power_entries1 = vec![EVPowerScheduleEntryType::new(3600, 11000.0)];

        let power_schedule1 = EVPowerScheduleType {
            time_anchor: time_anchor1.clone(),
            ev_power_schedule_entries: power_entries1,
            custom_data: None,
        }; // Create initial price schedule
        let price_entries1 = vec![EVAbsolutePriceScheduleEntryType::new_with_single_price(
            3600, 0.25, 0.0,
        )];

        let price_schedule1 = EVAbsolutePriceScheduleType {
            time_anchor: time_anchor1.clone(),
            currency: "EUR".to_string(),
            price_algorithm: "Power".to_string(),
            ev_absolute_price_schedule_entries: price_entries1,
            custom_data: None,
        };

        // Create updated power schedule
        let time_anchor2 = Utc::now();
        let power_entries2 = vec![
            EVPowerScheduleEntryType::new(1800, 22000.0),
            EVPowerScheduleEntryType::new(3600, 11000.0),
        ];

        let power_schedule2 = EVPowerScheduleType {
            time_anchor: time_anchor2.clone(),
            ev_power_schedule_entries: power_entries2,
            custom_data: None,
        }; // Create updated price schedule
        let price_entries2 = vec![
            EVAbsolutePriceScheduleEntryType::new_with_single_price(1800, 0.20, 0.0),
            EVAbsolutePriceScheduleEntryType::new_with_single_price(3600, 0.25, 0.0),
        ];

        let price_schedule2 = EVAbsolutePriceScheduleType {
            time_anchor: time_anchor2.clone(),
            currency: "USD".to_string(),
            price_algorithm: "PeakPower".to_string(),
            ev_absolute_price_schedule_entries: price_entries2,
            custom_data: None,
        };

        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let mut offer = EVEnergyOfferType::new(power_schedule1.clone(), price_schedule1.clone());

        offer
            .set_ev_power_schedule(power_schedule2.clone())
            .set_ev_absolute_price_schedule(price_schedule2.clone())
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(offer.ev_power_schedule(), &power_schedule2);
        assert_eq!(offer.ev_absolute_price_schedule(), &price_schedule2);
        assert_eq!(offer.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        offer.set_custom_data(None);
        assert_eq!(offer.custom_data(), None);
    }
}
