pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptionStyleModelTextStyle {
    Normal,
    Italic,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaptionStyleModelTextStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Normal => serializer.serialize_str("normal"),
            Self::Italic => serializer.serialize_str("italic"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaptionStyleModelTextStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "normal" => Ok(Self::Normal),
            "italic" => Ok(Self::Italic),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaptionStyleModelTextStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Italic => write!(f, "italic"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
