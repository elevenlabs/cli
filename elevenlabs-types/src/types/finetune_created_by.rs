pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinetuneCreatedBy {
    Self,
    Workspace,
    Elevenlabs,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinetuneCreatedBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Self => serializer.serialize_str("self"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::Elevenlabs => serializer.serialize_str("elevenlabs"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinetuneCreatedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "self" => Ok(Self::Self),
            "workspace" => Ok(Self::Workspace),
            "elevenlabs" => Ok(Self::Elevenlabs),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinetuneCreatedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Self => write!(f, "self"),
            Self::Workspace => write!(f, "workspace"),
            Self::Elevenlabs => write!(f, "elevenlabs"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
