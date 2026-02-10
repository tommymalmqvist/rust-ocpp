#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum MeasurandEnumType {
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Export.Offered")]
    CurrentExportOffered,
    #[serde(rename = "Current.Export.Minimum")]
    CurrentExportMinimum,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Import.Offered")]
    CurrentImportOffered,
    #[serde(rename = "Current.Import.Minimum")]
    CurrentImportMinimum,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Display.PresentSOC")]
    DisplayPresentSOC,
    #[serde(rename = "Display.MinimumSOC")]
    DisplayMinimumSOC,
    #[serde(rename = "Display.TargetSOC")]
    DisplayTargetSOC,
    #[serde(rename = "Display.MaximumSOC")]
    DisplayMaximumSOC,
    #[serde(rename = "Display.RemainingTimeToMinimumSOC")]
    DisplayRemainingTimeToMinimumSOC,
    #[serde(rename = "Display.RemainingTimeToTargetSOC")]
    DisplayRemainingTimeToTargetSOC,
    #[serde(rename = "Display.RemainingTimeToMaximumSOC")]
    DisplayRemainingTimeToMaximumSOC,
    #[serde(rename = "Display.ChargingComplete")]
    DisplayChargingComplete,
    #[serde(rename = "Display.BatteryEnergyCapacity")]
    DisplayBatteryEnergyCapacity,
    #[serde(rename = "Display.InletHot")]
    DisplayInletHot,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Active.Import.CableLoss")]
    EnergyActiveImportCableLoss,
    #[serde(rename = "Energy.Active.Import.LocalGeneration.Register")]
    EnergyActiveImportLocalGenerationRegister,
    #[serde(rename = "Energy.Active.Net")]
    EnergyActiveNet,
    #[serde(rename = "Energy.Active.Setpoint.Interval")]
    EnergyActiveSetpointInterval,
    #[serde(rename = "Energy.Apparent.Export")]
    EnergyApparentExport,
    #[serde(rename = "Energy.Apparent.Import")]
    EnergyApparentImport,
    #[serde(rename = "Energy.Apparent.Net")]
    EnergyApparentNet,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Reactive.Net")]
    EnergyReactiveNet,
    #[serde(rename = "EnergyRequest.Target")]
    EnergyRequestTarget,
    #[serde(rename = "EnergyRequest.Minimum")]
    EnergyRequestMinimum,
    #[serde(rename = "EnergyRequest.Maximum")]
    EnergyRequestMaximum,
    #[serde(rename = "EnergyRequest.Minimum.V2X")]
    EnergyRequestMinimumV2X,
    #[serde(rename = "EnergyRequest.Maximum.V2X")]
    EnergyRequestMaximumV2X,
    #[serde(rename = "EnergyRequest.Bulk")]
    EnergyRequestBulk,
    #[serde(rename = "Frequency")]
    Frequency,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Active.Setpoint")]
    PowerActiveSetpoint,
    #[serde(rename = "Power.Active.Residual")]
    PowerActiveResidual,
    #[serde(rename = "Power.Export.Minimum")]
    PowerExportMinimum,
    #[serde(rename = "Power.Export.Offered")]
    PowerExportOffered,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Power.Import.Offered")]
    PowerImportOffered,
    #[serde(rename = "Power.Import.Minimum")]
    PowerImportMinimum,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "Voltage")]
    Voltage,
    #[serde(rename = "Voltage.Minimum")]
    VoltageMinimum,
    #[serde(rename = "Voltage.Maximum")]
    VoltageMaximum,
}

impl Default for MeasurandEnumType {
    fn default() -> Self {
        Self::EnergyActiveImportRegister
    }
}
