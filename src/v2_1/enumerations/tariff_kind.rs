#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum TariffKindEnumType {
    #[default]
    #[serde(rename = "Charging")]
    Charging,
    #[serde(rename = "Parking")]
    Parking,
}
