use serde::{Deserialize, Serialize};

/// Standardized values for a connectorType field.
/// Fixed cable connections have a name that starts with "c" for captive cabled.
/// Socket connections have a name that starts with "s" for socket.
/// Wireless connections have a name that starts with "w" for wireless.
/// Swappable battery types have a name that starts with "b" for battery.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConnectorEnumType {
    /// Standard OCPP connector types
    #[serde(rename_all = "UPPERCASE")]
    Standard(StandardConnectorEnumType),
    /// Custom connector type value
    Custom(String),
}

/// Standard OCPP connector type values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StandardConnectorEnumType {
    /// Combined Charging System 1 (captive cabled) a.k.a. Combo 1
    #[serde(rename = "cCCS1")]
    CCCS1,
    /// Combined Charging System 2 (captive cabled) a.k.a. Combo 2
    #[serde(rename = "cCCS2")]
    CCCS2,
    /// ChaoJi (captive cabled) a.k.a. CHAdeMO 3.0
    #[serde(rename = "cChaoJi")]
    CChaoJi,
    /// JARI G105-1993 (captive cabled) a.k.a. CHAdeMO (captive cabled)
    #[serde(rename = "cG105")]
    CG105,
    /// GB/T 20234.3 DC connector (captive cabled)
    #[serde(rename = "cGBT-DC")]
    CGBTDC,
    /// Light Equipment Combined Charging System IS17017 (captive cabled)
    #[serde(rename = "cLECCS")]
    CLECCS,
    /// Megawatt Charging System (captive cabled)
    #[serde(rename = "cMCS")]
    CMCS,
    /// North American Charging Standard J3400 (captive cabled)
    #[serde(rename = "cNACS")]
    CNACS,
    /// Tesla MagicDock with built-in NACS to CCS1 adapter (captive cabled)
    #[serde(rename = "cNACS-CCS1")]
    CNACSCCS1,
    /// Omni Port with build-in CCS1 to NACS adapter (captive cabled)
    #[serde(rename = "cCCS1-NACS")]
    CCCS1NACS,
    /// Tesla Connector (captive cabled)
    #[serde(rename = "cTesla")]
    CTesla,
    /// IEC62196-2 Type 1 connector (captive cabled) a.k.a. J1772
    #[serde(rename = "cType1")]
    CType1,
    /// IEC62196-2 Type 2 connector (captive cabled) a.k.a. Mennekes connector
    #[serde(rename = "cType2")]
    CType2,
    /// Ultra-ChaoJi for megawatt charging (captive cabled)
    #[serde(rename = "cUltraChaoJi")]
    CUltraChaoJi,
    /// 16A 1 phase IEC60309 socket
    #[serde(rename = "s309-1P-16A")]
    S3091P16A,
    /// 32A 1 phase IEC60309 socket
    #[serde(rename = "s309-1P-32A")]
    S3091P32A,
    /// 16A 3 phase IEC60309 socket
    #[serde(rename = "s309-3P-16A")]
    S3093P16A,
    /// 32A 3 phase IEC60309 socket
    #[serde(rename = "s309-3P-32A")]
    S3093P32A,
    /// UK domestic socket a.k.a. 13Amp
    #[serde(rename = "sBS1361")]
    SBS1361,
    /// CEE 7/7 16A socket. May represent 7/4 and 7/5 a.k.a Schuko
    #[serde(rename = "sCEE-7-7")]
    SCEE77,
    /// IEC62196-2 Type 1 socket a.k.a. J1772
    #[serde(rename = "sType1")]
    SType1,
    /// IEC62196-2 Type 2 socket a.k.a. Mennekes connector
    #[serde(rename = "sType2")]
    SType2,
    /// IEC62196-2 Type 3 socket a.k.a. Scame
    #[serde(rename = "sType3")]
    SType3,
    /// Wireless inductively coupled connection (generic)
    #[serde(rename = "wInductive")]
    WInductive,
    /// Wireless resonant coupled connection (generic)
    #[serde(rename = "wResonant")]
    WResonant,
    /// Other single phase (domestic) sockets not mentioned above, rated at no more than 16A
    #[serde(rename = "Other1PhMax16A")]
    Other1PhMax16A,
    /// Other single phase sockets not mentioned above (over 16A)
    #[serde(rename = "Other1PhOver16A")]
    Other1PhOver16A,
    /// Other 3 phase sockets not mentioned above
    #[serde(rename = "Other3Ph")]
    Other3Ph,
    /// Pantograph connector
    #[serde(rename = "Pan")]
    Pan,
    /// Yet to be determined (e.g. before plugged in)
    #[serde(rename = "Undetermined")]
    Undetermined,
    /// Unknown/not determinable
    #[serde(rename = "Unknown")]
    Unknown,
}

impl ConnectorEnumType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standard(s) => match s {
                StandardConnectorEnumType::CCCS1 => "cCCS1",
                StandardConnectorEnumType::CCCS2 => "cCCS2",
                StandardConnectorEnumType::CChaoJi => "cChaoJi",
                StandardConnectorEnumType::CG105 => "cG105",
                StandardConnectorEnumType::CGBTDC => "cGBT-DC",
                StandardConnectorEnumType::CLECCS => "cLECCS",
                StandardConnectorEnumType::CMCS => "cMCS",
                StandardConnectorEnumType::CNACS => "cNACS",
                StandardConnectorEnumType::CNACSCCS1 => "cNACS-CCS1",
                StandardConnectorEnumType::CCCS1NACS => "cCCS1-NACS",
                StandardConnectorEnumType::CTesla => "cTesla",
                StandardConnectorEnumType::CType1 => "cType1",
                StandardConnectorEnumType::CType2 => "cType2",
                StandardConnectorEnumType::CUltraChaoJi => "cUltraChaoJi",
                StandardConnectorEnumType::S3091P16A => "s309-1P-16A",
                StandardConnectorEnumType::S3091P32A => "s309-1P-32A",
                StandardConnectorEnumType::S3093P16A => "s309-3P-16A",
                StandardConnectorEnumType::S3093P32A => "s309-3P-32A",
                StandardConnectorEnumType::SBS1361 => "sBS1361",
                StandardConnectorEnumType::SCEE77 => "sCEE-7-7",
                StandardConnectorEnumType::SType1 => "sType1",
                StandardConnectorEnumType::SType2 => "sType2",
                StandardConnectorEnumType::SType3 => "sType3",
                StandardConnectorEnumType::WInductive => "wInductive",
                StandardConnectorEnumType::WResonant => "wResonant",
                StandardConnectorEnumType::Other1PhMax16A => "Other1PhMax16A",
                StandardConnectorEnumType::Other1PhOver16A => "Other1PhOver16A",
                StandardConnectorEnumType::Other3Ph => "Other3Ph",
                StandardConnectorEnumType::Pan => "Pan",
                StandardConnectorEnumType::Undetermined => "Undetermined",
                StandardConnectorEnumType::Unknown => "Unknown",
            },
            Self::Custom(s) => s,
        }
    }
}

impl From<String> for ConnectorEnumType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "cCCS1" => Self::Standard(StandardConnectorEnumType::CCCS1),
            "cCCS2" => Self::Standard(StandardConnectorEnumType::CCCS2),
            "cChaoJi" => Self::Standard(StandardConnectorEnumType::CChaoJi),
            "cG105" => Self::Standard(StandardConnectorEnumType::CG105),
            "cGBT-DC" => Self::Standard(StandardConnectorEnumType::CGBTDC),
            "cLECCS" => Self::Standard(StandardConnectorEnumType::CLECCS),
            "cMCS" => Self::Standard(StandardConnectorEnumType::CMCS),
            "cNACS" => Self::Standard(StandardConnectorEnumType::CNACS),
            "cNACS-CCS1" => Self::Standard(StandardConnectorEnumType::CNACSCCS1),
            "cCCS1-NACS" => Self::Standard(StandardConnectorEnumType::CCCS1NACS),
            "cTesla" => Self::Standard(StandardConnectorEnumType::CTesla),
            "cType1" => Self::Standard(StandardConnectorEnumType::CType1),
            "cType2" => Self::Standard(StandardConnectorEnumType::CType2),
            "cUltraChaoJi" => Self::Standard(StandardConnectorEnumType::CUltraChaoJi),
            "s309-1P-16A" => Self::Standard(StandardConnectorEnumType::S3091P16A),
            "s309-1P-32A" => Self::Standard(StandardConnectorEnumType::S3091P32A),
            "s309-3P-16A" => Self::Standard(StandardConnectorEnumType::S3093P16A),
            "s309-3P-32A" => Self::Standard(StandardConnectorEnumType::S3093P32A),
            "sBS1361" => Self::Standard(StandardConnectorEnumType::SBS1361),
            "sCEE-7-7" => Self::Standard(StandardConnectorEnumType::SCEE77),
            "sType1" => Self::Standard(StandardConnectorEnumType::SType1),
            "sType2" => Self::Standard(StandardConnectorEnumType::SType2),
            "sType3" => Self::Standard(StandardConnectorEnumType::SType3),
            "wInductive" => Self::Standard(StandardConnectorEnumType::WInductive),
            "wResonant" => Self::Standard(StandardConnectorEnumType::WResonant),
            "Other1PhMax16A" => Self::Standard(StandardConnectorEnumType::Other1PhMax16A),
            "Other1PhOver16A" => Self::Standard(StandardConnectorEnumType::Other1PhOver16A),
            "Other3Ph" => Self::Standard(StandardConnectorEnumType::Other3Ph),
            "Pan" => Self::Standard(StandardConnectorEnumType::Pan),
            "Undetermined" => Self::Standard(StandardConnectorEnumType::Undetermined),
            "Unknown" => Self::Standard(StandardConnectorEnumType::Unknown),
            _ => Self::Custom(s),
        }
    }
}

impl ToString for ConnectorEnumType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
