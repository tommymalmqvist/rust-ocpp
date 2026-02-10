use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct CustomString {
    #[validate(length(max = 20, message = "String length must not exceed 20 characters"))]
    pub value: String,
}

/// Standardized values for a chargingLimitSource field.
/// Before OCPP 2.1 this used to be an enumeration. This has been changed to a predefined set of strings for more flexibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChargingLimitSourceEnumType {
    /// Standard OCPP values
    #[serde(rename_all = "UPPERCASE")]
    Standard(StandardChargingLimitSourceEnumType),
    /// Custom charging limit source value
    Custom(CustomString),
}

/// Standard OCPP charging limit source values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandardChargingLimitSourceEnumType {
    /// Indicates that an Energy Management System has sent a charging limit.
    #[serde(rename = "EMS")]
    EMS,
    /// Indicates that an external source, not being an EMS or system operator, has sent a charging limit.
    #[serde(rename = "Other")]
    Other,
    /// Indicates that a System Operator (DSO or TSO) has sent a charging limit.
    #[serde(rename = "SO")]
    SO,
    /// Indicates that the CSO has set this charging profile.
    #[serde(rename = "CSO")]
    CSO,
}

impl ChargingLimitSourceEnumType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standard(s) => match s {
                StandardChargingLimitSourceEnumType::EMS => "EMS",
                StandardChargingLimitSourceEnumType::Other => "Other",
                StandardChargingLimitSourceEnumType::SO => "SO",
                StandardChargingLimitSourceEnumType::CSO => "CSO",
            },
            Self::Custom(s) => &s.value,
        }
    }
}

impl From<String> for ChargingLimitSourceEnumType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "EMS" => Self::Standard(StandardChargingLimitSourceEnumType::EMS),
            "Other" => Self::Standard(StandardChargingLimitSourceEnumType::Other),
            "SO" => Self::Standard(StandardChargingLimitSourceEnumType::SO),
            "CSO" => Self::Standard(StandardChargingLimitSourceEnumType::CSO),
            _ => Self::Custom(CustomString { value: s }),
        }
    }
}

impl ToString for ChargingLimitSourceEnumType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
