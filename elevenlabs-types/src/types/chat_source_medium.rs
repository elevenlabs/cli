pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChatSourceMedium {
    Audio,
    Dtmf,
    Text,
    Image,
    File,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ChatSourceMedium {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Audio => serializer.serialize_str("audio"),
            Self::Dtmf => serializer.serialize_str("dtmf"),
            Self::Text => serializer.serialize_str("text"),
            Self::Image => serializer.serialize_str("image"),
            Self::File => serializer.serialize_str("file"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ChatSourceMedium {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "audio" => Ok(Self::Audio),
            "dtmf" => Ok(Self::Dtmf),
            "text" => Ok(Self::Text),
            "image" => Ok(Self::Image),
            "file" => Ok(Self::File),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ChatSourceMedium {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audio => write!(f, "audio"),
            Self::Dtmf => write!(f, "dtmf"),
            Self::Text => write!(f, "text"),
            Self::Image => write!(f, "image"),
            Self::File => write!(f, "file"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
