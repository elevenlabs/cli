pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The category of the voice.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceCategory {
    Generated,
    Cloned,
    Premade,
    Professional,
    Famous,
    HighQuality,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Generated => serializer.serialize_str("generated"),
            Self::Cloned => serializer.serialize_str("cloned"),
            Self::Premade => serializer.serialize_str("premade"),
            Self::Professional => serializer.serialize_str("professional"),
            Self::Famous => serializer.serialize_str("famous"),
            Self::HighQuality => serializer.serialize_str("high_quality"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "generated" => Ok(Self::Generated),
            "cloned" => Ok(Self::Cloned),
            "premade" => Ok(Self::Premade),
            "professional" => Ok(Self::Professional),
            "famous" => Ok(Self::Famous),
            "high_quality" => Ok(Self::HighQuality),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generated => write!(f, "generated"),
            Self::Cloned => write!(f, "cloned"),
            Self::Premade => write!(f, "premade"),
            Self::Professional => write!(f, "professional"),
            Self::Famous => write!(f, "famous"),
            Self::HighQuality => write!(f, "high_quality"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
