pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The resolution of the output image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytedanceSeedream5LiteRequestResolution {
    TwoK,
    ThreeK,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BytedanceSeedream5LiteRequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TwoK => serializer.serialize_str("2K"),
            Self::ThreeK => serializer.serialize_str("3K"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BytedanceSeedream5LiteRequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "2K" => Ok(Self::TwoK),
            "3K" => Ok(Self::ThreeK),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BytedanceSeedream5LiteRequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TwoK => write!(f, "2K"),
            Self::ThreeK => write!(f, "3K"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
