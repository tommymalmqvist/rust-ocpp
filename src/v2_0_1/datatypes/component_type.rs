//! A physical or logical component
use super::evse_type::EVSEType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// A Physical or Logcal component
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct ComponentType {
    /// Name of the component. Name should be taken from the list of standardized component names whenever possible. Case Insensitive. strongly advised to use Camel Case
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 50)))]
    pub name: String,
    /// Name of instance in case the component exists as multiple instances. Case Insensitive. strongly advised to use Camel Case
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// Specifies the EVSE when component is located at EVSE level, also specifies the connector when component is located at Connector level
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSEType>,
}
