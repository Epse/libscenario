#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(features = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FlightRules {
    VFR,
    IFR,
}

impl FlightRules {
    pub fn to_char(&self) -> char {
        match self {
            FlightRules::VFR => 'V',
            FlightRules::IFR => 'I',
        }
    }
}

impl Default for FlightRules {
    fn default() -> Self {
        FlightRules::IFR
    }
}
