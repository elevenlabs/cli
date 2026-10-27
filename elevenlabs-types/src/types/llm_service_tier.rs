pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request-time processing tier for a model.
/// 
/// STANDARD: default price and throughput.
/// FAST: pays a per-token price premium for higher throughput / lower latency.
/// See OpenAI's fast mode: https://openai.com/api-fast-mode/
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlmServiceTier {
    Standard,
    Fast,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LlmServiceTier {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Standard => serializer.serialize_str("standard"),
            Self::Fast => serializer.serialize_str("fast"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LlmServiceTier {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            "fast" => Ok(Self::Fast),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LlmServiceTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "standard"),
            Self::Fast => write!(f, "fast"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
