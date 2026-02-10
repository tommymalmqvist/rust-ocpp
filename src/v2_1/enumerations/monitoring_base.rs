#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MonitoringBaseEnumType {
    #[default]
    All,
    FactoryDefault,
    HardWiredOnly,
}
