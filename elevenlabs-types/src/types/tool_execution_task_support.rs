pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolExecutionTaskSupport {
    Forbidden,
    Optional,
    Required,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolExecutionTaskSupport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Forbidden => serializer.serialize_str("forbidden"),
            Self::Optional => serializer.serialize_str("optional"),
            Self::Required => serializer.serialize_str("required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolExecutionTaskSupport {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "forbidden" => Ok(Self::Forbidden),
            "optional" => Ok(Self::Optional),
            "required" => Ok(Self::Required),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolExecutionTaskSupport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forbidden => write!(f, "forbidden"),
            Self::Optional => write!(f, "optional"),
            Self::Required => write!(f, "required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
