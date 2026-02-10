#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum LogEnumType {
    #[default]
    DiagnosticsLog,
    SecurityLog,
    DataCollectorLog,
}
