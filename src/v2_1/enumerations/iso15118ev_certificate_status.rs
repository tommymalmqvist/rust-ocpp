#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum Iso15118EVCertificateStatusEnumType {
    #[default]
    Accepted,
    Failed,
}
