#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MobilityNeedsModeEnumType {
    #[default]
    #[serde(rename = "EVCC")]
    EVCC,
    #[serde(rename = "EVCC_SECC")]
    EVCCSECC,
}
