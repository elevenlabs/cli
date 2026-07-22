pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The field to use for ordering results from this query.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingListRequestOrderBy {
    CreatedAt,
    Name,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingListRequestOrderBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::Name => serializer.serialize_str("name"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingListRequestOrderBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "name" => Ok(Self::Name),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingListRequestOrderBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::Name => write!(f, "name"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
