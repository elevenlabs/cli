pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The aspect ratio of the output image. With `auto`, the model picks an aspect ratio based on the inputs.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BytedanceSeedream5ProRequestAspectRatio {
    Auto,
    One1,
    Three4,
    Sixteen9,
    Four3,
    Nine16,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BytedanceSeedream5ProRequestAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::One1 => serializer.serialize_str("1:1"),
            Self::Three4 => serializer.serialize_str("3:4"),
            Self::Sixteen9 => serializer.serialize_str("16:9"),
            Self::Four3 => serializer.serialize_str("4:3"),
            Self::Nine16 => serializer.serialize_str("9:16"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BytedanceSeedream5ProRequestAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "1:1" => Ok(Self::One1),
            "3:4" => Ok(Self::Three4),
            "16:9" => Ok(Self::Sixteen9),
            "4:3" => Ok(Self::Four3),
            "9:16" => Ok(Self::Nine16),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BytedanceSeedream5ProRequestAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::One1 => write!(f, "1:1"),
            Self::Three4 => write!(f, "3:4"),
            Self::Sixteen9 => write!(f, "16:9"),
            Self::Four3 => write!(f, "4:3"),
            Self::Nine16 => write!(f, "9:16"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
