pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomLlmapiType {
    ChatCompletions,
    Responses,
    Websocket,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CustomLlmapiType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ChatCompletions => serializer.serialize_str("chat_completions"),
            Self::Responses => serializer.serialize_str("responses"),
            Self::Websocket => serializer.serialize_str("websocket"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CustomLlmapiType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "chat_completions" => Ok(Self::ChatCompletions),
            "responses" => Ok(Self::Responses),
            "websocket" => Ok(Self::Websocket),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CustomLlmapiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChatCompletions => write!(f, "chat_completions"),
            Self::Responses => write!(f, "responses"),
            Self::Websocket => write!(f, "websocket"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
