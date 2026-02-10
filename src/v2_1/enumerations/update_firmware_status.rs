#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UpdateFirmwareStatusEnumType {
    #[default]
    #[serde(rename = "Accepted")]
    Accepted,
    #[serde(rename = "Rejected")]
    Rejected,
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,
    #[serde(rename = "InvalidCertificate")]
    InvalidCertificate,
    #[serde(rename = "RevokedCertificate")]
    RevokedCertificate,
}
