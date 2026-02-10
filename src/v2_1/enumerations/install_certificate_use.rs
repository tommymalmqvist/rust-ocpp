#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum InstallCertificateUseEnumType {
    V2GRootCertificate,
    MORootCertificate,
    ManufacturerRootCertificate,
    #[default]
    CSMSRootCertificate,
    OEMRootCertificate,
}
