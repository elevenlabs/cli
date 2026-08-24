pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Controls how tool responses are filtered before being visible to the agent.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResponseFilterMode {
    All,
    Allow,
    HideAll,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResponseFilterMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Allow => serializer.serialize_str("allow"),
            Self::HideAll => serializer.serialize_str("hide_all"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResponseFilterMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "allow" => Ok(Self::Allow),
            "hide_all" => Ok(Self::HideAll),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResponseFilterMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Allow => write!(f, "allow"),
            Self::HideAll => write!(f, "hide_all"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
