pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnowledgeBaseRagToolStatus {
    Success,
    NoDocuments,
    NoResults,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KnowledgeBaseRagToolStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::NoDocuments => serializer.serialize_str("no_documents"),
            Self::NoResults => serializer.serialize_str("no_results"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KnowledgeBaseRagToolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "no_documents" => Ok(Self::NoDocuments),
            "no_results" => Ok(Self::NoResults),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KnowledgeBaseRagToolStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::NoDocuments => write!(f, "no_documents"),
            Self::NoResults => write!(f, "no_results"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
