pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpeechHistoryItemResponseVoiceCategory {
    Premade,
    Cloned,
    Generated,
    Professional,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpeechHistoryItemResponseVoiceCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Premade => serializer.serialize_str("premade"),
            Self::Cloned => serializer.serialize_str("cloned"),
            Self::Generated => serializer.serialize_str("generated"),
            Self::Professional => serializer.serialize_str("professional"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpeechHistoryItemResponseVoiceCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "premade" => Ok(Self::Premade),
            "cloned" => Ok(Self::Cloned),
            "generated" => Ok(Self::Generated),
            "professional" => Ok(Self::Professional),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpeechHistoryItemResponseVoiceCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Premade => write!(f, "premade"),
            Self::Cloned => write!(f, "cloned"),
            Self::Generated => write!(f, "generated"),
            Self::Professional => write!(f, "professional"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
