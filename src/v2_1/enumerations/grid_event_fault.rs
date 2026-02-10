#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum GridEventFaultEnumType {
    #[default]
    CurrentImbalance,
    LocalEmergency,
    LowInputPower,
    OverCurrent,
    OverFrequency,
    OverVoltage,
    PhaseRotation,
    RemoteEmergency,
    UnderFrequency,
    UnderVoltage,
    VoltageImbalance,
}
