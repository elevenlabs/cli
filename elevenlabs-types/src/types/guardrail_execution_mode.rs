pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GuardrailExecutionMode {
    Streaming,
    Blocking,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GuardrailExecutionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Streaming => serializer.serialize_str("streaming"),
            Self::Blocking => serializer.serialize_str("blocking"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GuardrailExecutionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "streaming" => Ok(Self::Streaming),
            "blocking" => Ok(Self::Blocking),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GuardrailExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Streaming => write!(f, "streaming"),
            Self::Blocking => write!(f, "blocking"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
