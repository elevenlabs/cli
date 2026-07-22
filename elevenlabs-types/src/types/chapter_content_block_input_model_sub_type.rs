pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChapterContentBlockInputModelSubType {
    P,
    H1,
    H2,
    H3,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChapterContentBlockInputModelSubType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::P => serializer.serialize_str("p"),
            Self::H1 => serializer.serialize_str("h1"),
            Self::H2 => serializer.serialize_str("h2"),
            Self::H3 => serializer.serialize_str("h3"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChapterContentBlockInputModelSubType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "p" => Ok(Self::P),
            "h1" => Ok(Self::H1),
            "h2" => Ok(Self::H2),
            "h3" => Ok(Self::H3),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChapterContentBlockInputModelSubType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::P => write!(f, "p"),
            Self::H1 => write!(f, "h1"),
            Self::H2 => write!(f, "h2"),
            Self::H3 => write!(f, "h3"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
