pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterRole {
    Narrator,
    Main,
    Supporting,
    Minor,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CharacterRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Narrator => serializer.serialize_str("narrator"),
            Self::Main => serializer.serialize_str("main"),
            Self::Supporting => serializer.serialize_str("supporting"),
            Self::Minor => serializer.serialize_str("minor"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CharacterRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "narrator" => Ok(Self::Narrator),
            "main" => Ok(Self::Main),
            "supporting" => Ok(Self::Supporting),
            "minor" => Ok(Self::Minor),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CharacterRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Narrator => write!(f, "narrator"),
            Self::Main => write!(f, "main"),
            Self::Supporting => write!(f, "supporting"),
            Self::Minor => write!(f, "minor"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
