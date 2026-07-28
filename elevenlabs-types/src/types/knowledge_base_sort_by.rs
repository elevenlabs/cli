pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnowledgeBaseSortBy {
    Name,
    CreatedAt,
    UpdatedAt,
    Size,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for KnowledgeBaseSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Name => serializer.serialize_str("name"),
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::UpdatedAt => serializer.serialize_str("updated_at"),
            Self::Size => serializer.serialize_str("size"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for KnowledgeBaseSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "name" => Ok(Self::Name),
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            "size" => Ok(Self::Size),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for KnowledgeBaseSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::Size => write!(f, "size"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
