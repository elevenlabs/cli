pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RagIndexStatus {
    New,
    Created,
    Processing,
    Failed,
    Succeeded,
    RagLimitExceeded,
    DocumentTooSmall,
    CannotIndexFolder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RagIndexStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("new"),
            Self::Created => serializer.serialize_str("created"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::RagLimitExceeded => serializer.serialize_str("rag_limit_exceeded"),
            Self::DocumentTooSmall => serializer.serialize_str("document_too_small"),
            Self::CannotIndexFolder => serializer.serialize_str("cannot_index_folder"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RagIndexStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "new" => Ok(Self::New),
            "created" => Ok(Self::Created),
            "processing" => Ok(Self::Processing),
            "failed" => Ok(Self::Failed),
            "succeeded" => Ok(Self::Succeeded),
            "rag_limit_exceeded" => Ok(Self::RagLimitExceeded),
            "document_too_small" => Ok(Self::DocumentTooSmall),
            "cannot_index_folder" => Ok(Self::CannotIndexFolder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RagIndexStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Created => write!(f, "created"),
            Self::Processing => write!(f, "processing"),
            Self::Failed => write!(f, "failed"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::RagLimitExceeded => write!(f, "rag_limit_exceeded"),
            Self::DocumentTooSmall => write!(f, "document_too_small"),
            Self::CannotIndexFolder => write!(f, "cannot_index_folder"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
