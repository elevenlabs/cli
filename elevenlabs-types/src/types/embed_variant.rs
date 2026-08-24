pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmbedVariant {
    Tiny,
    Compact,
    Full,
    Expandable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for EmbedVariant {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Tiny => serializer.serialize_str("tiny"),
            Self::Compact => serializer.serialize_str("compact"),
            Self::Full => serializer.serialize_str("full"),
            Self::Expandable => serializer.serialize_str("expandable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for EmbedVariant {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "tiny" => Ok(Self::Tiny),
            "compact" => Ok(Self::Compact),
            "full" => Ok(Self::Full),
            "expandable" => Ok(Self::Expandable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for EmbedVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tiny => write!(f, "tiny"),
            Self::Compact => write!(f, "compact"),
            Self::Full => write!(f, "full"),
            Self::Expandable => write!(f, "expandable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
