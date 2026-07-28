pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProjectResponseAspectRatio {
    Sixteen9,
    Nine16,
    Four5,
    One1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProjectResponseAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sixteen9 => serializer.serialize_str("16:9"),
            Self::Nine16 => serializer.serialize_str("9:16"),
            Self::Four5 => serializer.serialize_str("4:5"),
            Self::One1 => serializer.serialize_str("1:1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectResponseAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "16:9" => Ok(Self::Sixteen9),
            "9:16" => Ok(Self::Nine16),
            "4:5" => Ok(Self::Four5),
            "1:1" => Ok(Self::One1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProjectResponseAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sixteen9 => write!(f, "16:9"),
            Self::Nine16 => write!(f, "9:16"),
            Self::Four5 => write!(f, "4:5"),
            Self::One1 => write!(f, "1:1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
