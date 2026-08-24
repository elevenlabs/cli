pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UsersSortBy {
    LastContactUnixSecs,
    ConversationCount,
    AverageSentimentScore,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UsersSortBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LastContactUnixSecs => serializer.serialize_str("last_contact_unix_secs"),
            Self::ConversationCount => serializer.serialize_str("conversation_count"),
            Self::AverageSentimentScore => serializer.serialize_str("average_sentiment_score"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UsersSortBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "last_contact_unix_secs" => Ok(Self::LastContactUnixSecs),
            "conversation_count" => Ok(Self::ConversationCount),
            "average_sentiment_score" => Ok(Self::AverageSentimentScore),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UsersSortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LastContactUnixSecs => write!(f, "last_contact_unix_secs"),
            Self::ConversationCount => write!(f, "conversation_count"),
            Self::AverageSentimentScore => write!(f, "average_sentiment_score"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
