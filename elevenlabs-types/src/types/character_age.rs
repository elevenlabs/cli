pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharacterAge {
    Young,
    MiddleAged,
    Old,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CharacterAge {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Young => serializer.serialize_str("young"),
            Self::MiddleAged => serializer.serialize_str("middle-aged"),
            Self::Old => serializer.serialize_str("old"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CharacterAge {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "young" => Ok(Self::Young),
            "middle-aged" => Ok(Self::MiddleAged),
            "old" => Ok(Self::Old),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CharacterAge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Young => write!(f, "young"),
            Self::MiddleAged => write!(f, "middle-aged"),
            Self::Old => write!(f, "old"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
