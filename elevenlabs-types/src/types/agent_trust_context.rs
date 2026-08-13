pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The trust context in which the agent operates.
/// 
/// UNKNOWN: not yet classified (existing agents created before this feature).
/// LOW: serves untrusted external participants (e.g. customer support, sales) —
/// outputs should be vetted and tool access scoped.
/// HIGH: serves the owner (e.g. personal assistant) — full tool access is appropriate.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentTrustContext {
    Unknown,
    Low,
    High,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentTrustContext {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::Low => serializer.serialize_str("low"),
            Self::High => serializer.serialize_str("high"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentTrustContext {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unknown" => Ok(Self::Unknown),
            "low" => Ok(Self::Low),
            "high" => Ok(Self::High),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentTrustContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Low => write!(f, "low"),
            Self::High => write!(f, "high"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
