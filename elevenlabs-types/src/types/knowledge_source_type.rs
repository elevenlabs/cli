pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of knowledge source.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnowledgeSourceType {
    File,
    Url,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KnowledgeSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::File => serializer.serialize_str("file"),
            Self::Url => serializer.serialize_str("url"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KnowledgeSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "file" => Ok(Self::File),
            "url" => Ok(Self::Url),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KnowledgeSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Url => write!(f, "url"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
