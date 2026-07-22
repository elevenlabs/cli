pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnDetectionType {
    SemanticVad,
    ServerVad,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TurnDetectionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SemanticVad => serializer.serialize_str("semantic_vad"),
            Self::ServerVad => serializer.serialize_str("server_vad"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TurnDetectionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "semantic_vad" => Ok(Self::SemanticVad),
            "server_vad" => Ok(Self::ServerVad),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TurnDetectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SemanticVad => write!(f, "semantic_vad"),
            Self::ServerVad => write!(f, "server_vad"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
