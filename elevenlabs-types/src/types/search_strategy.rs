pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchStrategy {
    Cat,
    Keyword,
    Semantic,
    Ls,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchStrategy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Cat => serializer.serialize_str("cat"),
            Self::Keyword => serializer.serialize_str("keyword"),
            Self::Semantic => serializer.serialize_str("semantic"),
            Self::Ls => serializer.serialize_str("ls"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchStrategy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "cat" => Ok(Self::Cat),
            "keyword" => Ok(Self::Keyword),
            "semantic" => Ok(Self::Semantic),
            "ls" => Ok(Self::Ls),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cat => write!(f, "cat"),
            Self::Keyword => write!(f, "keyword"),
            Self::Semantic => write!(f, "semantic"),
            Self::Ls => write!(f, "ls"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
