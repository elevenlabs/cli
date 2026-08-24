pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which product surface owns this agent document.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationProduct {
    Agents,
    SpeechEngine,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationProduct {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Agents => serializer.serialize_str("agents"),
            Self::SpeechEngine => serializer.serialize_str("speech_engine"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationProduct {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "agents" => Ok(Self::Agents),
            "speech_engine" => Ok(Self::SpeechEngine),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agents => write!(f, "agents"),
            Self::SpeechEngine => write!(f, "speech_engine"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
