pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptionStyleCharacterAnimationModelEnterType {
    None,
    Fade,
    Typewriter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaptionStyleCharacterAnimationModelEnterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Fade => serializer.serialize_str("fade"),
            Self::Typewriter => serializer.serialize_str("typewriter"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaptionStyleCharacterAnimationModelEnterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "fade" => Ok(Self::Fade),
            "typewriter" => Ok(Self::Typewriter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaptionStyleCharacterAnimationModelEnterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Fade => write!(f, "fade"),
            Self::Typewriter => write!(f, "typewriter"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
