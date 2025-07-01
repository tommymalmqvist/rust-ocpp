/// Status in TriggerMessageResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UnitOfMeasure {
    /// Watt-hours (energy). Default.
    #[default]
    Wh,
    /// kiloWatt-hours (energy).
    #[serde(rename = "kWh")]
    KWh,
    /// Var-hours (reactive energy).
    #[serde(rename = "varh")]
    Varh,
    /// kilovar-hours (reactive energy).
    #[serde(rename = "kvarh")]
    Kvarh,
    /// Watts (power).
    W,
    /// kilowatts (power).
    #[serde(rename = "kW")]
    Kw,
    /// VoltAmpere (apparent power).
    #[serde(rename = "VA")]
    Va,
    /// kiloVolt Ampere (apparent power).
    #[serde(rename = "kVA")]
    Kva,
    /// Vars (reactive power).
    #[serde(rename = "var")]
    Var,
    /// kilovars (reactive power).
    #[serde(rename = "kvar")]
    Kvar,
    /// Amperes (current).
    A,
    /// Voltage (r.m.s. AC).
    V,
    /// Degrees (temperature).
    /// Also allow Celcius as an alias for compatibility - see OCPP 1.6 errata
    #[serde(alias = "Celcius")]
    Celsius,
    /// Degrees (temperature).
    Fahrenheit,
    /// Degrees Kelvin (temperature).
    K,
    /// Percentage.
    Percent,
}
