pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Answer shape for a per-item intake question.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntakeQuestionKind {
    Text,
    Number,
    Choice,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IntakeQuestionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Number => serializer.serialize_str("number"),
            Self::Choice => serializer.serialize_str("choice"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IntakeQuestionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "number" => Ok(Self::Number),
            "choice" => Ok(Self::Choice),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IntakeQuestionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Number => write!(f, "number"),
            Self::Choice => write!(f, "choice"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
