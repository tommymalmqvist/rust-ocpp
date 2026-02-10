#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum LogStatusEnumType {
    #[default]
    Accepted,
    Rejected,
    AcceptedCanceled,
}
