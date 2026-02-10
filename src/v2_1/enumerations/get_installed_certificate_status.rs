#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GetInstalledCertificateStatusEnumType {
    #[default]
    Accepted,
    NotFound,
}
