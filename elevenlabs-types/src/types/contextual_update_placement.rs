pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Where the LLM view renders a contextual update.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContextualUpdatePlacement {
    InPlace,
    End,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ContextualUpdatePlacement {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InPlace => serializer.serialize_str("in_place"),
            Self::End => serializer.serialize_str("end"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContextualUpdatePlacement {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_place" => Ok(Self::InPlace),
            "end" => Ok(Self::End),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ContextualUpdatePlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InPlace => write!(f, "in_place"),
            Self::End => write!(f, "end"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
