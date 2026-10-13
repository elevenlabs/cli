pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TicketSearchMatchField {
    Title,
    Description,
    Comment,
    TurnComment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TicketSearchMatchField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Title => serializer.serialize_str("title"),
            Self::Description => serializer.serialize_str("description"),
            Self::Comment => serializer.serialize_str("comment"),
            Self::TurnComment => serializer.serialize_str("turn_comment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TicketSearchMatchField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "title" => Ok(Self::Title),
            "description" => Ok(Self::Description),
            "comment" => Ok(Self::Comment),
            "turn_comment" => Ok(Self::TurnComment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TicketSearchMatchField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Title => write!(f, "title"),
            Self::Description => write!(f, "description"),
            Self::Comment => write!(f, "comment"),
            Self::TurnComment => write!(f, "turn_comment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
