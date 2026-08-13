pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The aspect ratio of the output image. With `auto`, the model picks an aspect ratio based on the inputs.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GptImage2RequestAspectRatio {
    Auto,
    One1,
    Four5,
    Five4,
    Three4,
    Four3,
    Two3,
    Three2,
    One2,
    Two1,
    Nine16,
    Sixteen9,
    TwentyOne9,
    One3,
    Three1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GptImage2RequestAspectRatio {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::One1 => serializer.serialize_str("1:1"),
            Self::Four5 => serializer.serialize_str("4:5"),
            Self::Five4 => serializer.serialize_str("5:4"),
            Self::Three4 => serializer.serialize_str("3:4"),
            Self::Four3 => serializer.serialize_str("4:3"),
            Self::Two3 => serializer.serialize_str("2:3"),
            Self::Three2 => serializer.serialize_str("3:2"),
            Self::One2 => serializer.serialize_str("1:2"),
            Self::Two1 => serializer.serialize_str("2:1"),
            Self::Nine16 => serializer.serialize_str("9:16"),
            Self::Sixteen9 => serializer.serialize_str("16:9"),
            Self::TwentyOne9 => serializer.serialize_str("21:9"),
            Self::One3 => serializer.serialize_str("1:3"),
            Self::Three1 => serializer.serialize_str("3:1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GptImage2RequestAspectRatio {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "1:1" => Ok(Self::One1),
            "4:5" => Ok(Self::Four5),
            "5:4" => Ok(Self::Five4),
            "3:4" => Ok(Self::Three4),
            "4:3" => Ok(Self::Four3),
            "2:3" => Ok(Self::Two3),
            "3:2" => Ok(Self::Three2),
            "1:2" => Ok(Self::One2),
            "2:1" => Ok(Self::Two1),
            "9:16" => Ok(Self::Nine16),
            "16:9" => Ok(Self::Sixteen9),
            "21:9" => Ok(Self::TwentyOne9),
            "1:3" => Ok(Self::One3),
            "3:1" => Ok(Self::Three1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GptImage2RequestAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::One1 => write!(f, "1:1"),
            Self::Four5 => write!(f, "4:5"),
            Self::Five4 => write!(f, "5:4"),
            Self::Three4 => write!(f, "3:4"),
            Self::Four3 => write!(f, "4:3"),
            Self::Two3 => write!(f, "2:3"),
            Self::Three2 => write!(f, "3:2"),
            Self::One2 => write!(f, "1:2"),
            Self::Two1 => write!(f, "2:1"),
            Self::Nine16 => write!(f, "9:16"),
            Self::Sixteen9 => write!(f, "16:9"),
            Self::TwentyOne9 => write!(f, "21:9"),
            Self::One3 => write!(f, "1:3"),
            Self::Three1 => write!(f, "3:1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
