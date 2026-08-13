pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnowledgeBaseToolStatus {
    Success,
    NoMatchingDocuments,
    NoResults,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KnowledgeBaseToolStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Success => serializer.serialize_str("success"),
            Self::NoMatchingDocuments => serializer.serialize_str("no_matching_documents"),
            Self::NoResults => serializer.serialize_str("no_results"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KnowledgeBaseToolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "success" => Ok(Self::Success),
            "no_matching_documents" => Ok(Self::NoMatchingDocuments),
            "no_results" => Ok(Self::NoResults),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KnowledgeBaseToolStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::NoMatchingDocuments => write!(f, "no_matching_documents"),
            Self::NoResults => write!(f, "no_results"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
