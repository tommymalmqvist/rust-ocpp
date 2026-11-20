// DONE: Fix -based ones! Must rename with serde FIX THIS
/// Allowed values of ConnectorCode.
/// This enumeration does not attempt to include every possible power connector type worldwide as an individual type, but to specifically define those that are known to be in use (or likely to be in use) in the Charging Stations using the OCPP protocol. In particular, many of the very large number of domestic electrical sockets designs in use in many countries are excluded, unless there is evidence that they are or are likely to be approved for use on Charging Stations in some jurisdictions (e.g. as secondary connectors for charging light EVs such as electric scooters). These light connector types can be represented with the enumeration value Other1PhMax16A. Similarly, any single phase connector not otherwise enumerated that is rated for 16A or over should be reported as Other1PhOver16A. All 3 phase connector types not explicitly enumerated should be represented as Other3Ph.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Default)]
pub enum ConnectorEnumType {
    #[serde(rename = "cCCS1")]
    CCCS1,
    #[serde(rename = "cCCS2")]
    CCCS2,
    #[serde(rename = "cG105")]
    CG105,
    #[serde(rename = "cTesla")]
    CTesla,
    #[serde(rename = "cType1")]
    CType1,
    #[serde(rename = "cType2")]
    CType2,
    #[serde(rename = "s309-1P-16A")]
    S3091P16A,
    #[serde(rename = "s309-1P-32A")]
    S3091P32A,
    #[serde(rename = "s309-3P-16A")]
    S3093P16A,
    #[serde(rename = "s309-3P-32A")]
    S3093P32A,
    #[serde(rename = "sBS1361")]
    SBS1361,
    #[serde(rename = "sCEE-7-7")]
    SCEE77,
    #[serde(rename = "sType2")]
    SType2,
    #[serde(rename = "sType3")]
    SType3,
    Other1PhMax16A,
    Other1PhOver16A,
    Other3Ph,
    Pan,
    #[serde(rename = "wInductive")]
    WInductive,
    #[serde(rename = "wResonant")]
    WResonant,
    Undetermined,
    #[default]
    Unknown,
}
