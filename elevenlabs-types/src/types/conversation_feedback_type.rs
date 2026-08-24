pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationFeedbackType {
    Thumbs,
    Rating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationFeedbackType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Thumbs => serializer.serialize_str("thumbs"),
            Self::Rating => serializer.serialize_str("rating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationFeedbackType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "thumbs" => Ok(Self::Thumbs),
            "rating" => Ok(Self::Rating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationFeedbackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thumbs => write!(f, "thumbs"),
            Self::Rating => write!(f, "rating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
