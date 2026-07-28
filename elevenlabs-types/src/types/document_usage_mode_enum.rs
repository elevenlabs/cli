pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DocumentUsageModeEnum {
    Prompt,
    Auto,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DocumentUsageModeEnum {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Prompt => serializer.serialize_str("prompt"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DocumentUsageModeEnum {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "prompt" => Ok(Self::Prompt),
            "auto" => Ok(Self::Auto),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DocumentUsageModeEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Prompt => write!(f, "prompt"),
            Self::Auto => write!(f, "auto"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
