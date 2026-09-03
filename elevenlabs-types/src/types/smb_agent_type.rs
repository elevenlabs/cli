pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmbAgentType {
    CustomerFacing,
    Assistant,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SmbAgentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomerFacing => serializer.serialize_str("customer_facing"),
            Self::Assistant => serializer.serialize_str("assistant"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SmbAgentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer_facing" => Ok(Self::CustomerFacing),
            "assistant" => Ok(Self::Assistant),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SmbAgentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomerFacing => write!(f, "customer_facing"),
            Self::Assistant => write!(f, "assistant"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
