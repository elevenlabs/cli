pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Exit branch taken when the Genesys Bot Connector session ends.
/// 
/// These values are the intent names registered in the Genesys bot list, so they are
/// what the Call Bot Connector node branches on. Adding a value here without also
/// registering the intent in Genesys would produce an exit the flow cannot handle.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenesysBotOutcome {
    Success,
    Escalate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GenesysBotOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::Escalate => serializer.serialize_str("escalate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GenesysBotOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "escalate" => Ok(Self::Escalate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GenesysBotOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Escalate => write!(f, "escalate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
