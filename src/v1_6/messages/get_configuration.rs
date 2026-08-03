#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use crate::v1_6::types::KeyValue;

/// This contains the field definition of the GetConfiguration.req PDU sent by the Central System to the Charge Point. See also Get Configuration
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    /// Optional. List of keys for which the configuration value is requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 50)))]
    pub key: Option<Vec<String>>,
}

/// This contains the field definition of the GetConfiguration.conf PDU sent by Charge Point the to the Central System in response to a GetConfiguration.req. See also Get Configuration
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    /// Optional. List of requested or known keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_key: Option<Vec<KeyValue>>,
    /// Optional. Requested keys that are unknown
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 50)))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_key: Option<Vec<String>>,
}
