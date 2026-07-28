pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The state of the chapter.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChapterWithContentResponseModelState {
    Default,
    Converting,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChapterWithContentResponseModelState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Default => serializer.serialize_str("default"),
            Self::Converting => serializer.serialize_str("converting"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChapterWithContentResponseModelState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "default" => Ok(Self::Default),
            "converting" => Ok(Self::Converting),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChapterWithContentResponseModelState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Converting => write!(f, "converting"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
