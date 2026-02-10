use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{custom_data::CustomDataType, sales_tariff_entry::SalesTariffEntryType};

/// A SalesTariff provided by a Mobility Operator (EMSP).
/// NOTE: This dataType is based on dataTypes from ISO 15118-2.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SalesTariffType {
    /// Required. SalesTariff identifier used to identify one sales tariff.
    /// An SAID remains a unique identifier for one schedule throughout a charging session.
    #[validate(range(min = 0))]
    pub id: i32,

    /// Optional. A human readable title/description of the sales tariff e.g. for HMI display purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 32))]
    pub sales_tariff_description: Option<String>,

    /// Optional. Defines the overall number of distinct price levels used across all provided SalesTariff elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(range(min = 0))]
    pub num_e_price_levels: Option<i32>,

    /// Required. List of sales tariff entries.
    #[validate(length(min = 1, max = 1024), nested)]
    pub sales_tariff_entry: Vec<SalesTariffEntryType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl SalesTariffType {
    /// Creates a new `SalesTariffType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - SalesTariff identifier used to identify one sales tariff
    /// * `sales_tariff_entry` - List of sales tariff entries
    ///
    /// # Returns
    ///
    /// A new instance of `SalesTariffType` with optional fields set to `None`
    pub fn new(id: i32, sales_tariff_entry: Vec<SalesTariffEntryType>) -> Self {
        Self {
            custom_data: None,
            id,
            sales_tariff_description: None,
            num_e_price_levels: None,
            sales_tariff_entry,
        }
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this sales tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Sets the sales tariff description.
    ///
    /// # Arguments
    ///
    /// * `description` - A human readable title/description of the sales tariff
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_sales_tariff_description(mut self, description: String) -> Self {
        self.sales_tariff_description = Some(description);
        self
    }

    /// Sets the number of price levels used across all provided SalesTariff elements.
    ///
    /// # Arguments
    ///
    /// * `num_e_price_levels` - The number of distinct price levels
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_num_e_price_levels(mut self, num_e_price_levels: i32) -> Self {
        self.num_e_price_levels = Some(num_e_price_levels);
        self
    }

    /// Gets the ID.
    ///
    /// # Returns
    ///
    /// The sales tariff identifier
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the ID.
    ///
    /// # Arguments
    ///
    /// * `id` - SalesTariff identifier
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the sales tariff description.
    ///
    /// # Returns
    ///
    /// An optional human readable title/description of the sales tariff
    pub fn sales_tariff_description(&self) -> Option<&str> {
        self.sales_tariff_description.as_deref()
    }

    /// Sets the sales tariff description.
    ///
    /// # Arguments
    ///
    /// * `description` - A human readable title/description of the sales tariff, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sales_tariff_description(&mut self, description: Option<String>) -> &mut Self {
        self.sales_tariff_description = description;
        self
    }

    /// Gets the number of price levels.
    ///
    /// # Returns
    ///
    /// An optional number of distinct price levels used across all provided SalesTariff elements
    pub fn num_e_price_levels(&self) -> Option<i32> {
        self.num_e_price_levels
    }

    /// Sets the number of price levels.
    ///
    /// # Arguments
    ///
    /// * `num_e_price_levels` - The number of distinct price levels, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_num_e_price_levels(&mut self, num_e_price_levels: Option<i32>) -> &mut Self {
        self.num_e_price_levels = num_e_price_levels;
        self
    }

    /// Gets the sales tariff entries.
    ///
    /// # Returns
    ///
    /// A reference to the list of sales tariff entries
    pub fn sales_tariff_entry(&self) -> &[SalesTariffEntryType] {
        &self.sales_tariff_entry
    }

    /// Sets the sales tariff entries.
    ///
    /// # Arguments
    ///
    /// * `sales_tariff_entry` - List of sales tariff entries
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_sales_tariff_entry(
        &mut self,
        sales_tariff_entry: Vec<SalesTariffEntryType>,
    ) -> &mut Self {
        self.sales_tariff_entry = sales_tariff_entry;
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
    /// * `custom_data` - Custom data for this sales tariff, or None to clear
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
    use super::super::relative_time_interval::RelativeTimeIntervalType;
    use super::*;

    #[test]
    fn test_new_sales_tariff() {
        let id = 1;
        let interval = RelativeTimeIntervalType::new_default();
        let entry = SalesTariffEntryType::new(interval);
        let sales_tariff = SalesTariffType::new(id, vec![entry.clone()]);

        assert_eq!(sales_tariff.id(), id);
        assert_eq!(sales_tariff.sales_tariff_description(), None);
        assert_eq!(sales_tariff.num_e_price_levels(), None);
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry);
        assert_eq!(sales_tariff.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = 1;
        let interval = RelativeTimeIntervalType::new_default();
        let entry = SalesTariffEntryType::new(interval);
        let description = "Peak Hours Tariff".to_string();
        let num_levels = 3;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        let sales_tariff = SalesTariffType::new(id, vec![entry.clone()])
            .with_sales_tariff_description(description.clone())
            .with_num_e_price_levels(num_levels)
            .with_custom_data(custom_data.clone());

        assert_eq!(sales_tariff.id(), id);
        assert_eq!(
            sales_tariff.sales_tariff_description(),
            Some(description.as_str())
        );
        assert_eq!(sales_tariff.num_e_price_levels(), Some(num_levels));
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry);
        assert_eq!(sales_tariff.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = 1;
        let interval1 = RelativeTimeIntervalType::new_default();
        let entry1 = SalesTariffEntryType::new(interval1);
        let mut sales_tariff = SalesTariffType::new(id1, vec![entry1]);

        let id2 = 2;
        let interval2 = RelativeTimeIntervalType::new_default();
        let entry2 = SalesTariffEntryType::new(interval2);
        let description = "Off-peak Hours Tariff".to_string();
        let num_levels = 5;
        let custom_data = CustomDataType {
            vendor_id: "VendorX".to_string(),
            additional_properties: Default::default(),
        };

        sales_tariff
            .set_id(id2)
            .set_sales_tariff_description(Some(description.clone()))
            .set_num_e_price_levels(Some(num_levels))
            .set_sales_tariff_entry(vec![entry2.clone()])
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(sales_tariff.id(), id2);
        assert_eq!(
            sales_tariff.sales_tariff_description(),
            Some(description.as_str())
        );
        assert_eq!(sales_tariff.num_e_price_levels(), Some(num_levels));
        assert_eq!(sales_tariff.sales_tariff_entry().len(), 1);
        assert_eq!(sales_tariff.sales_tariff_entry()[0], entry2);
        assert_eq!(sales_tariff.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        sales_tariff
            .set_sales_tariff_description(None)
            .set_num_e_price_levels(None)
            .set_custom_data(None);

        assert_eq!(sales_tariff.sales_tariff_description(), None);
        assert_eq!(sales_tariff.num_e_price_levels(), None);
        assert_eq!(sales_tariff.custom_data(), None);
    }
}
