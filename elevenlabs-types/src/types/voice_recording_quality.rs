pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VoiceRecordingQuality {
    Studio,
    Good,
    Ok,
    Poor,
    Bad,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VoiceRecordingQuality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Studio => serializer.serialize_str("studio"),
            Self::Good => serializer.serialize_str("good"),
            Self::Ok => serializer.serialize_str("ok"),
            Self::Poor => serializer.serialize_str("poor"),
            Self::Bad => serializer.serialize_str("bad"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VoiceRecordingQuality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "studio" => Ok(Self::Studio),
            "good" => Ok(Self::Good),
            "ok" => Ok(Self::Ok),
            "poor" => Ok(Self::Poor),
            "bad" => Ok(Self::Bad),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VoiceRecordingQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Studio => write!(f, "studio"),
            Self::Good => write!(f, "good"),
            Self::Ok => write!(f, "ok"),
            Self::Poor => write!(f, "poor"),
            Self::Bad => write!(f, "bad"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
