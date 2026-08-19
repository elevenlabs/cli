pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TopicSortBy {
    Conversations,
    Sentiment,
    SuccessRate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TopicSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Conversations => serializer.serialize_str("conversations"),
            Self::Sentiment => serializer.serialize_str("sentiment"),
            Self::SuccessRate => serializer.serialize_str("success_rate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TopicSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversations" => Ok(Self::Conversations),
            "sentiment" => Ok(Self::Sentiment),
            "success_rate" => Ok(Self::SuccessRate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TopicSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conversations => write!(f, "conversations"),
            Self::Sentiment => write!(f, "sentiment"),
            Self::SuccessRate => write!(f, "success_rate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
