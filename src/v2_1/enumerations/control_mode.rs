use serde::{Deserialize, Serialize};

/// Indicates whether EV wants to operate in Dynamic or Scheduled mode.
/// When absent, Scheduled mode is assumed for backwards compatibility.
///
/// ISO 15118-20:
/// ServiceSelectionReq(SelectedEnergyTransferService)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlModeEnumType {
    #[serde(rename = "ScheduledControl")]
    ScheduledControl,
    #[serde(rename = "DynamicControl")]
    DynamicControl,
}

impl Default for ControlModeEnumType {
    fn default() -> Self {
        Self::ScheduledControl
    }
}
