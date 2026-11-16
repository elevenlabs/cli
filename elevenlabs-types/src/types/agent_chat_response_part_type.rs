pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentChatResponsePartType {
    Start,
    Delta,
    Stop,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentChatResponsePartType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Start => serializer.serialize_str("start"),
            Self::Delta => serializer.serialize_str("delta"),
            Self::Stop => serializer.serialize_str("stop"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentChatResponsePartType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "start" => Ok(Self::Start),
            "delta" => Ok(Self::Delta),
            "stop" => Ok(Self::Stop),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentChatResponsePartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::Delta => write!(f, "delta"),
            Self::Stop => write!(f, "stop"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
