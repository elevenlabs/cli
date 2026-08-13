pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The background of the output image. With `auto`, the model picks the background that suits the image.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GptImage1RequestBackground {
    Transparent,
    Opaque,
    Auto,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GptImage1RequestBackground {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Transparent => serializer.serialize_str("transparent"),
            Self::Opaque => serializer.serialize_str("opaque"),
            Self::Auto => serializer.serialize_str("auto"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GptImage1RequestBackground {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transparent" => Ok(Self::Transparent),
            "opaque" => Ok(Self::Opaque),
            "auto" => Ok(Self::Auto),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GptImage1RequestBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transparent => write!(f, "transparent"),
            Self::Opaque => write!(f, "opaque"),
            Self::Auto => write!(f, "auto"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
