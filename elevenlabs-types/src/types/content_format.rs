pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Canonical representation of a knowledge base document's stored content.
/// 
/// HTML is the legacy default; documents created before this field existed are
/// interpreted as HTML.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ContentFormat {
    Html,
    Markdown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ContentFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Html => serializer.serialize_str("html"),
            Self::Markdown => serializer.serialize_str("markdown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ContentFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "html" => Ok(Self::Html),
            "markdown" => Ok(Self::Markdown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ContentFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Html => write!(f, "html"),
            Self::Markdown => write!(f, "markdown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
