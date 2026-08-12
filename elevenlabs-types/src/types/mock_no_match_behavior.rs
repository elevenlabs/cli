pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MockNoMatchBehavior {
    CallRealTool,
    RaiseError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MockNoMatchBehavior {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CallRealTool => serializer.serialize_str("call_real_tool"),
            Self::RaiseError => serializer.serialize_str("raise_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MockNoMatchBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "call_real_tool" => Ok(Self::CallRealTool),
            "raise_error" => Ok(Self::RaiseError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MockNoMatchBehavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CallRealTool => write!(f, "call_real_tool"),
            Self::RaiseError => write!(f, "raise_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
