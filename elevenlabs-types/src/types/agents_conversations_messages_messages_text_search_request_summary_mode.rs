pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether to include transcript summaries in the response.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessagesTextSearchRequestSummaryMode {
    Exclude,
    Include,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MessagesTextSearchRequestSummaryMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exclude => serializer.serialize_str("exclude"),
            Self::Include => serializer.serialize_str("include"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MessagesTextSearchRequestSummaryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "exclude" => Ok(Self::Exclude),
            "include" => Ok(Self::Include),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MessagesTextSearchRequestSummaryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exclude => write!(f, "exclude"),
            Self::Include => write!(f, "include"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
