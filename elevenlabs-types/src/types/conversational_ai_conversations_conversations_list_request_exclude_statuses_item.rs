pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationsListRequestExcludeStatusesItem {
    Initiated,
    InProgress,
    Processing,
    Done,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationsListRequestExcludeStatusesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Initiated => serializer.serialize_str("initiated"),
            Self::InProgress => serializer.serialize_str("in-progress"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Done => serializer.serialize_str("done"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationsListRequestExcludeStatusesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "initiated" => Ok(Self::Initiated),
            "in-progress" => Ok(Self::InProgress),
            "processing" => Ok(Self::Processing),
            "done" => Ok(Self::Done),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationsListRequestExcludeStatusesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Initiated => write!(f, "initiated"),
            Self::InProgress => write!(f, "in-progress"),
            Self::Processing => write!(f, "processing"),
            Self::Done => write!(f, "done"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
