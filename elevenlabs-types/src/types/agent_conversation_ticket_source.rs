pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentConversationTicketSource {
    Qa,
    Agent,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentConversationTicketSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Qa => serializer.serialize_str("qa"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentConversationTicketSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "qa" => Ok(Self::Qa),
            "agent" => Ok(Self::Agent),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentConversationTicketSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Qa => write!(f, "qa"),
            Self::Agent => write!(f, "agent"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
