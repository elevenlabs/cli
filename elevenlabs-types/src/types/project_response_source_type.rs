pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectResponseSourceType {
    Blank,
    Book,
    Article,
    Genfm,
    Video,
    Screenplay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectResponseSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blank => serializer.serialize_str("blank"),
            Self::Book => serializer.serialize_str("book"),
            Self::Article => serializer.serialize_str("article"),
            Self::Genfm => serializer.serialize_str("genfm"),
            Self::Video => serializer.serialize_str("video"),
            Self::Screenplay => serializer.serialize_str("screenplay"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectResponseSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "blank" => Ok(Self::Blank),
            "book" => Ok(Self::Book),
            "article" => Ok(Self::Article),
            "genfm" => Ok(Self::Genfm),
            "video" => Ok(Self::Video),
            "screenplay" => Ok(Self::Screenplay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectResponseSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => write!(f, "blank"),
            Self::Book => write!(f, "book"),
            Self::Article => write!(f, "article"),
            Self::Genfm => write!(f, "genfm"),
            Self::Video => write!(f, "video"),
            Self::Screenplay => write!(f, "screenplay"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
