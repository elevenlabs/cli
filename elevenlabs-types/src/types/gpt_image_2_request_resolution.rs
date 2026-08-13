pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The resolution of the output image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GptImage2RequestResolution {
    OneK,
    TwoK,
    FourK,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GptImage2RequestResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneK => serializer.serialize_str("1K"),
            Self::TwoK => serializer.serialize_str("2K"),
            Self::FourK => serializer.serialize_str("4K"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GptImage2RequestResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1K" => Ok(Self::OneK),
            "2K" => Ok(Self::TwoK),
            "4K" => Ok(Self::FourK),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GptImage2RequestResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneK => write!(f, "1K"),
            Self::TwoK => write!(f, "2K"),
            Self::FourK => write!(f, "4K"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
