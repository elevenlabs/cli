pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Buckets that make up the non-LLM ``platform_charge`` of a conversation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlatformCategory {
    Voice,
    Silence,
    Burst,
    Asr,
    TextMessage,
    Reception,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlatformCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Voice => serializer.serialize_str("voice"),
            Self::Silence => serializer.serialize_str("silence"),
            Self::Burst => serializer.serialize_str("burst"),
            Self::Asr => serializer.serialize_str("asr"),
            Self::TextMessage => serializer.serialize_str("text_message"),
            Self::Reception => serializer.serialize_str("reception"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlatformCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "voice" => Ok(Self::Voice),
            "silence" => Ok(Self::Silence),
            "burst" => Ok(Self::Burst),
            "asr" => Ok(Self::Asr),
            "text_message" => Ok(Self::TextMessage),
            "reception" => Ok(Self::Reception),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlatformCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Voice => write!(f, "voice"),
            Self::Silence => write!(f, "silence"),
            Self::Burst => write!(f, "burst"),
            Self::Asr => write!(f, "asr"),
            Self::TextMessage => write!(f, "text_message"),
            Self::Reception => write!(f, "reception"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
