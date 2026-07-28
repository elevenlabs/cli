use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "productionUs")]
    ProductionUs,
    #[serde(rename = "productionEu")]
    ProductionEu,
    #[serde(rename = "productionIndia")]
    ProductionIndia,
    #[serde(rename = "productionSingapore")]
    ProductionSingapore,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Production => "https://api.elevenlabs.io",
            Self::ProductionUs => "https://api.us.elevenlabs.io",
            Self::ProductionEu => "https://api.eu.residency.elevenlabs.io",
            Self::ProductionIndia => "https://api.in.residency.elevenlabs.io",
            Self::ProductionSingapore => "https://api.sg.residency.elevenlabs.io",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Production
    }
}
