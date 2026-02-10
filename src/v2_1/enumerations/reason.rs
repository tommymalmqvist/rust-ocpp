#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ReasonEnumType {
    #[serde(rename = "DeAuthorized")]
    DeAuthorized,
    #[serde(rename = "EmergencyStop")]
    EmergencyStop,
    #[serde(rename = "EnergyLimitReached")]
    EnergyLimitReached,
    #[serde(rename = "EVDisconnected")]
    EVDisconnected,
    #[serde(rename = "GroundFault")]
    GroundFault,
    #[serde(rename = "ImmediateReset")]
    ImmediateReset,
    #[serde(rename = "MasterPass")]
    MasterPass,
    #[default]
    #[serde(rename = "Local")]
    Local,
    #[serde(rename = "LocalOutOfCredit")]
    LocalOutOfCredit,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "OvercurrentFault")]
    OvercurrentFault,
    #[serde(rename = "PowerLoss")]
    PowerLoss,
    #[serde(rename = "PowerQuality")]
    PowerQuality,
    #[serde(rename = "Reboot")]
    Reboot,
    #[serde(rename = "Remote")]
    Remote,
    #[serde(rename = "SOCLimitReached")]
    SOCLimitReached,
    #[serde(rename = "StoppedByEV")]
    StoppedByEV,
    #[serde(rename = "TimeLimitReached")]
    TimeLimitReached,
    #[serde(rename = "Timeout")]
    Timeout,
    #[serde(rename = "ReqEnergyTransferRejected")]
    ReqEnergyTransferRejected,
}
