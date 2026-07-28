pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Sort criteria. Must be one of: created_date, usage_character_count_1y, trending, cloned_by_count.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoicesGetSharedRequestSort {
    CreatedDate,
    UsageCharacterCount1Y,
    Trending,
    ClonedByCount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoicesGetSharedRequestSort {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedDate => serializer.serialize_str("created_date"),
            Self::UsageCharacterCount1Y => serializer.serialize_str("usage_character_count_1y"),
            Self::Trending => serializer.serialize_str("trending"),
            Self::ClonedByCount => serializer.serialize_str("cloned_by_count"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoicesGetSharedRequestSort {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_date" => Ok(Self::CreatedDate),
            "usage_character_count_1y" => Ok(Self::UsageCharacterCount1Y),
            "trending" => Ok(Self::Trending),
            "cloned_by_count" => Ok(Self::ClonedByCount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoicesGetSharedRequestSort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedDate => write!(f, "created_date"),
            Self::UsageCharacterCount1Y => write!(f, "usage_character_count_1y"),
            Self::Trending => write!(f, "trending"),
            Self::ClonedByCount => write!(f, "cloned_by_count"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
