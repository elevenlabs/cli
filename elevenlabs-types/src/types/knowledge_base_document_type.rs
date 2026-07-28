pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnowledgeBaseDocumentType {
    File,
    Url,
    Text,
    Folder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KnowledgeBaseDocumentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::File => serializer.serialize_str("file"),
            Self::Url => serializer.serialize_str("url"),
            Self::Text => serializer.serialize_str("text"),
            Self::Folder => serializer.serialize_str("folder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KnowledgeBaseDocumentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "file" => Ok(Self::File),
            "url" => Ok(Self::Url),
            "text" => Ok(Self::Text),
            "folder" => Ok(Self::Folder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KnowledgeBaseDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Url => write!(f, "url"),
            Self::Text => write!(f, "text"),
            Self::Folder => write!(f, "folder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
