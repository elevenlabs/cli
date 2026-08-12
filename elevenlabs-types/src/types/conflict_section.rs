pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConflictSection {
    ConversationConfig,
    PlatformSettings,
    Procedures,
    Workflow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConflictSection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationConfig => serializer.serialize_str("conversation_config"),
            Self::PlatformSettings => serializer.serialize_str("platform_settings"),
            Self::Procedures => serializer.serialize_str("procedures"),
            Self::Workflow => serializer.serialize_str("workflow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConflictSection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation_config" => Ok(Self::ConversationConfig),
            "platform_settings" => Ok(Self::PlatformSettings),
            "procedures" => Ok(Self::Procedures),
            "workflow" => Ok(Self::Workflow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConflictSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationConfig => write!(f, "conversation_config"),
            Self::PlatformSettings => write!(f, "platform_settings"),
            Self::Procedures => write!(f, "procedures"),
            Self::Workflow => write!(f, "workflow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
