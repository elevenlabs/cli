pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The order direction to use for results from this query.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DubbingListRequestOrderDirection {
    Descending,
    Ascending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DubbingListRequestOrderDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Descending => serializer.serialize_str("DESCENDING"),
            Self::Ascending => serializer.serialize_str("ASCENDING"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DubbingListRequestOrderDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DESCENDING" => Ok(Self::Descending),
            "ASCENDING" => Ok(Self::Ascending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DubbingListRequestOrderDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Descending => write!(f, "DESCENDING"),
            Self::Ascending => write!(f, "ASCENDING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
