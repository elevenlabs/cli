pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Sort by creation time; newest first by default.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectListRequestSortDirection {
    Ascending,
    Descending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectListRequestSortDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ascending => serializer.serialize_str("ASCENDING"),
            Self::Descending => serializer.serialize_str("DESCENDING"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectListRequestSortDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ASCENDING" => Ok(Self::Ascending),
            "DESCENDING" => Ok(Self::Descending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectListRequestSortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ascending => write!(f, "ASCENDING"),
            Self::Descending => write!(f, "DESCENDING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
