pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Sort mode for listing tests. Use 'folders_first' to place folders before tests.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TestsListRequestSortMode {
    Default,
    FoldersFirst,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TestsListRequestSortMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Default => serializer.serialize_str("default"),
            Self::FoldersFirst => serializer.serialize_str("folders_first"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TestsListRequestSortMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "default" => Ok(Self::Default),
            "folders_first" => Ok(Self::FoldersFirst),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TestsListRequestSortMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::FoldersFirst => write!(f, "folders_first"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
