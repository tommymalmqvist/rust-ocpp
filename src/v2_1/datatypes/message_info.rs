use super::{
    component::ComponentType, custom_data::CustomDataType, id_token::IdTokenType,
    message_content::MessageContentType,
};
use crate::v2_1::enumerations::{MessagePriorityEnumType, MessageStateEnumType};
use crate::v2_1::helpers::validator::validate_identifier_string;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Contains message details, for a message to be displayed on a Charging Station.
///
/// This type is used in display message related requests and responses to provide
/// information about messages that should be shown on a Charging Station's display.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfoType {
    /// Required. The identifier that identifies this message.
    ///
    /// Unique id within an exchange context. It is defined within the OCPP context
    /// as a positive Integer value (greater or equal to zero).
    #[validate(range(min = 0))]
    pub id: i32,

    /// Required. Priority with which this message should be shown.
    ///
    /// Defines how the message should be prioritized on the Charging Station's display.
    pub priority: MessagePriorityEnumType,

    /// Required. Current state of this message.
    ///
    /// Defines during which state of the Charging Station this message should be shown.
    pub state: MessageStateEnumType,

    /// Required. Date and time at which this message was received.
    ///
    /// From what date-time should this message be shown. If omitted: directly.
    pub start_timestamp: DateTime<Utc>,

    /// Optional. Date and time at which this message should be removed from the display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<DateTime<Utc>>,

    /// Optional. Transaction Id for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 36), custom(function = "validate_identifier_string"))]
    pub transaction_id: Option<String>,

    /// Optional. Message details for a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub message: Option<MessageContentType>,

    /// Optional. Display component that this message concerns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub display: Option<ComponentType>,

    /// Optional. Identification of the token for which this message is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_token: Option<IdTokenType>,

    /// Optional. Additional message details.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub message_extra: Option<MessageContentType>,

    /// Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(nested)]
    pub custom_data: Option<CustomDataType>,
}

impl MessageInfoType {
    /// Creates a new `MessageInfoType` with required fields.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier that identifies this message (must be >= 0)
    /// * `priority` - Priority with which this message should be shown
    /// * `state` - Current state of this message
    /// * `start_timestamp` - Date and time at which this message was received
    ///
    /// # Returns
    ///
    /// A new instance of `MessageInfoType` with optional fields set to `None`
    ///
    /// # Example
    ///
    /// ```
    /// use rust_ocpp::v2_1::datatypes::message_info::MessageInfoType;
    /// use rust_ocpp::v2_1::enumerations::{MessagePriorityEnumType, MessageStateEnumType};
    /// use chrono::Utc;
    ///
    /// let message_info = MessageInfoType::new(
    ///     1,
    ///     MessagePriorityEnumType::AlwaysFront,
    ///     MessageStateEnumType::Idle,
    ///     Utc::now()
    /// );
    /// ```
    pub fn new(
        id: i32,
        priority: MessagePriorityEnumType,
        state: MessageStateEnumType,
        start_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            priority,
            state,
            start_timestamp,
            end_timestamp: None,
            transaction_id: None,
            message: None,
            display: None,
            id_token: None,
            message_extra: None,
            custom_data: None,
        }
    }

    /// Creates a builder for `MessageInfoType` with required fields.
    ///
    /// This is an alternative to using `new()` followed by `with_*` methods.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier that identifies this message (must be >= 0)
    /// * `priority` - Priority with which this message should be shown
    /// * `state` - Current state of this message
    /// * `start_timestamp` - Date and time at which this message was received
    ///
    /// # Returns
    ///
    /// A new instance of `MessageInfoType` with optional fields set to `None`
    pub fn builder(
        id: i32,
        priority: MessagePriorityEnumType,
        state: MessageStateEnumType,
        start_timestamp: DateTime<Utc>,
    ) -> Self {
        Self::new(id, priority, state, start_timestamp)
    }

    /// Sets the end timestamp.
    ///
    /// # Arguments
    ///
    /// * `end_timestamp` - Date and time at which this message should be removed from the display
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_end_timestamp(mut self, end_timestamp: DateTime<Utc>) -> Self {
        self.end_timestamp = Some(end_timestamp);
        self
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - Transaction Id for which this message is intended
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_transaction_id(mut self, transaction_id: String) -> Self {
        self.transaction_id = Some(transaction_id);
        self
    }

    /// Sets the message content.
    ///
    /// # Arguments
    ///
    /// * `message` - Message details for a specific user
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_message(mut self, message: MessageContentType) -> Self {
        self.message = Some(message);
        self
    }

    /// Sets the display component.
    ///
    /// # Arguments
    ///
    /// * `display` - Display component that this message concerns
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_display(mut self, display: ComponentType) -> Self {
        self.display = Some(display);
        self
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - Identification of the token for which this message is intended
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_id_token(mut self, id_token: IdTokenType) -> Self {
        self.id_token = Some(id_token);
        self
    }

    /// Sets the additional message content.
    ///
    /// # Arguments
    ///
    /// * `message_extra` - Additional message details
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_message_extra(mut self, message_extra: MessageContentType) -> Self {
        self.message_extra = Some(message_extra);
        self
    }

    /// Sets the custom data.
    ///
    /// # Arguments
    ///
    /// * `custom_data` - Custom data for this message info
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn with_custom_data(mut self, custom_data: CustomDataType) -> Self {
        self.custom_data = Some(custom_data);
        self
    }

    /// Gets the message identifier.
    ///
    /// # Returns
    ///
    /// The identifier that identifies this message
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Sets the message identifier.
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier that identifies this message (must be >= 0)
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    /// Gets the message priority.
    ///
    /// # Returns
    ///
    /// The priority with which this message should be shown
    pub fn priority(&self) -> &MessagePriorityEnumType {
        &self.priority
    }

    /// Sets the message priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - Priority with which this message should be shown
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_priority(&mut self, priority: MessagePriorityEnumType) -> &mut Self {
        self.priority = priority;
        self
    }

    /// Gets the message state.
    ///
    /// # Returns
    ///
    /// The current state of this message
    pub fn state(&self) -> &MessageStateEnumType {
        &self.state
    }

    /// Sets the message state.
    ///
    /// # Arguments
    ///
    /// * `state` - Current state of this message
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_state(&mut self, state: MessageStateEnumType) -> &mut Self {
        self.state = state;
        self
    }

    /// Gets the start timestamp.
    ///
    /// # Returns
    ///
    /// The date and time at which this message was received
    pub fn start_timestamp(&self) -> &DateTime<Utc> {
        &self.start_timestamp
    }

    /// Sets the start timestamp.
    ///
    /// # Arguments
    ///
    /// * `start_timestamp` - Date and time at which this message was received
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_start_timestamp(&mut self, start_timestamp: DateTime<Utc>) -> &mut Self {
        self.start_timestamp = start_timestamp;
        self
    }

    /// Gets the end timestamp.
    ///
    /// # Returns
    ///
    /// An optional reference to the date and time at which this message should be removed
    pub fn end_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.end_timestamp.as_ref()
    }

    /// Sets the end timestamp.
    ///
    /// # Arguments
    ///
    /// * `end_timestamp` - Date and time at which this message should be removed, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_end_timestamp(&mut self, end_timestamp: Option<DateTime<Utc>>) -> &mut Self {
        self.end_timestamp = end_timestamp;
        self
    }

    /// Gets the transaction ID.
    ///
    /// # Returns
    ///
    /// An optional reference to the transaction Id for which this message is intended
    pub fn transaction_id(&self) -> Option<&str> {
        self.transaction_id.as_deref()
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    ///
    /// * `transaction_id` - Transaction Id for which this message is intended, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_transaction_id(&mut self, transaction_id: Option<String>) -> &mut Self {
        self.transaction_id = transaction_id;
        self
    }

    /// Gets the message content.
    ///
    /// # Returns
    ///
    /// An optional reference to the message details for a specific user
    pub fn message(&self) -> Option<&MessageContentType> {
        self.message.as_ref()
    }

    /// Sets the message content.
    ///
    /// # Arguments
    ///
    /// * `message` - Message details for a specific user, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_message(&mut self, message: Option<MessageContentType>) -> &mut Self {
        self.message = message;
        self
    }

    /// Gets the display component.
    ///
    /// # Returns
    ///
    /// An optional reference to the display component that this message concerns
    pub fn display(&self) -> Option<&ComponentType> {
        self.display.as_ref()
    }

    /// Sets the display component.
    ///
    /// # Arguments
    ///
    /// * `display` - Display component that this message concerns, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_display(&mut self, display: Option<ComponentType>) -> &mut Self {
        self.display = display;
        self
    }

    /// Gets the ID token.
    ///
    /// # Returns
    ///
    /// An optional reference to the identification of the token for which this message is intended
    pub fn id_token(&self) -> Option<&IdTokenType> {
        self.id_token.as_ref()
    }

    /// Sets the ID token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - Identification of the token for which this message is intended, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_id_token(&mut self, id_token: Option<IdTokenType>) -> &mut Self {
        self.id_token = id_token;
        self
    }

    /// Gets the additional message content.
    ///
    /// # Returns
    ///
    /// An optional reference to the additional message details
    pub fn message_extra(&self) -> Option<&MessageContentType> {
        self.message_extra.as_ref()
    }

    /// Sets the additional message content.
    ///
    /// # Arguments
    ///
    /// * `message_extra` - Additional message details, or None to clear
    ///
    /// # Returns
    ///
    /// Self reference for method chaining
    pub fn set_message_extra(&mut self, message_extra: Option<MessageContentType>) -> &mut Self {
        self.message_extra = message_extra;
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
    /// * `custom_data` - Custom data for this message info, or None to clear
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
    use serde_json::{json, Value};

    #[test]
    fn test_new_message_info() {
        let id = 1;
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();

        let message_info =
            MessageInfoType::new(id, priority.clone(), state.clone(), start_timestamp);

        assert_eq!(message_info.id(), id);
        assert_eq!(message_info.priority(), &priority);
        assert_eq!(message_info.state(), &state);
        assert_eq!(message_info.start_timestamp(), &start_timestamp);
        assert_eq!(message_info.end_timestamp(), None);
        assert_eq!(message_info.transaction_id(), None);
        assert_eq!(message_info.message(), None);
        assert_eq!(message_info.display(), None);
        assert_eq!(message_info.id_token(), None);
        assert_eq!(message_info.message_extra(), None);
        assert_eq!(message_info.custom_data(), None);
    }

    #[test]
    fn test_builder() {
        let id = 1;
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();

        let message_info =
            MessageInfoType::builder(id, priority.clone(), state.clone(), start_timestamp);

        assert_eq!(message_info.id(), id);
        assert_eq!(message_info.priority(), &priority);
        assert_eq!(message_info.state(), &state);
        assert_eq!(message_info.start_timestamp(), &start_timestamp);
        assert_eq!(message_info.end_timestamp(), None);
        assert_eq!(message_info.transaction_id(), None);
        assert_eq!(message_info.message(), None);
        assert_eq!(message_info.display(), None);
        assert_eq!(message_info.id_token(), None);
        assert_eq!(message_info.message_extra(), None);
        assert_eq!(message_info.custom_data(), None);
    }

    #[test]
    fn test_with_methods() {
        let id = 1;
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();
        let end_timestamp = start_timestamp + chrono::Duration::hours(1);
        let transaction_id = "TX001".to_string();

        let message_content = MessageContentType::new(
            "Please plug in your vehicle.".to_string(),
            crate::v2_1::enumerations::MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        let message_extra = MessageContentType::new(
            "Additional information".to_string(),
            crate::v2_1::enumerations::MessageFormatEnumType::UTF8,
            "en".to_string(),
        );

        let display = ComponentType::new("MainDisplay".to_string());
        let id_token = IdTokenType::new("TAG123".to_string(), "RFID".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let message_info =
            MessageInfoType::new(id, priority.clone(), state.clone(), start_timestamp)
                .with_end_timestamp(end_timestamp)
                .with_transaction_id(transaction_id.clone())
                .with_message(message_content.clone())
                .with_message_extra(message_extra.clone())
                .with_display(display.clone())
                .with_id_token(id_token.clone())
                .with_custom_data(custom_data.clone());

        assert_eq!(message_info.id(), id);
        assert_eq!(message_info.priority(), &priority);
        assert_eq!(message_info.state(), &state);
        assert_eq!(message_info.start_timestamp(), &start_timestamp);
        assert_eq!(message_info.end_timestamp(), Some(&end_timestamp));
        assert_eq!(message_info.transaction_id(), Some(transaction_id.as_str()));
        assert_eq!(message_info.message(), Some(&message_content));
        assert_eq!(message_info.message_extra(), Some(&message_extra));
        assert_eq!(message_info.display(), Some(&display));
        assert_eq!(message_info.id_token(), Some(&id_token));
        assert_eq!(message_info.custom_data(), Some(&custom_data));
    }

    #[test]
    fn test_setter_methods() {
        let id1 = 1;
        let id2 = 2;
        let priority1 = MessagePriorityEnumType::AlwaysFront;
        let priority2 = MessagePriorityEnumType::InFront;
        let state1 = MessageStateEnumType::Charging;
        let state2 = MessageStateEnumType::Faulted;
        let start_timestamp1 = Utc::now();
        let start_timestamp2 = start_timestamp1 + chrono::Duration::hours(2);
        let end_timestamp = start_timestamp1 + chrono::Duration::hours(1);
        let transaction_id = "TX001".to_string();

        let message_content = MessageContentType::new(
            "Please plug in your vehicle.".to_string(),
            crate::v2_1::enumerations::MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        let message_extra = MessageContentType::new(
            "Additional information".to_string(),
            crate::v2_1::enumerations::MessageFormatEnumType::UTF8,
            "en".to_string(),
        );

        let display = ComponentType::new("MainDisplay".to_string());
        let id_token = IdTokenType::new("TAG123".to_string(), "RFID".to_string());
        let custom_data = CustomDataType::new("VendorX".to_string());

        let mut message_info =
            MessageInfoType::new(id1, priority1.clone(), state1.clone(), start_timestamp1);

        message_info
            .set_id(id2)
            .set_priority(priority2.clone())
            .set_state(state2.clone())
            .set_start_timestamp(start_timestamp2)
            .set_end_timestamp(Some(end_timestamp))
            .set_transaction_id(Some(transaction_id.clone()))
            .set_message(Some(message_content.clone()))
            .set_message_extra(Some(message_extra.clone()))
            .set_display(Some(display.clone()))
            .set_id_token(Some(id_token.clone()))
            .set_custom_data(Some(custom_data.clone()));

        assert_eq!(message_info.id(), id2);
        assert_eq!(message_info.priority(), &priority2);
        assert_eq!(message_info.state(), &state2);
        assert_eq!(message_info.start_timestamp(), &start_timestamp2);
        assert_eq!(message_info.end_timestamp(), Some(&end_timestamp));
        assert_eq!(message_info.transaction_id(), Some(transaction_id.as_str()));
        assert_eq!(message_info.message(), Some(&message_content));
        assert_eq!(message_info.message_extra(), Some(&message_extra));
        assert_eq!(message_info.display(), Some(&display));
        assert_eq!(message_info.id_token(), Some(&id_token));
        assert_eq!(message_info.custom_data(), Some(&custom_data));

        // Test clearing optional fields
        message_info
            .set_end_timestamp(None)
            .set_transaction_id(None)
            .set_message(None)
            .set_message_extra(None)
            .set_display(None)
            .set_id_token(None)
            .set_custom_data(None);

        assert_eq!(message_info.end_timestamp(), None);
        assert_eq!(message_info.transaction_id(), None);
        assert_eq!(message_info.message(), None);
        assert_eq!(message_info.message_extra(), None);
        assert_eq!(message_info.display(), None);
        assert_eq!(message_info.id_token(), None);
        assert_eq!(message_info.custom_data(), None);
    }

    #[test]
    fn test_validation_success() {
        let message_info = MessageInfoType::new(
            1,
            MessagePriorityEnumType::AlwaysFront,
            MessageStateEnumType::Charging,
            Utc::now(),
        );

        // Validation should pass
        assert!(message_info.validate().is_ok());
    }

    #[test]
    fn test_validation_id_range() {
        // Create a message with negative id (invalid)
        let message_info = MessageInfoType {
            id: -1,
            priority: MessagePriorityEnumType::AlwaysFront,
            state: MessageStateEnumType::Charging,
            start_timestamp: Utc::now(),
            end_timestamp: None,
            transaction_id: None,
            message: None,
            display: None,
            id_token: None,
            message_extra: None,
            custom_data: None,
        };

        // Validation should fail
        let result = message_info.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("id"));
    }

    #[test]
    fn test_validation_transaction_id_length() {
        // Create a message with transaction_id that exceeds the maximum length (36 characters)
        let long_transaction_id = "a".repeat(37);
        let message_info = MessageInfoType::new(
            1,
            MessagePriorityEnumType::AlwaysFront,
            MessageStateEnumType::Charging,
            Utc::now(),
        )
        .with_transaction_id(long_transaction_id);

        // Validation should fail
        let result = message_info.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.field_errors().contains_key("transaction_id"));
    }

    #[test]
    fn test_validation_nested_fields() {
        // Create a message with invalid nested fields
        let invalid_message_content = MessageContentType {
            content: "a".repeat(1025), // Exceeds max length of 1024
            format: crate::v2_1::enumerations::MessageFormatEnumType::ASCII,
            language: "en".to_string(),
            custom_data: None,
        };

        let message_info = MessageInfoType::new(
            1,
            MessagePriorityEnumType::AlwaysFront,
            MessageStateEnumType::Charging,
            Utc::now(),
        )
        .with_message(invalid_message_content);

        // Validation should fail due to nested validation
        let result = message_info.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_serialization() {
        let id = 1;
        let priority = MessagePriorityEnumType::AlwaysFront;
        let state = MessageStateEnumType::Charging;
        let start_timestamp = Utc::now();

        let message_content = MessageContentType::new(
            "Please plug in your vehicle.".to_string(),
            crate::v2_1::enumerations::MessageFormatEnumType::ASCII,
            "en".to_string(),
        );

        let custom_data = CustomDataType::new("VendorX".to_string())
            .with_property("version".to_string(), json!("1.0"));

        let message_info = MessageInfoType::new(id, priority, state, start_timestamp)
            .with_message(message_content)
            .with_custom_data(custom_data);

        // Serialize to JSON
        let serialized = serde_json::to_string(&message_info).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();

        // Check that fields are correctly serialized
        assert_eq!(deserialized["id"], id);
        assert_eq!(deserialized["priority"], "AlwaysFront");
        assert_eq!(deserialized["state"], "Charging");
        assert!(deserialized["startTimestamp"].is_string());
        assert!(deserialized["message"].is_object());
        assert_eq!(
            deserialized["message"]["content"],
            "Please plug in your vehicle."
        );
        assert_eq!(deserialized["customData"]["vendorId"], "VendorX");
        assert_eq!(deserialized["customData"]["version"], "1.0");
    }

    #[test]
    fn test_deserialization() {
        // Create JSON with all fields
        let json_with_all_fields = json!({
            "id": 1,
            "priority": "AlwaysFront",
            "state": "Charging",
            "startTimestamp": "2023-01-01T12:00:00Z",
            "endTimestamp": "2023-01-01T13:00:00Z",
            "transactionId": "TX001",
            "message": {
                "content": "Please plug in your vehicle.",
                "format": "ASCII",
                "language": "en"
            },
            "messageExtra": {
                "content": "Additional information",
                "format": "UTF8",
                "language": "en"
            },
            "display": {
                "name": "MainDisplay"
            },
            "idToken": {
                "idToken": "TAG123",
                "type": "RFID"
            },
            "customData": {
                "vendorId": "VendorX"
            }
        });

        // Deserialize
        let message_info: MessageInfoType = serde_json::from_value(json_with_all_fields).unwrap();

        // Check fields
        assert_eq!(message_info.id(), 1);
        assert_eq!(
            message_info.priority(),
            &MessagePriorityEnumType::AlwaysFront
        );
        assert_eq!(message_info.state(), &MessageStateEnumType::Charging);
        assert_eq!(message_info.transaction_id(), Some("TX001"));
        assert!(message_info.message().is_some());
        assert!(message_info.message_extra().is_some());
        assert!(message_info.display().is_some());
        assert!(message_info.id_token().is_some());
        assert!(message_info.custom_data().is_some());

        // Create JSON with only required fields
        let json_required_only = json!({
            "id": 2,
            "priority": "InFront",
            "state": "Idle",
            "startTimestamp": "2023-01-01T12:00:00Z"
        });

        // Deserialize
        let message_info: MessageInfoType = serde_json::from_value(json_required_only).unwrap();

        // Check fields
        assert_eq!(message_info.id(), 2);
        assert_eq!(message_info.priority(), &MessagePriorityEnumType::InFront);
        assert_eq!(message_info.state(), &MessageStateEnumType::Idle);
        assert!(message_info.end_timestamp().is_none());
        assert!(message_info.transaction_id().is_none());
        assert!(message_info.message().is_none());
        assert!(message_info.message_extra().is_none());
        assert!(message_info.display().is_none());
        assert!(message_info.id_token().is_none());
        assert!(message_info.custom_data().is_none());
    }

    #[test]
    fn test_all_message_states() {
        // Test with all possible message states
        let states = vec![
            MessageStateEnumType::Charging,
            MessageStateEnumType::Faulted,
            MessageStateEnumType::Idle,
            MessageStateEnumType::Unavailable,
            MessageStateEnumType::Suspended,
            MessageStateEnumType::Discharging,
        ];

        for state in states {
            let message_info = MessageInfoType::new(
                1,
                MessagePriorityEnumType::AlwaysFront,
                state.clone(),
                Utc::now(),
            );

            assert_eq!(message_info.state(), &state);
        }
    }
}
