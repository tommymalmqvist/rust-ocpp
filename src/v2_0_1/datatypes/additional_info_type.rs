//! Contains a case insensitive identifier to use for the authorization and the type of authorization to support multiple forms of identifiers.
#[cfg(feature = "std")]
use crate::v2_0_1::helpers::validator::validate_identifier_string;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Contains a case insensitive identifier to use for the authorization and the
/// type of authorization to support multiple forms of identifiers.
///
/// AdditionalInfoType is used by: [IdTokenType](`crate::v2_0_1::datatypes::id_token_type::IdTokenType`)
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
#[serde(rename_all = "camelCase")]
pub struct AdditionalInfoType {
    /// This field specifies the additional IdToken
    #[cfg_attr(
        feature = "std",
        validate(
            length(min = 1, max = 36),
            custom(function = "validate_identifier_string")
        )
    )]
    pub additional_id_token: String,
    /// This defines the type of the additionalIdToken. This is a custom type, so the implementation needs to be agreed upon by all involved parties.
    #[cfg_attr(feature = "std", validate(length(min = 0, max = 50)))]
    #[serde(rename = "type")]
    pub kind: String,
}
