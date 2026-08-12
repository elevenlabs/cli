pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptionStyleModelTextAlign {
    Start,
    Center,
    End,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaptionStyleModelTextAlign {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Start => serializer.serialize_str("start"),
            Self::Center => serializer.serialize_str("center"),
            Self::End => serializer.serialize_str("end"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaptionStyleModelTextAlign {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "start" => Ok(Self::Start),
            "center" => Ok(Self::Center),
            "end" => Ok(Self::End),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaptionStyleModelTextAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::Center => write!(f, "center"),
            Self::End => write!(f, "end"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
