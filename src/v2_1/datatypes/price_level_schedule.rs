use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, price_level_schedule_entry::PriceLevelScheduleEntryType};

/// Price level schedule structure defines a list of time periods during which a specific price level applies.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PriceLevelScheduleType {
    /// Required. Starting point of this price schedule.
    pub time_anchor: DateTime<Utc>,

    /// Required. Unique ID of this price schedule.
    #[validate(range(min = 0))]
    pub price_schedule_id: i32,

    /// Required. Defines the overall number of distinct price level elements used across all PriceLevelSchedules.
    #[validate(range(min = 0))]
    pub number_of_price_levels: i32,

    /// Required. List of price level schedule entries.
    #[validate(length(min = 1, max = 100))]
    #[validate(nested)]
    pub price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,

    /// Optional. Description of the price schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 32))]
    pub price_schedule_description: Option<String>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl PriceLevelScheduleType {
    /// Creates a new `PriceLevelScheduleType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of this price schedule
    /// * `price_schedule_id` - Unique ID of this price schedule
    /// * `number_of_price_levels` - Overall number of distinct price level elements
    /// * `price_level_schedule_entries` - List of price level schedule entries
    ///
    /// # Returns
    ///
    /// A new instance of `PriceLevelScheduleType` with optional fields set to `None`
    pub fn new(
        time_anchor: DateTime<Utc>,
        price_schedule_id: i32,
        number_of_price_levels: i32,
        price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
    ) -> Self {
        Self {
            time_anchor,
            price_schedule_id,
            number_of_price_levels,
            price_level_schedule_entries,
            price_schedule_description: None,
            custom_data: None,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this price level schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_price_schedule_description(mut self, description: String) -> Self {
        self.price_schedule_description = Some(description);
        self
    }

    /// Gets the time anchor.
    ///
    /// # Returns
    ///
    /// A reference to the starting point of this price schedule
    pub fn time_anchor(&self) -> &DateTime<Utc> {
        &self.time_anchor
    }

    /// Sets the time anchor.
    ///
    /// # Arguments
    ///
    /// * `time_anchor` - Starting point of this price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_time_anchor(&mut self, time_anchor: DateTime<Utc>) -> &mut Self {
        self.time_anchor = time_anchor;
        self
    }

    /// Gets the price schedule ID.
    ///
    /// # Returns
    ///
    /// The unique ID of this price schedule
    pub fn price_schedule_id(&self) -> i32 {
        self.price_schedule_id
    }

    /// Sets the price schedule ID.
    ///
    /// # Arguments
    ///
    /// * `price_schedule_id` - Unique ID of this price schedule
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_id(&mut self, price_schedule_id: i32) -> &mut Self {
        self.price_schedule_id = price_schedule_id;
        self
    }

    /// Gets the number of price levels.
    ///
    /// # Returns
    ///
    /// The overall number of distinct price level elements
    pub fn number_of_price_levels(&self) -> i32 {
        self.number_of_price_levels
    }

    /// Sets the number of price levels.
    ///
    /// # Arguments
    ///
    /// * `number_of_price_levels` - Overall number of distinct price level elements
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_number_of_price_levels(&mut self, number_of_price_levels: i32) -> &mut Self {
        self.number_of_price_levels = number_of_price_levels;
        self
    }

    /// Gets the price level schedule entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of price level schedule entries
    pub fn price_level_schedule_entries(&self) -> &Vec<PriceLevelScheduleEntryType> {
        &self.price_level_schedule_entries
    }

    /// Sets the price level schedule entries.
    ///
    /// # Arguments
    ///
    /// * `price_level_schedule_entries` - List of price level schedule entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_level_schedule_entries(
        &mut self,
        price_level_schedule_entries: Vec<PriceLevelScheduleEntryType>,
    ) -> &mut Self {
        self.price_level_schedule_entries = price_level_schedule_entries;
        self
    }

    /// Gets the price schedule description.
    ///
    /// # Returns
    ///
    /// An optional reference to the description of the price schedule
    pub fn price_schedule_description(&self) -> Option<&String> {
        self.price_schedule_description.as_ref()
    }

    /// Sets the price schedule description.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of the price schedule, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_price_schedule_description(&mut self, description: Option<String>) -> &mut Self {
        self.price_schedule_description = description;
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
    /// * `custom_data` - Custom data for this price level schedule, or None to clear
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

    #[test]
    fn test_new_price_level_schedule() {
        let time_anchor = Utc::now();
        let price_schedule_id = 1;
        let number_of_price_levels = 3;
        let entries = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];

        let schedule = PriceLevelScheduleType::new(
            time_anchor.clone(),
            price_schedule_id,
            number_of_price_levels,
            entries.clone(),
        );

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.price_schedule_id(), price_schedule_id);
        assert_eq!(schedule.number_of_price_levels(), number_of_price_levels);
        assert_eq!(schedule.price_level_schedule_entries(), &entries);
        assert_eq!(schedule.price_schedule_description(), None);
        assert_eq!(schedule.custom_data(), None);
    }

    #[test]
    fn test_with_custom_data_and_description() {
        let time_anchor = Utc::now();
        let price_schedule_id = 1;
        let number_of_price_levels = 3;
        let entries = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };
        let description = "Test Schedule".to_string();

        let schedule = PriceLevelScheduleType::new(
            time_anchor.clone(),
            price_schedule_id,
            number_of_price_levels,
            entries.clone(),
        )
        .with_custom_data(custom_data.clone())
        .with_price_schedule_description(description.clone());

        assert_eq!(schedule.time_anchor(), &time_anchor);
        assert_eq!(schedule.price_schedule_id(), price_schedule_id);
        assert_eq!(schedule.number_of_price_levels(), number_of_price_levels);
        assert_eq!(schedule.price_level_schedule_entries(), &entries);
        assert_eq!(schedule.price_schedule_description(), Some(&description));
        assert_eq!(schedule.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let time_anchor1 = Utc::now();
        let price_schedule_id1 = 1;
        let number_of_price_levels1 = 3;
        let entries1 = vec![
            PriceLevelScheduleEntryType::new(3600, 1),
            PriceLevelScheduleEntryType::new(7200, 2),
        ];

        let time_anchor2 = Utc::now();
        let price_schedule_id2 = 2;
        let number_of_price_levels2 = 5;
        let entries2 = vec![
            PriceLevelScheduleEntryType::new(1800, 3),
            PriceLevelScheduleEntryType::new(3600, 2),
            PriceLevelScheduleEntryType::new(5400, 1),
        ];
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };
        let description = "Updated Schedule".to_string();

        let mut schedule = PriceLevelScheduleType::new(
            time_anchor1.clone(),
            price_schedule_id1,
            number_of_price_levels1,
            entries1.clone(),
        );

        schedule
            .set_time_anchor(time_anchor2.clone())
            .set_price_schedule_id(price_schedule_id2)
            .set_number_of_price_levels(number_of_price_levels2)
            .set_price_level_schedule_entries(entries2.clone())
            .set_price_schedule_description(Some(description.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(schedule.time_anchor(), &time_anchor2);
        assert_eq!(schedule.price_schedule_id(), price_schedule_id2);
        assert_eq!(schedule.number_of_price_levels(), number_of_price_levels2);
        assert_eq!(schedule.price_level_schedule_entries(), &entries2);
        assert_eq!(schedule.price_schedule_description(), Some(&description));
        assert_eq!(schedule.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        schedule.set_price_schedule_description(None);
        schedule.set_custom_data(None);
        assert_eq!(schedule.price_schedule_description(), None);
        assert_eq!(schedule.custom_data(), None);
    }
}
