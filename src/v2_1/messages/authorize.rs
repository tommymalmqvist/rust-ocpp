use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::v2_1::{
    datatypes::{CustomDataType, IdTokenInfoType, IdTokenType, TariffType},
    enumerations::{AuthorizeCertificateStatusEnumType, EnergyTransferModeEnumType},
};

/// Request to start a transaction with the given idToken.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// The X.509 certificate chain presented by EV and encoded in PEM format.
    /// Order of certificates in chain is from leaf up to (but excluding) root certificate.
    /// Only needed in case of central contract validation when Charging Station cannot validate the contract certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(max = 10000))]
    pub certificate: Option<String>,

    /// Required. Contains the identifier that needs to be authorized.
    pub id_token: IdTokenType,

    /// Optional list of OCSP request data for certificates that need to be validated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 1, max = 4))]
    pub iso15118_certificate_hash_data: Option<Vec<OCSPRequestDataType>>,
}

/// Information about a certificate for an OCSP check.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OCSPRequestDataType {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Used algorithms for the hashes provided.
    pub hash_algorithm: HashAlgorithmEnumType,

    /// Required. The hash of the issuer's distinguished name (DN), that must be calculated over the DER
    /// encoding of the issuer's name field in the certificate being checked.
    #[validate(length(max = 128))]
    pub issuer_name_hash: String,

    /// Required. The hash of the DER encoded public key: the value (excluding tag and length) of the subject
    /// public key field in the issuer's certificate.
    #[validate(length(max = 128))]
    pub issuer_key_hash: String,

    /// Required. The string representation of the hexadecimal value of the serial number without the
    /// prefix "0x" and without leading zeroes.
    #[validate(length(max = 40))]
    pub serial_number: String,

    /// Required. This contains the responder URL (Case insensitive).
    #[validate(length(max = 2000))]
    pub responder_url: String,
}

/// Used algorithms for the hashes provided.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum HashAlgorithmEnumType {
    #[serde(rename = "SHA256")]
    SHA256,
    #[serde(rename = "SHA384")]
    SHA384,
    #[serde(rename = "SHA512")]
    SHA512,
}

/// Response to an AuthorizeRequest.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    /// Optional. Custom data from the Charging Station.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<CustomDataType>,

    /// Required. Contains information about authorization status, expiry and group id.
    pub id_token_info: IdTokenInfoType,

    /// Optional. Certificate status information.
    /// - if all certificates are valid: return 'Accepted'.
    /// - if one of the certificates was revoked, return 'CertificateRevoked'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<AuthorizeCertificateStatusEnumType>,

    /// Optional. List of allowed energy transfer modes the EV can choose from. If omitted this defaults to charging only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_energy_transfer: Option<Vec<EnergyTransferModeEnumType>>,

    /// Optional. The tariff that is applied to this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tariff: Option<TariffType>,
}
