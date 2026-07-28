pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserFeedbackScore {
    Like,
    Dislike,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserFeedbackScore {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Like => serializer.serialize_str("like"),
            Self::Dislike => serializer.serialize_str("dislike"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserFeedbackScore {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "like" => Ok(Self::Like),
            "dislike" => Ok(Self::Dislike),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserFeedbackScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Like => write!(f, "like"),
            Self::Dislike => write!(f, "dislike"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
