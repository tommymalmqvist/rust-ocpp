#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum LocationEnumType {
    Body,
    Cable,
    EV,
    Inlet,
    #[default]
    Outlet,
}
