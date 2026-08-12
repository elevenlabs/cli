pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationHistoryTranscriptOtherToolsResultCommonModelType {
    Client,
    Webhook,
    Mcp,
    Code,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationHistoryTranscriptOtherToolsResultCommonModelType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Client => serializer.serialize_str("client"),
            Self::Webhook => serializer.serialize_str("webhook"),
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Code => serializer.serialize_str("code"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationHistoryTranscriptOtherToolsResultCommonModelType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client" => Ok(Self::Client),
            "webhook" => Ok(Self::Webhook),
            "mcp" => Ok(Self::Mcp),
            "code" => Ok(Self::Code),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationHistoryTranscriptOtherToolsResultCommonModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Client => write!(f, "client"),
            Self::Webhook => write!(f, "webhook"),
            Self::Mcp => write!(f, "mcp"),
            Self::Code => write!(f, "code"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
