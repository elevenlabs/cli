pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Valid Twilio edge locations.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TwilioEdgeLocation {
    Ashburn,
    Dublin,
    Frankfurt,
    SaoPaulo,
    Singapore,
    Sydney,
    Tokyo,
    Umatilla,
    Roaming,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TwilioEdgeLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ashburn => serializer.serialize_str("ashburn"),
            Self::Dublin => serializer.serialize_str("dublin"),
            Self::Frankfurt => serializer.serialize_str("frankfurt"),
            Self::SaoPaulo => serializer.serialize_str("sao-paulo"),
            Self::Singapore => serializer.serialize_str("singapore"),
            Self::Sydney => serializer.serialize_str("sydney"),
            Self::Tokyo => serializer.serialize_str("tokyo"),
            Self::Umatilla => serializer.serialize_str("umatilla"),
            Self::Roaming => serializer.serialize_str("roaming"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TwilioEdgeLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ashburn" => Ok(Self::Ashburn),
            "dublin" => Ok(Self::Dublin),
            "frankfurt" => Ok(Self::Frankfurt),
            "sao-paulo" => Ok(Self::SaoPaulo),
            "singapore" => Ok(Self::Singapore),
            "sydney" => Ok(Self::Sydney),
            "tokyo" => Ok(Self::Tokyo),
            "umatilla" => Ok(Self::Umatilla),
            "roaming" => Ok(Self::Roaming),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TwilioEdgeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ashburn => write!(f, "ashburn"),
            Self::Dublin => write!(f, "dublin"),
            Self::Frankfurt => write!(f, "frankfurt"),
            Self::SaoPaulo => write!(f, "sao-paulo"),
            Self::Singapore => write!(f, "singapore"),
            Self::Sydney => write!(f, "sydney"),
            Self::Tokyo => write!(f, "tokyo"),
            Self::Umatilla => write!(f, "umatilla"),
            Self::Roaming => write!(f, "roaming"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
