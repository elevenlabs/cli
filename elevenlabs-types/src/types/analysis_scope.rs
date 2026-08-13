pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnalysisScope {
    Conversation,
    Agent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AnalysisScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Conversation => serializer.serialize_str("conversation"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AnalysisScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation" => Ok(Self::Conversation),
            "agent" => Ok(Self::Agent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AnalysisScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conversation => write!(f, "conversation"),
            Self::Agent => write!(f, "agent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
